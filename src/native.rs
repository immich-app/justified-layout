extern crate core;
extern crate alloc;

use core::simd::prelude::*;
use core::simd::simd_swizzle;
use alloc::vec;
use alloc::vec::Vec;

use crate::LayoutOptions;

type F32x = Simd<f32, 4>;

#[inline(always)]
pub fn get_justified_layout_simd(aspect_ratios: &[f32], options: LayoutOptions) -> Vec<f32> {
    if aspect_ratios.is_empty() {
        return vec![];
    }

    let mut positions = vec![0.0; aspect_ratios.len() * 4 + 4];
    let min_row_height = (options.row_height * (1.0 - options.tolerance)).max(0.0);
    let max_row_height = options.row_height * (1.0 + options.tolerance);
    let max_row_aspect_ratio = options.row_width / min_row_height;
    let target_row_aspect_ratio = options.row_width / options.row_height;
    let spacing_aspect_ratio = options.spacing / options.row_height;

    let mut cur_aspect_ratio = 0.0;
    let mut row_aspect_ratio = 0.0;
    let mut max_actual_row_width: f32 = 0.0;
    let mut row_start_idx: usize = 0;
    let mut top = 0.0;

    let mut row_diff = target_row_aspect_ratio;
    for (i, &aspect_ratio) in aspect_ratios.iter().enumerate() {
        cur_aspect_ratio += aspect_ratio;
        let cur_diff = (cur_aspect_ratio - target_row_aspect_ratio).abs();

        if (cur_aspect_ratio > max_row_aspect_ratio || cur_diff > row_diff) && i > 0 {
            let aspect_ratio_row = &aspect_ratios[row_start_idx..i];
            let row_out = &mut positions[row_start_idx * 4 + 4..i * 4 + 4];

            let total_aspect_ratio =
                row_aspect_ratio - (spacing_aspect_ratio * aspect_ratio_row.len() as f32);
            let spacing_pixels = options.spacing * (aspect_ratio_row.len() as u16 - 1) as f32;
            let scaled_row_height =
                ((options.row_width - spacing_pixels) / total_aspect_ratio).min(max_row_height);

            let actual_row_width = finalize_row_simd(
                row_out,
                aspect_ratio_row,
                scaled_row_height,
                top,
                options.spacing,
            );

            top += scaled_row_height + options.spacing;
            max_actual_row_width = max_actual_row_width.max(actual_row_width + spacing_pixels);
            row_start_idx = i;
            cur_aspect_ratio = aspect_ratio;
            row_diff = (cur_aspect_ratio - target_row_aspect_ratio).abs();
        } else {
            row_diff = cur_diff;
        }
        cur_aspect_ratio += spacing_aspect_ratio;
        row_aspect_ratio = cur_aspect_ratio;
    }

    // Last row (same special logic as lib.rs)
    let aspect_ratio_row = &aspect_ratios[row_start_idx..];
    let total_aspect_ratio =
        row_aspect_ratio - (spacing_aspect_ratio * aspect_ratio_row.len() as f32);
    let spacing_pixels = options.spacing * (aspect_ratio_row.len() as u16 - 1) as f32;

    let base_row_height = (options.row_width - spacing_pixels) / total_aspect_ratio;
    let scaled_row_height = if base_row_height > max_row_height {
        if row_start_idx > 0 {
            unsafe { *positions.get_unchecked(row_start_idx * 4 + 3) }
        } else {
            options.row_height
        }
    } else {
        base_row_height
    };

    let row_out = &mut positions[row_start_idx * 4 + 4..];
    let actual_row_width = finalize_row_simd(
        row_out,
        aspect_ratio_row,
        scaled_row_height,
        top,
        options.spacing,
    );

    unsafe {
        *positions.get_unchecked_mut(0) = (actual_row_width + spacing_pixels).max(max_actual_row_width);
        *positions.get_unchecked_mut(1) = top + scaled_row_height;
    }
    positions
}

/// SIMD prefix sum: [a, b, c, d] → [a, a+b, a+b+c, a+b+c+d]
#[inline(always)]
fn prefix_sum(v: F32x) -> F32x {
    let s1 = v + simd_swizzle!(v, F32x::splat(0.0), [4, 0, 1, 2]);
    s1 + simd_swizzle!(s1, F32x::splat(0.0), [4, 4, 0, 1])
}

/// Finalize a row: compute widths, prefix-sum lefts, transpose, and store — all in SIMD.
/// Returns the sum of item widths (excluding spacing).
#[inline(always)]
fn finalize_row_simd(
    row_out: &mut [f32],
    aspect_ratios: &[f32],
    scaled_row_height: f32,
    top: f32,
    spacing: f32,
) -> f32 {
    let height_vec = F32x::splat(scaled_row_height);
    let top_vec = F32x::splat(top);
    let spacing_offsets = F32x::from_array([0.0, spacing, 2.0 * spacing, 3.0 * spacing]);

    let mut running_left = 0.0f32;
    let mut total_width = 0.0f32;

    let (ar_chunks, ar_remainder) = aspect_ratios.as_chunks::<4>();
    let (out_chunks, remainder_out) = row_out.split_at_mut(ar_chunks.len() * 16);
    // SAFETY: out_chunks length is ar_chunks.len() * 16, exactly divisible by 16
    let out_chunks = unsafe { out_chunks.as_chunks_unchecked_mut::<16>() };

    for (ar_chunk, out_chunk) in ar_chunks.iter().zip(out_chunks.iter_mut()) {
        let ar = F32x::from_array(*ar_chunk);
        let widths = ar * height_vec;

        // Exclusive prefix sum + running offset + spacing
        let inclusive = prefix_sum(widths);
        let exclusive = simd_swizzle!(inclusive, F32x::splat(0.0), [4, 0, 1, 2]);
        let lefts = exclusive + F32x::splat(running_left) + spacing_offsets;

        // 4×4 transpose: SoA [tops, lefts, widths, heights] → AoS [t,l,w,h, t,l,w,h, ...]
        let tl_lo = simd_swizzle!(top_vec, lefts, [0, 4, 1, 5]);
        let tl_hi = simd_swizzle!(top_vec, lefts, [2, 6, 3, 7]);
        let wh_lo = simd_swizzle!(widths, height_vec, [0, 4, 1, 5]);
        let wh_hi = simd_swizzle!(widths, height_vec, [2, 6, 3, 7]);

        let out0 = simd_swizzle!(tl_lo, wh_lo, [0, 1, 4, 5]);
        let out1 = simd_swizzle!(tl_lo, wh_lo, [2, 3, 6, 7]);
        let out2 = simd_swizzle!(tl_hi, wh_hi, [0, 1, 4, 5]);
        let out3 = simd_swizzle!(tl_hi, wh_hi, [2, 3, 6, 7]);

        out0.copy_to_slice(&mut out_chunk[0..4]);
        out1.copy_to_slice(&mut out_chunk[4..8]);
        out2.copy_to_slice(&mut out_chunk[8..12]);
        out3.copy_to_slice(&mut out_chunk[12..16]);

        let chunk_total = inclusive[3];
        running_left += chunk_total + 4.0 * spacing;
        total_width += chunk_total;
    }

    // Remainder: use iterator to avoid bounds checks
    // SAFETY: remainder_out length is ar_remainder.len() * 4, exactly divisible by 4
    let remainder_positions = unsafe { remainder_out.as_chunks_unchecked_mut::<4>() };
    for (&ar, pos) in ar_remainder.iter().zip(remainder_positions.iter_mut()) {
        let w = ar * scaled_row_height;
        let out = F32x::from_array([top, running_left, w, scaled_row_height]);
        out.copy_to_slice(pos);
        running_left += w + spacing;
        total_width += w;
    }

    total_width
}
