extern crate alloc;

use core::simd::prelude::*;
use core::simd::simd_swizzle;
use alloc::vec;
use alloc::vec::Vec;

use crate::LayoutOptions;

type F32x = Simd<f32, 4>;

const BLOCK: usize = 4;

/// Compute local prefix sum within a 4-element block: [a,b,c,d] → [a, a+b, a+b+c, a+b+c+d]
/// This has NO cross-block dependency — can be computed independently for any block.
#[inline(always)]
fn local_prefix(v: F32x) -> F32x {
    let s1 = v + simd_swizzle!(v, F32x::splat(0.0), [4, 0, 1, 2]);
    s1 + simd_swizzle!(s1, F32x::splat(0.0), [4, 4, 0, 1])
}

/// Accumulate: add carry from previous blocks to a local prefix sum block.
/// Returns the updated carry (last element of the result).
#[inline(always)]
fn accumulate(local: F32x, carry: F32x) -> (F32x, F32x) {
    let result = local + carry;
    // Broadcast the last element as the new carry
    let new_carry = F32x::splat(result[3]);
    (result, new_carry)
}

#[inline(always)]
pub fn get_justified_layout_simd(aspect_ratios: &[f32], options: LayoutOptions) -> Vec<f32> {
    let n = aspect_ratios.len();
    if n == 0 {
        return vec![];
    }

    let mut positions = vec![0.0; n * 4 + 4];
    let min_row_height = options.row_height.algebraic_mul(1.0f32.algebraic_sub(options.tolerance)).max(0.0);
    let max_row_height = options.row_height.algebraic_mul(options.tolerance.algebraic_add(1.0));
    let max_row_aspect_ratio = options.row_width.algebraic_div(min_row_height);
    let target_row_aspect_ratio = options.row_width.algebraic_div(options.row_height);
    let spacing_aspect_ratio = options.spacing.algebraic_div(options.row_height);

    let max_ar_vec = F32x::splat(max_row_aspect_ratio);
    let neg_target_vec = F32x::splat(-target_row_aspect_ratio);

    // We compute prefix sums of (ar + spacing_ar) to get cumulative aspect ratios.
    // But the check happens BEFORE adding spacing for the current item.
    // So we prefix-sum (ar), check, then add spacing.
    // Actually the original loop does: cur += ar; check; cur += spacing; row_ar = cur.
    // The value checked is: row_start_sum + sum(ar[start..=i]) + (i - start) * spacing_ar
    // which is equivalent to prefix_sum(ar + spacing_ar) - spacing_ar (since spacing is added after check)

    // For simplicity, we'll store a scratch buffer with the local prefix sums
    // to avoid recomputing them in the accumulate phase.
    // Use a small stack buffer — we only need BLOCK elements at a time for interleaving.

    let mut cur_aspect_ratio = 0.0f32;
    let mut row_aspect_ratio = 0.0f32;
    let mut max_actual_row_width = 0.0f32;
    let mut row_start_idx: usize = 0;
    let mut top = 0.0f32;
    let mut row_diff = target_row_aspect_ratio;

    // Process blocks of 4 with the interleaved prefix-sum/accumulate pattern.
    // Phase A (local prefix) runs one block ahead of Phase B (accumulate + boundary check).
    //
    // Within Phase B, when no boundary is found in a block of 4, we advance
    // cur_aspect_ratio, row_aspect_ratio, and row_diff in one SIMD step.
    // When a boundary IS found, we fall back to scalar for that block.

    let full_blocks = n / BLOCK;

    // Precompute local prefix sums for each block (Phase A)
    // We store just the raw prefix of the aspect ratios within each block.
    // During accumulate (Phase B), we add the running carry.
    //
    // Using the interleaving pattern from the article: instead of two separate passes,
    // we interleave Phase A (compute local prefix for block i+1) with
    // Phase B (accumulate + process block i).

    if full_blocks > 0 {
        // Prolog: compute local prefix for block 0
        let ar0 = F32x::from_slice(&aspect_ratios[0..BLOCK]);
        let mut ahead_local = local_prefix(ar0);
        let mut carry = F32x::splat(0.0);

        for block_idx in 0..full_blocks {
            // Phase B for current block: accumulate + check boundaries
            let (global_prefix, new_carry) = accumulate(ahead_local, carry);

            // Phase A for NEXT block (runs ahead, independent of Phase B)
            // This gives the OoO engine independent work to schedule.
            if block_idx + 1 < full_blocks {
                let next_start = (block_idx + 1) * BLOCK;
                let next_ar = F32x::from_slice(&aspect_ratios[next_start..]);
                ahead_local = local_prefix(next_ar);
            }

            // Now process the 4 items in this block using the global prefix values.
            // global_prefix[k] = sum(ar[0..=block_start+k]) — the cumulative AR at each position.
            //
            // The original algorithm tracks cur_aspect_ratio = sum(ar + spacing) from row_start.
            // We need to convert: for item at absolute index j,
            //   cur_ar_check = global_prefix[j] - global_prefix[row_start-1] + (j - row_start) * spacing_ar
            //
            // But tracking this across row resets is complex with SIMD.
            // Instead: check if ANY item in this block would trigger a boundary.
            // If not (common case), advance 4 items at once.
            // If yes, fall back to scalar for this block.

            let block_start = block_idx * BLOCK;

            // Compute the "cur_aspect_ratio" values as the original loop would see them.
            // These are relative to the current row start, including spacing.
            // cur_ar[k] = (global_prefix[k] - global_prefix_at_row_start) + items_in_row[k] * spacing_ar
            let block_ar = F32x::from_slice(&aspect_ratios[block_start..]);
            let block_prefix = local_prefix(block_ar);
            let spacing_offsets = F32x::from_array([
                0.0,
                spacing_aspect_ratio,
                spacing_aspect_ratio.algebraic_mul(2.0),
                spacing_aspect_ratio.algebraic_mul(3.0),
            ]);
            let cur_values = block_prefix + F32x::splat(cur_aspect_ratio) + spacing_offsets;

            // Check 1: none exceed max_row_ar
            let exceeds_max = cur_values.simd_gt(max_ar_vec);

            // Check 2: diffs are monotonically decreasing (approaching target)
            let diffs = (cur_values + neg_target_vec).abs();
            let prev_diffs = simd_swizzle!(diffs, F32x::splat(row_diff), [4, 0, 1, 2]);
            let getting_worse = diffs.simd_gt(prev_diffs);

            let has_boundary = exceeds_max.any() || (getting_worse.any() && block_start > 0);

            if !has_boundary {
                // Fast path: no boundary in these 4 items — advance in bulk
                cur_aspect_ratio = cur_values[3].algebraic_add(spacing_aspect_ratio);
                row_aspect_ratio = cur_aspect_ratio;
                row_diff = diffs[3];
                carry = new_carry;
            } else {
                // Slow path: process items one at a time (boundary in this block)
                for j in 0..BLOCK {
                    let idx = block_start + j;
                    let aspect_ratio = aspect_ratios[idx];
                    cur_aspect_ratio = cur_aspect_ratio.algebraic_add(aspect_ratio);
                    let cur_diff = cur_aspect_ratio.algebraic_sub(target_row_aspect_ratio).abs();

                    if (cur_aspect_ratio > max_row_aspect_ratio || cur_diff > row_diff) && idx > 0 {
                        let count = idx - row_start_idx;
                        let total_aspect_ratio = row_aspect_ratio.algebraic_sub(
                            spacing_aspect_ratio.algebraic_mul(count as f32),
                        );
                        let spacing_pixels = options.spacing.algebraic_mul(f32::from(count as u16 - 1));
                        let scaled_row_height = (options.row_width.algebraic_sub(spacing_pixels))
                            .algebraic_div(total_aspect_ratio)
                            .min(max_row_height);

                        let row = &mut positions[row_start_idx * 4 + 4..idx * 4 + 4];
                        let aspect_ratio_row = &aspect_ratios[row_start_idx..idx];
                        let mut actual_row_width = spacing_pixels;
                        let mut left = 0.0f32;
                        let row_positions = unsafe { row.as_chunks_unchecked_mut::<4>() };
                        for (&ratio, pos) in aspect_ratio_row.iter().zip(row_positions) {
                            let width = ratio.algebraic_mul(scaled_row_height);
                            pos[0] = top;
                            pos[1] = left;
                            pos[2] = width;
                            pos[3] = scaled_row_height;
                            left = left.algebraic_add(width.algebraic_add(options.spacing));
                            actual_row_width = actual_row_width.algebraic_add(width);
                        }
                        top = top.algebraic_add(scaled_row_height.algebraic_add(options.spacing));
                        max_actual_row_width = actual_row_width.max(max_actual_row_width);
                        row_start_idx = idx;
                        cur_aspect_ratio = aspect_ratio;
                        row_diff = cur_aspect_ratio.algebraic_sub(target_row_aspect_ratio).abs();
                    } else {
                        row_diff = cur_diff;
                    }
                    cur_aspect_ratio = cur_aspect_ratio.algebraic_add(spacing_aspect_ratio);
                    row_aspect_ratio = cur_aspect_ratio;
                }
                // Recompute carry after scalar processing
                carry = F32x::splat(global_prefix[3]);
            }
        }
    }

    // Scalar remainder (last < 4 items)
    let rem_start = full_blocks * BLOCK;
    for idx in rem_start..n {
        let aspect_ratio = aspect_ratios[idx];
        cur_aspect_ratio = cur_aspect_ratio.algebraic_add(aspect_ratio);
        let cur_diff = cur_aspect_ratio.algebraic_sub(target_row_aspect_ratio).abs();

        if (cur_aspect_ratio > max_row_aspect_ratio || cur_diff > row_diff) && idx > 0 {
            let count = idx - row_start_idx;
            let total_aspect_ratio = row_aspect_ratio.algebraic_sub(
                spacing_aspect_ratio.algebraic_mul(count as f32),
            );
            let spacing_pixels = options.spacing.algebraic_mul(f32::from(count as u16 - 1));
            let scaled_row_height = (options.row_width.algebraic_sub(spacing_pixels))
                .algebraic_div(total_aspect_ratio)
                .min(max_row_height);

            let row = &mut positions[row_start_idx * 4 + 4..idx * 4 + 4];
            let aspect_ratio_row = &aspect_ratios[row_start_idx..idx];
            let mut actual_row_width = spacing_pixels;
            let mut left = 0.0f32;
            let row_positions = unsafe { row.as_chunks_unchecked_mut::<4>() };
            for (&ratio, pos) in aspect_ratio_row.iter().zip(row_positions) {
                let width = ratio.algebraic_mul(scaled_row_height);
                pos[0] = top;
                pos[1] = left;
                pos[2] = width;
                pos[3] = scaled_row_height;
                left = left.algebraic_add(width.algebraic_add(options.spacing));
                actual_row_width = actual_row_width.algebraic_add(width);
            }
            top = top.algebraic_add(scaled_row_height.algebraic_add(options.spacing));
            max_actual_row_width = actual_row_width.max(max_actual_row_width);
            row_start_idx = idx;
            cur_aspect_ratio = aspect_ratio;
            row_diff = cur_aspect_ratio.algebraic_sub(target_row_aspect_ratio).abs();
        } else {
            row_diff = cur_diff;
        }
        cur_aspect_ratio = cur_aspect_ratio.algebraic_add(spacing_aspect_ratio);
        row_aspect_ratio = cur_aspect_ratio;
    }

    // Last row
    let aspect_ratio_row = &aspect_ratios[row_start_idx..];
    let total_aspect_ratio =
        row_aspect_ratio.algebraic_sub(spacing_aspect_ratio.algebraic_mul(aspect_ratio_row.len() as f32));
    let spacing_pixels = options.spacing.algebraic_mul((aspect_ratio_row.len() as u16 - 1) as f32);

    let base_row_height = (options.row_width.algebraic_sub(spacing_pixels)).algebraic_div(total_aspect_ratio);
    let scaled_row_height = if base_row_height > max_row_height {
        if row_start_idx > 0 {
            unsafe { *positions.get_unchecked(row_start_idx * 4 + 3) }
        } else {
            options.row_height
        }
    } else {
        base_row_height
    };

    let row = &mut positions[row_start_idx * 4 + 4..];
    let mut actual_row_width = spacing_pixels;
    let mut left = 0.0f32;
    let row_positions = unsafe { row.as_chunks_unchecked_mut::<4>() };
    for (&ratio, pos) in aspect_ratio_row.iter().zip(row_positions) {
        let width = ratio.algebraic_mul(scaled_row_height);
        pos[0] = top;
        pos[1] = left;
        pos[2] = width;
        pos[3] = scaled_row_height;
        left = left.algebraic_add(width.algebraic_add(options.spacing));
        actual_row_width = actual_row_width.algebraic_add(width);
    }
    unsafe {
        *positions.get_unchecked_mut(0) = actual_row_width.max(max_actual_row_width);
        *positions.get_unchecked_mut(1) = top.algebraic_add(scaled_row_height);
    }
    positions
}