#![cfg_attr(target_arch = "wasm32", no_std)]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

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
pub struct LayoutBox {
    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,
}

struct RowState {
    cur_aspect_ratio: f32,
    row_aspect_ratio: f32,
    row_diff: f32,
    row_start_idx: usize,
    top: f32,
    max_actual_row_width: f32,
}

impl RowState {
    #[inline(always)]
    fn new(options: &LayoutOptions) -> Self {
        Self {
            cur_aspect_ratio: 0.0,
            row_aspect_ratio: 0.0,
            row_diff: options.target_row_aspect_ratio,
            row_start_idx: 0,
            top: 0.0,
            max_actual_row_width: 0.0,
        }
    }

    #[inline(always)]
    fn is_row_full(&self, options: &LayoutOptions) -> bool {
        self.cur_aspect_ratio > options.max_row_aspect_ratio
            || (self.cur_aspect_ratio - options.target_row_aspect_ratio).abs() > self.row_diff
    }

    #[inline(always)]
    fn advance(&mut self, aspect_ratio: f32) {
        self.cur_aspect_ratio += aspect_ratio;
    }

    #[inline(always)]
    fn commit(&mut self, options: &LayoutOptions) {
        self.row_diff = (self.cur_aspect_ratio - options.target_row_aspect_ratio).abs();
        self.cur_aspect_ratio += options.spacing_aspect_ratio;
        self.row_aspect_ratio = self.cur_aspect_ratio;
    }

    /// Compute row height, write positions, update state for the next row.
    /// Returns scaled_row_height (needed for last-row container height).
    #[inline(always)]
    fn finalize_row(
        &mut self,
        positions: &mut [f32],
        aspect_ratios: &[f32],
        options: &LayoutOptions,
        end_idx: usize,
        prev_row_height: Option<f32>,
    ) -> f32 {
        let row_ratios = &aspect_ratios[self.row_start_idx..end_idx];
        let row_positions = &mut positions[self.row_start_idx * 4 + 4..end_idx * 4 + 4];
        let count = row_ratios.len();

        let total_aspect_ratio =
            self.row_aspect_ratio - (options.spacing_aspect_ratio * count as f32);
        let spacing_pixels = options.spacing * f32::from(count as u16 - 1);
        let base_row_height = (options.row_width - spacing_pixels) / total_aspect_ratio;
        let scaled_row_height = match prev_row_height {
            Some(prev) if base_row_height > options.max_row_height => prev,
            _ => base_row_height.min(options.max_row_height),
        };

        let mut actual_row_width = spacing_pixels;
        let mut left = 0.0f32;
        // SAFETY: row_positions has row_ratios.len() * 4 f32s, matching LayoutBox's repr(C) layout
        let boxes: &mut [LayoutBox] = unsafe {
            core::slice::from_raw_parts_mut(
                row_positions.as_mut_ptr() as *mut LayoutBox,
                row_ratios.len(),
            )
        };
        for (&ratio, b) in row_ratios.iter().zip(boxes.iter_mut()) {
            let width = ratio * scaled_row_height;
            *b = LayoutBox {
                top: self.top,
                left,
                width,
                height: scaled_row_height,
            };
            left += width + options.spacing;
            actual_row_width += width;
        }

        self.top += scaled_row_height + options.spacing;
        self.max_actual_row_width = actual_row_width.max(self.max_actual_row_width);
        self.row_start_idx = end_idx;
        if end_idx < aspect_ratios.len() {
            self.cur_aspect_ratio = aspect_ratios[end_idx];
            self.row_diff = (self.cur_aspect_ratio - options.target_row_aspect_ratio).abs();
        }
        scaled_row_height
    }
}

pub struct Layout {
    pub(crate) positions: Vec<f32>,
}

impl Layout {
    pub fn new(aspect_ratios: &[f32], options: &LayoutOptions) -> Self {
        if aspect_ratios.len() == 0 {
            return Layout { positions: vec![] };
        }

        let mut positions = vec![0.0; aspect_ratios.len() * 4 + 4];
        let mut state = RowState::new(options);

        for (i, &aspect_ratio) in aspect_ratios.into_iter().enumerate() {
            state.advance(aspect_ratio);

            if state.is_row_full(options) && i > 0 {
                state.finalize_row(&mut positions, aspect_ratios, options, i, None);
            }

            state.commit(options);
        }

        // Last row: use the previous row's height as fallback if it can't fill
        let prev_row_height = if state.row_start_idx > 0 {
            // SAFETY: this is guaranteed to be within bounds
            unsafe { Some(*positions.get_unchecked(state.row_start_idx * 4 + 3)) }
        } else {
            Some(options.row_height)
        };
        let n = aspect_ratios.len();
        let top_before = state.top;
        let scaled_row_height =
            state.finalize_row(&mut positions, aspect_ratios, options, n, prev_row_height);

        unsafe {
            *positions.get_unchecked_mut(0) = state.max_actual_row_width;
            *positions.get_unchecked_mut(1) = top_before + scaled_row_height;
        }

        Layout { positions }
    }

    pub fn boxes(&self) -> &[LayoutBox] {
        if self.positions.is_empty() {
            return &[];
        }
        // SAFETY: positions[4..] is LayoutBox's worth of repr(C) f32s
        unsafe {
            core::slice::from_raw_parts(
                self.positions.as_ptr().add(4) as *const LayoutBox,
                self.len(),
            )
        }
    }

    pub fn width(&self) -> f32 {
        self.positions[0]
    }

    pub fn height(&self) -> f32 {
        self.positions[1]
    }

    pub fn len(&self) -> usize {
        (self.positions.len() - 4) / 4
    }
}
