let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


let cachedFloat32ArrayMemory0 = null;

function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

let WASM_VECTOR_LEN = 0;

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getFloat32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedInt32ArrayMemory0 = null;

function getInt32ArrayMemory0() {
    if (cachedInt32ArrayMemory0 === null || cachedInt32ArrayMemory0.byteLength === 0) {
        cachedInt32ArrayMemory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32ArrayMemory0;
}

function getArrayI32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}
/**
 * Given an input of aspect ratios representing boxes, returns a vector 4 times its length + 4.
 * The first element is the maximum width across all rows, the second is the total height required
 * to display all rows, the next two are padding, and the remaining elements are sequences of 4
 * elements for each box, representing the top, left, width and height positions.
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
 * @param {Float32Array} aspect_ratios
 * @param {number} row_height
 * @param {number} row_width
 * @param {number} spacing
 * @param {number} tolerance
 * @returns {Int32Array}
 */
export function get_justified_layout(aspect_ratios, row_height, row_width, spacing, tolerance) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArrayF32ToWasm0(aspect_ratios, wasm.__wbindgen_export_0);
        const len0 = WASM_VECTOR_LEN;
        wasm.get_justified_layout(retptr, ptr0, len0, row_height, row_width, spacing, tolerance);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v2 = getArrayI32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export_1(r0, r1 * 4, 4);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

