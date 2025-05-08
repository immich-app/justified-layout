use wasm_bindgen::prelude::*;

pub struct LayoutOptions {
    pub row_height: f32,
    pub row_width: f32,
    pub spacing: f32,
    pub tolerance: f32,
}

/// Given an input of aspect ratios representing boxes, returns a vector 4 times its length + 4.
/// The first element is the maximum width across all rows, the second is the total height required
/// to display all rows, the next two are padding, and the remaining elements are sequences of 4
/// elements for each box, representing the top, left, width and height positions.
/// `row_height` is a positive float that is the target height of the row.
///     It is not strictly followed; the actual height may be off by one due to truncation, and may be
///     substantially different if only one box can fit on a row and this box cannot fit with the
///     target height. The height cannot exceed this target unless `tolerance` is greater than zero.
/// `row_width` is a positive float that is the target width of the row.
///     It can be exceeded by a rounding error or shorter if the boxes cannot fill the row
///     width given the `tolerance`.
/// `spacing` is a non-negative float that controls the spacing between boxes, including between rows.
///     Notably, there is no offset applied in directions where there is no box.
///     The first box will have its top and left positions both at 0, not at `spacing`, and so on.
/// `tolerance` is a non-negative float that gives more freedom to fill the row width.
///     When there is free space in the row and the next box cannot fit in this row, it can scale
///     the boxes to a larger height to fill this space while respecting aspect ratios. Additionally,
///     the height can be shorter if shrinking the row height would allow more boxes to fit
///     in the row without causing the height to be more off from the target height. A value of 0.15
///     signifies that the actual row height may be up to 15% shorter or taller than the target height.
///
/// Note: The response being Vec<i32> rather than a struct or list of structs is important, as the
///       JS-WASM interop is *massively* slower when moving structs to JS instead of an array and
///       importing integers is faster than floats.
#[wasm_bindgen]
pub fn get_justified_layout(
    aspect_ratios: &[f32],
    row_height: f32,
    row_width: f32,
    spacing: f32,
    tolerance: f32,
) -> Vec<f32> {
    let options = LayoutOptions {
        row_height,
        row_width,
        spacing,
        tolerance,
    };

    _get_justified_layout(aspect_ratios, options)
}

#[inline(always)]
pub fn _get_justified_layout(aspect_ratios: &[f32], options: LayoutOptions) -> Vec<f32> {
    let mut positions = vec![0.0; aspect_ratios.len() * 4 + 4]; // 2 for container width and height, 2 for alignment
    let min_row_height = options.row_height * (1.0 - options.tolerance);
    let max_row_height = options.row_height * (1.0 + options.tolerance);
    let mut cur_aspect_ratio = 0.0;
    let mut row_aspect_ratio = 0.0;
    let mut max_actual_row_width = 0.0;
    let mut row_start_idx: usize = 0;
    let mut top = 0.0;
    let max_row_aspect_ratio = options.row_width / min_row_height;
    let target_row_aspect_ratio = options.row_width / options.row_height;
    let spacing_aspect_ratio = options.spacing / options.row_height;

    let mut row_diff = target_row_aspect_ratio;
    for i in 0..aspect_ratios.len() {
        let aspect_ratio = aspect_ratios[i];
        cur_aspect_ratio += aspect_ratio;
        let cur_diff = (cur_aspect_ratio - target_row_aspect_ratio).abs();

        // there are no more boxes that can fit in this row
        if (cur_aspect_ratio > max_row_aspect_ratio || cur_diff > row_diff) && i > 0 {
            let row = &mut positions[row_start_idx * 4 + 4..i * 4 + 4];
            let aspect_ratio_row = &aspect_ratios[row_start_idx..i];

            // treat the row's boxes as a single entity and scale them to fit the row width
            let total_aspect_ratio =
                row_aspect_ratio - (spacing_aspect_ratio * aspect_ratio_row.len() as f32);
            let spacing_pixels = options.spacing * f32::from(aspect_ratio_row.len() as u16 - 1);
            let scaled_row_height =
                ((options.row_width - spacing_pixels) / total_aspect_ratio).min(max_row_height);

            let mut actual_row_width = spacing_pixels;
            let mut left = 0.0;
            // SAFETY: this slice's length is guaranteed to be a multiple of 4
            let row_positions = unsafe { row.as_chunks_unchecked_mut::<4>() };
            for i in 0..aspect_ratio_row.len() {
                let pos = &mut row_positions[i];
                let width = aspect_ratio_row[i] * scaled_row_height;
                pos[0] = top;
                pos[1] = left;
                pos[2] = width;
                pos[3] = scaled_row_height;
                left += width + options.spacing;
                actual_row_width += width;
            }
            top += scaled_row_height + options.spacing;
            max_actual_row_width = actual_row_width.max(max_actual_row_width);
            row_start_idx = i;
            cur_aspect_ratio = aspect_ratio;
            row_diff = (cur_aspect_ratio - target_row_aspect_ratio).abs();
        } else {
            row_diff = cur_diff;
        }
        cur_aspect_ratio += spacing_aspect_ratio;
        row_aspect_ratio = cur_aspect_ratio;
    }

    // this is the same as in the for loop and processes the last row
    // inlined because it ends up producing much better assembly
    let row = &mut positions[row_start_idx * 4 + 4..];
    let aspect_ratio_row = &aspect_ratios[row_start_idx..];
    let total_aspect_ratio =
        row_aspect_ratio - (spacing_aspect_ratio * aspect_ratio_row.len() as f32);
    let spacing_pixels = options.spacing * (aspect_ratio_row.len() as u16 - 1) as f32;
    let scaled_row_height =
        ((options.row_width - spacing_pixels) / total_aspect_ratio).min(max_row_height);

    let mut actual_row_width = spacing_pixels;
    let mut left = 0.0;
    // SAFETY: this slice's length is guaranteed to be a multiple of 4
    let row_positions = unsafe { row.as_chunks_unchecked_mut::<4>() };
    for i in 0..aspect_ratio_row.len() {
        let pos = &mut row_positions[i];
        let width = aspect_ratio_row[i] * scaled_row_height;
        pos[0] = top;
        pos[1] = left;
        pos[2] = width;
        pos[3] = scaled_row_height;
        left += width + options.spacing;
        actual_row_width += width;
    }
    // SAFETY: these indices are guaranteed to be within the vector's bounds
    unsafe {
        *positions.get_unchecked_mut(0) = actual_row_width.max(max_actual_row_width);
        *positions.get_unchecked_mut(1) = top + scaled_row_height;
    }
    positions
}
