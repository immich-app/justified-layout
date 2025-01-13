use justified_layout::get_justified_layout;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn fits_perfectly_on_one_row() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let row_width = 900.0;
    let row_height = 300.0;
    let spacing = 0.0;
    let height_tolerance = 0.0;

    let layout = get_justified_layout(
        input.as_slice(),
        row_height,
        row_width,
        spacing,
        height_tolerance,
    );
    assert_eq!(layout.len(), 16);
    let max_row_width = layout[0];
    assert_eq!(max_row_width, 900);

    let max_row_height = layout[1];
    assert_eq!(max_row_height, 300);

    let [top1, left1, width1, height1] = layout[4..8] else {
        unreachable!()
    };
    assert_eq!(top1, 0);
    assert_eq!(left1, 0);
    assert_eq!(width1, 300);
    assert_eq!(height1, 300);

    let [top2, left2, width2, height2] = layout[8..12] else {
        unreachable!()
    };
    assert_eq!(top2, 0);
    assert_eq!(left2, width1);
    assert_eq!(width2, 300);
    assert_eq!(height2, 300);

    let [top3, left3, width3, height3] = layout[12..16] else {
        unreachable!()
    };
    assert_eq!(top3, 0);
    assert_eq!(left3, width1 + width2);
    assert_eq!(width3, 300);
    assert_eq!(height3, 300);
}

#[wasm_bindgen_test]
fn applies_spacing() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let row_width = 904.0;
    let row_height = 300.0;
    let spacing = 2.0;
    let height_tolerance = 0.0;

    let layout = get_justified_layout(
        input.as_slice(),
        row_height,
        row_width,
        spacing,
        height_tolerance,
    );
    assert_eq!(layout.len(), 16);
    let max_row_width = layout[0];
    assert_eq!(max_row_width, 904);

    let [top1, left1, width1, height1] = layout[4..8] else {
        unreachable!()
    };
    assert_eq!(top1, 0);
    assert_eq!(left1, 0);
    assert_eq!(width1, 300);
    assert_eq!(height1, 300);

    let [top2, left2, width2, height2] = layout[8..12] else {
        unreachable!()
    };
    assert_eq!(top2, 0);
    assert_eq!(left2, width1 + spacing as i32);
    assert_eq!(width2, 300);
    assert_eq!(height2, 300);

    let [top3, left3, width3, height3] = layout[12..16] else {
        unreachable!()
    };
    assert_eq!(top3, 0);
    assert_eq!(left3, width1 + spacing as i32 + width2 + spacing as i32);
    assert_eq!(width3, 300);
    assert_eq!(height3, 300);
}

#[wasm_bindgen_test]
fn expands_row_based_on_height_tolerance() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let row_width = 1000.0;
    let row_height = 300.0;
    let spacing = 2.0;
    let height_tolerance = 0.1;

    let layout = get_justified_layout(
        input.as_slice(),
        row_height,
        row_width,
        spacing,
        height_tolerance,
    );
    assert_eq!(layout.len(), 16);
    let max_row_width = layout[0];
    assert_eq!(max_row_width, 994);

    let [top1, left1, width1, height1] = layout[4..8] else {
        unreachable!()
    };
    assert_eq!(top1, 0);
    assert_eq!(left1, 0);
    assert_eq!(width1, 330);
    assert_eq!(height1, 330);

    let [top2, left2, width2, height2] = layout[8..12] else {
        unreachable!()
    };
    assert_eq!(top2, 0);
    assert_eq!(left2, width1 + spacing as i32);
    assert_eq!(width2, 330);
    assert_eq!(height2, 330);

    let [top3, left3, width3, height3] = layout[12..16] else {
        unreachable!()
    };
    assert_eq!(top3, 0);
    assert_eq!(left3, width1 + spacing as i32 + width2 + spacing as i32);
    assert_eq!(width3, 330);
    assert_eq!(height3, 330);
}

#[wasm_bindgen_test]
fn adds_second_row_due_to_spacing() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let row_width = 900.0;
    let row_height = 300.0;
    let spacing = 2.0;
    let height_tolerance = 0.0;

    let layout = get_justified_layout(
        input.as_slice(),
        row_height,
        row_width,
        spacing,
        height_tolerance,
    );
    assert_eq!(layout.len(), 16);
    let max_row_width = layout[0];
    assert_eq!(max_row_width, 602);

    let [top1, left1, width1, height1] = layout[4..8] else {
        unreachable!()
    };
    assert_eq!(top1, 0);
    assert_eq!(left1, 0);
    assert_eq!(width1, 300);
    assert_eq!(height1, 300);
    //
    let [top2, left2, width2, height2] = layout[8..12] else {
        unreachable!()
    };
    assert_eq!(top2, 0);
    assert_eq!(left2, width1 + spacing as i32);
    assert_eq!(width2, 300);
    assert_eq!(height2, 300);

    let [top3, left3, width3, height3] = layout[12..16] else {
        unreachable!()
    };
    assert_eq!(top3, (row_height + spacing) as i32);
    assert_eq!(left3, 0);
    assert_eq!(width3, 300);
    assert_eq!(height3, 300);
}

#[wasm_bindgen_test]
fn positions_boxes_with_different_aspect_ratios() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let row_width = 900.0;
    let row_height = 300.0;
    let spacing = 2.0;
    let height_tolerance = 0.0;

    let layout = get_justified_layout(
        input.as_slice(),
        row_height,
        row_width,
        spacing,
        height_tolerance,
    );
    assert_eq!(layout.len(), 16);
    let max_row_width = layout[0];
    assert_eq!(max_row_width, 771);

    let [top1, left1, width1, height1] = layout[4..8] else {
        unreachable!()
    };
    assert_eq!(top1, 0);
    assert_eq!(left1, 0);
    assert_eq!(width1, (300.0 * (16.0 / 9.0)) as i32);
    assert_eq!(height1, 300);
    //
    let [top2, left2, width2, height2] = layout[8..12] else {
        unreachable!()
    };
    assert_eq!(top2, height1 + spacing as i32);
    assert_eq!(left2, 0);
    assert_eq!(width2, 300 * 2);
    assert_eq!(height2, 300);

    let [top3, left3, width3, height3] = layout[12..16] else {
        unreachable!()
    };
    assert_eq!(top3, height1 + spacing as i32);
    assert_eq!(left3, width2 + spacing as i32);
    assert_eq!(width3, (300.0 * (9.0 / 16.0)) as i32);
    assert_eq!(height3, 300);
}

#[wasm_bindgen_test]
fn scales_boxes_with_different_aspect_ratios_when_using_height_tolerance() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let row_width = 900.0;
    let row_height = 300.0;
    let spacing = 2.0;
    let height_tolerance = 0.2;

    let layout = get_justified_layout(
        input.as_slice(),
        row_height,
        row_width,
        spacing,
        height_tolerance,
    );
    assert_eq!(layout.len(), 16);
    let max_row_width = layout[0];
    assert_eq!(max_row_width, 900);

    let [top1, left1, width1, height1] = layout[4..8] else {
        unreachable!()
    };
    assert_eq!(top1, 0);
    assert_eq!(left1, 0);
    assert_eq!(width1, 640);
    assert_eq!(height1, 360);
    //
    let [top2, left2, width2, height2] = layout[8..12] else {
        unreachable!()
    };
    assert_eq!(top2, height1 + spacing as i32);
    assert_eq!(left2, 0);
    assert_eq!(width2, 700);
    assert_eq!(height2, 350);

    let [top3, left3, width3, height3] = layout[12..16] else {
        unreachable!()
    };
    assert_eq!(top3, height1 + spacing as i32);
    assert_eq!(left3, width2 + spacing as i32);
    assert_eq!(width3, 197);
    assert_eq!(height3, 350);
}

#[wasm_bindgen_test]
fn one_square_box_on_each_row() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let row_width = 599.0;
    let row_height = 300.0;
    let spacing = 2.0;
    let height_tolerance = 0.0;

    let layout = get_justified_layout(
        input.as_slice(),
        row_height,
        row_width,
        spacing,
        height_tolerance,
    );
    assert_eq!(layout.len(), 16);
    let max_row_width = layout[0];
    assert_eq!(max_row_width, 300);

    let [top1, left1, width1, height1] = layout[4..8] else {
        unreachable!()
    };
    assert_eq!(top1, 0);
    assert_eq!(left1, 0);
    assert_eq!(width1, 300);
    assert_eq!(height1, 300);

    let [top2, left2, width2, height2] = layout[8..12] else {
        unreachable!()
    };
    assert_eq!(top2, height1 + spacing as i32);
    assert_eq!(left2, 0);
    assert_eq!(width2, 300);
    assert_eq!(height2, 300);

    let [top3, left3, width3, height3] = layout[12..16] else {
        unreachable!()
    };
    assert_eq!(top3, height1 + spacing as i32 + height2 + spacing as i32);
    assert_eq!(left3, 0);
    assert_eq!(width3, 300);
    assert_eq!(height3, 300);
}

#[wasm_bindgen_test]
fn different_shaped_boxes_on_each_row() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let row_width = 600.0;
    let row_height = 300.0;
    let spacing = 2.0;
    let height_tolerance = 0.0;

    let layout = get_justified_layout(
        input.as_slice(),
        row_height,
        row_width,
        spacing,
        height_tolerance,
    );
    assert_eq!(layout.len(), 16);
    let max_row_width = layout[0];
    assert_eq!(max_row_width, 600);

    let [top1, left1, width1, height1] = layout[4..8] else {
        unreachable!()
    };
    assert_eq!(top1, 0);
    assert_eq!(left1, 0);
    assert_eq!(width1, 533);
    assert_eq!(height1, 300);
    //
    let [top2, left2, width2, height2] = layout[8..12] else {
        unreachable!()
    };
    assert_eq!(top2, height1 + spacing as i32);
    assert_eq!(left2, 0);
    assert_eq!(width2, 600);
    assert_eq!(height2, 300);

    let [top3, left3, width3, height3] = layout[12..16] else {
        unreachable!()
    };
    assert_eq!(top3, height1 + spacing as i32 + height2 + spacing as i32);
    assert_eq!(left3, 0);
    assert_eq!(width3, 168);
    assert_eq!(height3, 300);
}

#[wasm_bindgen_test]
fn one_box_on_each_row_with_scaling() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let row_width = 600.0;
    let row_height = 300.0;
    let spacing = 2.0;
    let height_tolerance = 0.15;

    let layout = get_justified_layout(
        input.as_slice(),
        row_height,
        row_width,
        spacing,
        height_tolerance,
    );
    assert_eq!(layout.len(), 16);
    let max_row_width = layout[0];
    assert_eq!(max_row_width, 600);

    let [top1, left1, width1, height1] = layout[4..8] else {
        unreachable!()
    };
    assert_eq!(top1, 0);
    assert_eq!(left1, 0);
    assert_eq!(width1, 600);
    assert_eq!(height1, 337);
    //
    let [top2, left2, width2, height2] = layout[8..12] else {
        unreachable!()
    };
    assert_eq!(top2, height1 + spacing as i32);
    assert_eq!(left2, 0);
    assert_eq!(width2, 600);
    assert_eq!(height2, 300);

    let [top3, left3, width3, height3] = layout[12..16] else {
        unreachable!()
    };
    assert_eq!(top3, height1 + spacing as i32 + height2 + spacing as i32);
    assert_eq!(left3, 0);
    assert_eq!(width3, 194);
    assert_eq!(height3, 345);
}
