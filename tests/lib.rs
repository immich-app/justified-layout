use justified_layout::get_justified_layout;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

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

    let max_row_height = layout[1];
    assert_eq!(max_row_height, 330);

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

    let max_row_height = layout[1];
    assert_eq!(max_row_height, 602);

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

    let max_row_height = layout[1];
    assert_eq!(max_row_height, 602);

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

    let max_row_height = layout[1];
    assert_eq!(max_row_height, 712 + 1); // 1 to account for ceil()

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

    let max_row_height = layout[1];
    assert_eq!(max_row_height, 904);

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

    let max_row_height = layout[1];
    assert_eq!(max_row_height, 904);

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

    let max_row_height = layout[1];
    assert_eq!(max_row_height, 337 + 2 + 300 + 2 + 345 + 1); // 1 to account for ceil()

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

#[wasm_bindgen_test]
fn add_box_to_full_row_when_it_helps() {
    let input: Vec<f32> = vec![
        1.5,
        0.6666666666666666,
        1.3274336283185841,
        1.3333333333333333,
        0.7516666666666667,
        1.5,
        0.665,
        1.4018691588785046,
        1.3392857142857142,
    ];
    let row_width = 350.0;
    let row_height = 75.0;
    let spacing = 4.0;
    let height_tolerance = 0.15;

    let layout = get_justified_layout(
        input.as_slice(),
        row_height,
        row_width,
        spacing,
        height_tolerance,
    );
    // assert_eq!(layout, vec![]);
    assert_eq!(layout.len(), 40);
    let max_row_width = layout[0];
    assert_eq!(max_row_width, 351);

    let [top1, left1, width1, height1] = layout[4..8] else {
        unreachable!()
    };
    assert_eq!(top1, 0);
    assert_eq!(left1, 0);
    assert_eq!(width1, 105);
    assert_eq!(height1, 70);

    let [top2, left2, width2, height2] = layout[8..12] else {
        unreachable!()
    };
    assert_eq!(top2, 0);
    assert_eq!(left2, width1 + spacing as i32);
    assert_eq!(width2, 46);
    assert_eq!(height2, 70);

    let [top3, left3, width3, height3] = layout[12..16] else {
        unreachable!()
    };
    assert_eq!(top3, 0);
    assert_eq!(left3, width1 + spacing as i32 + width2 + spacing as i32);
    assert_eq!(width3, 92);
    assert_eq!(height3, 70);

    let [top4, left4, width4, height4] = layout[16..20] else {
        unreachable!()
    };
    assert_eq!(top4, 0);
    assert_eq!(
        left4,
        width1 + spacing as i32 + width2 + spacing as i32 + width3 + spacing as i32 + 1
    );
    assert_eq!(width4, 93); // should be 92
    assert_eq!(height4, 70);

    let [top5, left5, width5, height5] = layout[20..24] else {
        unreachable!()
    };
    assert_eq!(top5, height1 + spacing as i32);
    assert_eq!(left5, 0);
    assert_eq!(width5, 58);
    assert_eq!(height5, 78);

    let [top6, left6, width6, height6] = layout[24..28] else {
        unreachable!()
    };
    assert_eq!(top6, height1 + spacing as i32);
    assert_eq!(left6, width5 + spacing as i32);
    assert_eq!(width6, 117);
    assert_eq!(height6, 78);

    let [top7, left7, width7, height7] = layout[28..32] else {
        unreachable!()
    };
    assert_eq!(top7, height1 + spacing as i32);
    assert_eq!(left7, width5 + spacing as i32 + width6 + spacing as i32 + 1);
    assert_eq!(width7, 52);
    assert_eq!(height7, 78);

    let [top8, left8, width8, height8] = layout[32..36] else {
        unreachable!()
    };
    assert_eq!(top8, height1 + spacing as i32);
    assert_eq!(
        left8,
        width5 + spacing as i32 + width6 + spacing as i32 + width7 + spacing as i32 + 1
    );
    assert_eq!(width8, 109);
    assert_eq!(height8, 78);

    let [top9, left9, width9, height9] = layout[36..40] else {
        unreachable!()
    };
    assert_eq!(top9, height1 + spacing as i32 + height5 + spacing as i32);
    assert_eq!(left9, 0);
    assert_eq!(width9, 115);
    assert_eq!(height9, 86);

    let max_row_height = layout[1];
    assert_eq!(
        max_row_height,
        height1 + spacing as i32 + height5 + spacing as i32 + height9 + 1
    );
}
