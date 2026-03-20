use alloc::vec::Vec;
use wasm_bindgen::prelude::*;

use crate::{Layout, LayoutOptions};

#[global_allocator]
static A: rlsf::SmallGlobalTlsf = rlsf::SmallGlobalTlsf::new();

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    unreachable!()
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
#[wasm_bindgen]
pub fn get_justified_layout(
    aspect_ratios: &[f32],
    row_height: f32,
    row_width: f32,
    spacing: f32,
    tolerance: f32,
) -> Vec<f32> {
    let options = LayoutOptions::new(row_height, row_width, spacing, tolerance);
    Layout::new(aspect_ratios, &options).positions
}
