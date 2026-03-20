#![cfg_attr(target_arch = "wasm32", no_std)]
#![deny(clippy::undocumented_unsafe_blocks)]
extern crate alloc;

use alloc::{
    alloc::{alloc, Layout as AllocLayout},
    vec,
    vec::Vec,
};

#[cfg(target_arch = "wasm32")]
mod wasm;

#[derive(Clone, Copy)]
pub struct LayoutOptions {
    pub row_height: f32,
    pub row_width: f32,
    pub spacing: f32,
    pub tolerance: f32,
    max_row_height: f32,
    max_row_aspect_ratio: f32,
    target_row_aspect_ratio: f32,
    spacing_aspect_ratio: f32,
}

impl LayoutOptions {
    pub const fn new(row_height: f32, row_width: f32, spacing: f32, tolerance: f32) -> Self {
        let min_row_height = (row_height * (1.0 - tolerance)).max(0.0);
        Self {
            row_height,
            row_width,
            spacing,
            tolerance,
            max_row_height: row_height * (1.0 + tolerance),
            max_row_aspect_ratio: row_width / min_row_height,
            target_row_aspect_ratio: row_width / row_height,
            spacing_aspect_ratio: spacing / row_height,
        }
    }
}

#[repr(C)]
pub struct Box {
    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,
}

pub struct Layout {
    pub(crate) positions: Vec<f32>,
}

impl Layout {
    pub fn new(aspect_ratios: &[f32], options: &LayoutOptions) -> Self {
        if aspect_ratios.is_empty() {
            return Layout { positions: vec![0.0; 4] };
        }

        let len = aspect_ratios.len() * 4 + 4;
        let layout = AllocLayout::array::<f32>(len).unwrap();
        // SAFETY: allocate without zero-init; all positions are written before read
        let ptr = unsafe { alloc(layout) as *mut f32 };
        if ptr.is_null() {
            panic!("Could not allocate memory");
        }

        // SAFETY: allocated with the same length above
        let mut positions = unsafe { Vec::from_raw_parts(ptr, len, len) };
        let mut cur_ar = 0.0f32;
        let mut row_ar = 0.0f32;
        let mut row_diff = options.target_row_aspect_ratio;
        let mut row_start = 0usize;
        let mut top = 0.0f32;
        let mut max_width = 0.0f32;

        for (i, &ar) in aspect_ratios.iter().enumerate() {
            cur_ar += ar;

            let is_full = cur_ar > options.max_row_aspect_ratio
                || (cur_ar - options.target_row_aspect_ratio).abs() > row_diff;
            if is_full && i > 0 {
                let (base_h, sp) = row_height(row_ar, i - row_start, options);
                let h = base_h.min(options.max_row_height);
                let w = write_row(
                    &mut positions[row_start * 4 + 4..i * 4 + 4],
                    &aspect_ratios[row_start..i],
                    h,
                    top,
                    options.spacing,
                    sp,
                );

                top += h + options.spacing;
                max_width = w.max(max_width);
                row_start = i;
                cur_ar = ar;
            }

            row_diff = (cur_ar - options.target_row_aspect_ratio).abs();
            cur_ar += options.spacing_aspect_ratio;
            row_ar = cur_ar;
        }

        // Last row: use the previous row's height if it can't fill
        let (base_h, sp) = row_height(row_ar, aspect_ratios.len() - row_start, options);
        let prev_h = if row_start > 0 {
            // SAFETY: row_start * 4 + 3 is within bounds when row_start > 0
            unsafe { *positions.get_unchecked(row_start * 4 + 3) }
        } else {
            options.row_height
        };
        let h = if base_h > options.max_row_height {
            prev_h
        } else {
            base_h
        };
        let w = write_row(
            &mut positions[row_start * 4 + 4..],
            &aspect_ratios[row_start..],
            h,
            top,
            options.spacing,
            sp,
        );
        max_width = w.max(max_width);

        // SAFETY: the first 4 elements are guaranteed within bounds
        unsafe {
            *positions.get_unchecked_mut(0) = max_width;
            *positions.get_unchecked_mut(1) = top + h;
        }

        Layout { positions }
    }

    pub fn boxes(&self) -> &[Box] {
        if self.positions.is_empty() {
            return &[];
        }
        // SAFETY: positions[4..] is Box's worth of repr(C) f32s
        unsafe {
            core::slice::from_raw_parts(self.positions.as_ptr().add(4) as *const Box, self.len())
        }
    }

    pub fn width(&self) -> f32 {
        // SAFETY: the first 4 elements are guaranteed within bounds
        unsafe { *self.positions.get_unchecked(0) }
    }

    pub fn height(&self) -> f32 {
        // SAFETY: the first 4 elements are guaranteed within bounds
        unsafe { *self.positions.get_unchecked(1) }
    }

    pub fn len(&self) -> usize {
        (self.positions.len() - 4) / 4
    }

    pub fn is_empty(&self) -> bool {
        self.positions.len() > 4
    }
}

/// Compute the unclamped row height and spacing pixels for a completed row.
#[inline(always)]
fn row_height(row_aspect_ratio: f32, count: usize, options: &LayoutOptions) -> (f32, f32) {
    let total_aspect_ratio = row_aspect_ratio - (options.spacing_aspect_ratio * count as f32);
    let spacing_pixels = options.spacing * f32::from(count as u16 - 1);
    let base = (options.row_width - spacing_pixels) / total_aspect_ratio;
    (base, spacing_pixels)
}

/// Write positions for a row's items. Returns the actual row width (items + spacing).
#[inline(always)]
fn write_row(
    positions: &mut [f32],
    aspect_ratios: &[f32],
    height: f32,
    top: f32,
    spacing: f32,
    spacing_pixels: f32,
) -> f32 {
    let mut actual_row_width = spacing_pixels;
    let mut left = 0.0f32;
    // SAFETY: positions has aspect_ratios.len() * 4 f32s, matching Box's repr(C) layout
    let boxes: &mut [Box] = unsafe {
        core::slice::from_raw_parts_mut(positions.as_mut_ptr() as *mut Box, aspect_ratios.len())
    };
    for (&ratio, b) in aspect_ratios.iter().zip(boxes.iter_mut()) {
        let width = ratio * height;
        *b = Box {
            top,
            left,
            width,
            height,
        };
        left += width + spacing;
        actual_row_width += width;
    }
    actual_row_width
}
