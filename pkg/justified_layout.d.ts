/* tslint:disable */
/* eslint-disable */
/**
 * Given an input of aspect ratios representing boxes, returns a vector 4 times its length + 1.
 * The first element is the maximum width across all rows, while the remaining elements are
 * sequences of 4 elements for each box, representing the top, left, width and height positions.
 * `row_height` is a positive float that is the target height of the row.
 *     It is not strictly followed; the actual height may be off by one due to truncation, and may be
 *     substantially different if only one box can fit on a row and this box cannot fit with the
 *     target height. The height cannot exceed this target unless `tolerance` is greater than zero.
 * `row_width` is a positive float that is the target width of the row.
 *     It will not be exceeded, but a row may have a shorter width if the boxes cannot fill the row
 *     width given the `tolerance`. Additionally, as the positions are in floats,
 *     rounding them to integers may yield a very slightly different width.
 * `spacing` is a non-negative float that controls the spacing between boxes, including between rows.
 *     Notably, there is no offset applied in directions where there is no box.
 *     The first box will have its top and left positions both at 0, not at `spacing`, and so on.
 * `tolerance` is a non-negative float that gives more freedom to fill the row width.
 *     When there is free space in the row and the next box cannot fit in this row, it can scale
 *     the boxes to a larger height to fill this space while respecting aspect ratios. A value of
 *     0.15 signifies that the actual row height may be up to 15% greater than the target height.
 *
 * Note: The response being Vec<i32> rather than a struct or list of structs is important, as the
 *       JS-WASM interop is *massively* slower when moving structs to JS instead of an array and
 *       importing integers is faster than floats.
 */
export function get_justified_layout(aspect_ratios: Float32Array, row_height: number, row_width: number, spacing: number, tolerance: number): Int32Array;
