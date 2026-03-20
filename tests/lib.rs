use justified_layout::{Layout, LayoutOptions};

#[test]
fn fits_perfectly_on_one_row() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions::new(300.0, 900.0, 0.0, 0.0);

    let layout = Layout::new(&input, &options);
    assert_eq!(layout.len(), 3);
    assert_eq!(layout.width(), 900.0);
    assert_eq!(layout.height(), 300.0);

    let boxes = layout.boxes();
    assert_eq!(boxes[0].top, 0.0);
    assert_eq!(boxes[0].left, 0.0);
    assert_eq!(boxes[0].width, 300.0);
    assert_eq!(boxes[0].height, 300.0);

    assert_eq!(boxes[1].top, 0.0);
    assert_eq!(boxes[1].left, 300.0);
    assert_eq!(boxes[1].width, 300.0);
    assert_eq!(boxes[1].height, 300.0);

    assert_eq!(boxes[2].top, 0.0);
    assert_eq!(boxes[2].left, 600.0);
    assert_eq!(boxes[2].width, 300.0);
    assert_eq!(boxes[2].height, 300.0);
}

#[test]
fn applies_spacing() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let spacing = 2.0;
    let options = LayoutOptions::new(300.0, 904.0, spacing, 0.0);

    let layout = Layout::new(&input, &options);
    assert_eq!(layout.len(), 3);
    assert_eq!(layout.width(), 904.0);
    assert_eq!(layout.height(), 300.0);

    let boxes = layout.boxes();
    assert_eq!(boxes[0].top, 0.0);
    assert_eq!(boxes[0].left, 0.0);
    assert_eq!(boxes[0].width, 300.0);
    assert_eq!(boxes[0].height, 300.0);

    assert_eq!(boxes[1].top, 0.0);
    assert_eq!(boxes[1].left, 300.0 + spacing);
    assert_eq!(boxes[1].width, 300.0);
    assert_eq!(boxes[1].height, 300.0);

    assert_eq!(boxes[2].top, 0.0);
    assert_eq!(boxes[2].left, 300.0 + spacing + 300.0 + spacing);
    assert_eq!(boxes[2].width, 300.0);
    assert_eq!(boxes[2].height, 300.0);
}

#[test]
fn expands_row_based_on_height_tolerance() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let spacing = 2.0;
    let options = LayoutOptions::new(300.0, 994.0, spacing, 0.1);

    let layout = Layout::new(&input, &options);
    assert_eq!(layout.len(), 3);
    assert_eq!(layout.width(), 994.0);
    assert_eq!(layout.height(), 330.0);

    let boxes = layout.boxes();
    assert_eq!(boxes[0].top, 0.0);
    assert_eq!(boxes[0].left, 0.0);
    assert_eq!(boxes[0].width, 330.0);
    assert_eq!(boxes[0].height, 330.0);

    assert_eq!(boxes[1].top, 0.0);
    assert_eq!(boxes[1].left, 330.0 + spacing);
    assert_eq!(boxes[1].width, 330.0);
    assert_eq!(boxes[1].height, 330.0);

    assert_eq!(boxes[2].top, 0.0);
    assert_eq!(boxes[2].left, 330.0 + spacing + 330.0 + spacing);
    assert_eq!(boxes[2].width, 330.0);
    assert_eq!(boxes[2].height, 330.0);
}

#[test]
fn uses_target_height_if_max_height_cannot_fill_row() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let spacing = 2.0;
    let options = LayoutOptions::new(300.0, 1000.0, spacing, 0.1);

    let layout = Layout::new(&input, &options);
    assert_eq!(layout.len(), 3);
    assert_eq!(layout.width(), 904.0);
    assert_eq!(layout.height(), 300.0);

    let boxes = layout.boxes();
    assert_eq!(boxes[0].top, 0.0);
    assert_eq!(boxes[0].left, 0.0);
    assert_eq!(boxes[0].width, 300.0);
    assert_eq!(boxes[0].height, 300.0);

    assert_eq!(boxes[1].top, 0.0);
    assert_eq!(boxes[1].left, 300.0 + spacing);
    assert_eq!(boxes[1].width, 300.0);
    assert_eq!(boxes[1].height, 300.0);

    assert_eq!(boxes[2].top, 0.0);
    assert_eq!(boxes[2].left, 300.0 + spacing + 300.0 + spacing);
    assert_eq!(boxes[2].width, 300.0);
    assert_eq!(boxes[2].height, 300.0);
}

#[test]
fn adds_second_row_due_to_spacing() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let row_height = 300.0;
    let spacing = 2.0;
    let options = LayoutOptions::new(row_height, 900.0, spacing, 0.0);

    let layout = Layout::new(&input, &options);
    assert_eq!(layout.len(), 3);
    assert_eq!(layout.width(), 602.0);
    assert_eq!(layout.height(), 602.0);

    let boxes = layout.boxes();
    assert_eq!(boxes[0].top, 0.0);
    assert_eq!(boxes[0].left, 0.0);
    assert_eq!(boxes[0].width, 300.0);
    assert_eq!(boxes[0].height, 300.0);

    assert_eq!(boxes[1].top, 0.0);
    assert_eq!(boxes[1].left, 300.0 + spacing);
    assert_eq!(boxes[1].width, 300.0);
    assert_eq!(boxes[1].height, 300.0);

    assert_eq!(boxes[2].top, row_height + spacing);
    assert_eq!(boxes[2].left, 0.0);
    assert_eq!(boxes[2].width, 300.0);
    assert_eq!(boxes[2].height, 300.0);
}

#[test]
fn positions_boxes_with_different_aspect_ratios() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let spacing = 2.0;
    let options = LayoutOptions::new(300.0, 900.0, spacing, 0.0);

    let layout = Layout::new(&input, &options);
    assert_eq!(layout.len(), 3);
    assert_eq!(layout.width(), 770.75);
    assert_eq!(layout.height(), 602.0);

    let boxes = layout.boxes();
    assert_eq!(boxes[0].top, 0.0);
    assert_eq!(boxes[0].left, 0.0);
    assert_eq!(boxes[0].width, 300.0 * (16.0 / 9.0));
    assert_eq!(boxes[0].height, 300.0);

    assert_eq!(boxes[1].top, boxes[0].height + spacing);
    assert_eq!(boxes[1].left, 0.0);
    assert_eq!(boxes[1].width, 300.0 * 2.0);
    assert_eq!(boxes[1].height, 300.0);

    assert_eq!(boxes[2].top, boxes[0].height + spacing);
    assert_eq!(boxes[2].left, boxes[1].width + spacing);
    assert_eq!(boxes[2].width, 300.0 * (9.0 / 16.0));
    assert_eq!(boxes[2].height, 300.0);
}

#[test]
fn scales_boxes_with_different_aspect_ratios_when_using_height_tolerance() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let spacing = 2.0;
    let options = LayoutOptions::new(300.0, 900.0, spacing, 0.2);

    let layout = Layout::new(&input, &options);
    assert_eq!(layout.len(), 3);
    assert_eq!(layout.width(), 900.0);
    assert_eq!(layout.height(), 712.439);

    let boxes = layout.boxes();
    assert_eq!(boxes[0].top, 0.0);
    assert_eq!(boxes[0].left, 0.0);
    assert_eq!(boxes[0].width, 640.0);
    assert_eq!(boxes[0].height, 360.0);

    assert_eq!(boxes[1].top, boxes[0].height + spacing);
    assert_eq!(boxes[1].left, 0.0);
    assert_eq!(boxes[1].width, 700.87805);
    assert_eq!(boxes[1].height, 350.43903);

    assert_eq!(boxes[2].top, boxes[0].height + spacing);
    assert_eq!(boxes[2].left, boxes[1].width + spacing);
    assert_eq!(boxes[2].width, 197.12195);
    assert_eq!(boxes[2].height, 350.43903);
}

#[test]
fn one_square_box_on_each_row() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let spacing = 2.0;
    let options = LayoutOptions::new(300.0, 599.0, spacing, 0.0);

    let layout = Layout::new(&input, &options);
    assert_eq!(layout.len(), 3);
    assert_eq!(layout.width(), 300.0);
    assert_eq!(layout.height(), 904.0);

    let boxes = layout.boxes();
    assert_eq!(boxes[0].top, 0.0);
    assert_eq!(boxes[0].left, 0.0);
    assert_eq!(boxes[0].width, 300.0);
    assert_eq!(boxes[0].height, 300.0);

    assert_eq!(boxes[1].top, boxes[0].height + spacing);
    assert_eq!(boxes[1].left, 0.0);
    assert_eq!(boxes[1].width, 300.0);
    assert_eq!(boxes[1].height, 300.0);

    assert_eq!(boxes[2].top, boxes[0].height + spacing + boxes[1].height + spacing);
    assert_eq!(boxes[2].left, 0.0);
    assert_eq!(boxes[2].width, 300.0);
    assert_eq!(boxes[2].height, 300.0);
}

#[test]
fn different_shaped_boxes_on_each_row() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let spacing = 2.0;
    let options = LayoutOptions::new(300.0, 600.0, spacing, 0.0);

    let layout = Layout::new(&input, &options);
    assert_eq!(layout.len(), 3);
    assert_eq!(layout.width(), 600.0);
    assert_eq!(layout.height(), 904.0);

    let boxes = layout.boxes();
    assert_eq!(boxes[0].top, 0.0);
    assert_eq!(boxes[0].left, 0.0);
    assert_eq!(boxes[0].width, 533.3333);
    assert_eq!(boxes[0].height, 300.0);

    assert_eq!(boxes[1].top, boxes[0].height + spacing);
    assert_eq!(boxes[1].left, 0.0);
    assert_eq!(boxes[1].width, 600.0);
    assert_eq!(boxes[1].height, 300.0);

    assert_eq!(boxes[2].top, boxes[0].height + spacing + boxes[1].height + spacing);
    assert_eq!(boxes[2].left, 0.0);
    assert_eq!(boxes[2].width, 168.75);
    assert_eq!(boxes[2].height, 300.0);
}

#[test]
fn one_box_on_each_row_with_scaling() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let spacing = 2.0;
    let options = LayoutOptions::new(300.0, 600.0, spacing, 0.15);

    let layout = Layout::new(&input, &options);
    assert_eq!(layout.len(), 3);
    assert_eq!(layout.width(), 600.0);
    assert_eq!(layout.height(), 337.5 + 2.0 + 300.0 + 2.0 + 300.0);

    let boxes = layout.boxes();
    assert_eq!(boxes[0].top, 0.0);
    assert_eq!(boxes[0].left, 0.0);
    assert_eq!(boxes[0].width, 600.0);
    assert_eq!(boxes[0].height, 337.5);

    assert_eq!(boxes[1].top, boxes[0].height + spacing);
    assert_eq!(boxes[1].left, 0.0);
    assert_eq!(boxes[1].width, 600.0);
    assert_eq!(boxes[1].height, 300.0);

    assert_eq!(boxes[2].top, boxes[0].height + spacing + boxes[1].height + spacing);
    assert_eq!(boxes[2].left, 0.0);
    assert_eq!(boxes[2].width, 168.75);
    assert_eq!(boxes[2].height, 300.0);
}

#[test]
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
    let spacing = 4.0;
    let options = LayoutOptions::new(75.0, 350.0, spacing, 0.15);

    let layout = Layout::new(&input, &options);
    assert_eq!(layout.len(), 9);
    assert_eq!(layout.width(), 350.00003);

    let boxes = layout.boxes();
    assert_eq!(boxes[0].top, 0.0);
    assert_eq!(boxes[0].left, 0.0);
    assert_eq!(boxes[0].width, 105.02475);
    assert_eq!(boxes[0].height, 70.0165);

    assert_eq!(boxes[1].top, 0.0);
    assert_eq!(boxes[1].left, boxes[0].width + spacing);
    assert_eq!(boxes[1].width, 46.67767);
    assert_eq!(boxes[1].height, 70.0165);

    assert_eq!(boxes[2].top, 0.0);
    assert_eq!(boxes[2].left, boxes[0].width + spacing + boxes[1].width + spacing);
    assert_eq!(boxes[2].width, 92.94225);
    assert_eq!(boxes[2].height, 70.0165);

    assert_eq!(boxes[3].top, 0.0);
    assert_eq!(
        boxes[3].left,
        boxes[0].width + spacing + boxes[1].width + spacing + boxes[2].width + spacing
    );
    assert_eq!(boxes[3].width, 93.35534);
    assert_eq!(boxes[3].height, 70.0165);

    assert_eq!(boxes[4].top, boxes[0].height + spacing);
    assert_eq!(boxes[4].left, 0.0);
    assert_eq!(boxes[4].width, 58.830894);
    assert_eq!(boxes[4].height, 78.267265);

    assert_eq!(boxes[5].top, boxes[0].height + spacing);
    assert_eq!(boxes[5].left, boxes[4].width + spacing);
    assert_eq!(boxes[5].width, 117.400894);
    assert_eq!(boxes[5].height, 78.267265);

    assert_eq!(boxes[6].top, boxes[0].height + spacing);
    assert_eq!(boxes[6].left, boxes[4].width + spacing + boxes[5].width + spacing);
    assert_eq!(boxes[6].width, 52.047733);
    assert_eq!(boxes[6].height, 78.267265);

    assert_eq!(boxes[7].top, boxes[0].height + spacing);
    assert_eq!(
        boxes[7].left,
        boxes[4].width + spacing + boxes[5].width + spacing + boxes[6].width + spacing
    );
    assert_eq!(boxes[7].width, 109.72047);
    assert_eq!(boxes[7].height, 78.267265);

    assert_eq!(boxes[8].top, boxes[0].height + spacing + boxes[4].height + spacing);
    assert_eq!(boxes[8].left, 0.0);
    assert_eq!(boxes[8].width, 104.822235);
    assert_eq!(boxes[8].height, 78.267265);

    assert_eq!(
        layout.height(),
        boxes[0].height + spacing + boxes[4].height + spacing + boxes[8].height
    );
}

#[test]
fn fills_last_row_when_within_max_row_height() {
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
        0.5625,
    ];
    let options = LayoutOptions::new(100.0, 640.0, 2.0, 0.2);

    let layout = Layout::new(&input, &options);
    assert_eq!(layout.len(), 10);
    assert_eq!(layout.width(), 640.00006);
    assert_eq!(layout.height(), 230.84763);

    let boxes = layout.boxes();
    assert_eq!(boxes[9].top, 115.279915);
    assert_eq!(boxes[9].left, 574.99316);
    assert_eq!(boxes[9].width, 65.00684);
    assert_eq!(boxes[9].height, 115.56772);
}

#[test]
fn empty_input() {
    let options = LayoutOptions::new(300.0, 900.0, 0.0, 0.0);
    let layout = Layout::new(&[], &options);
    assert_eq!(layout.len(), 0);
    assert_eq!(layout.width(), 0.0);
    assert_eq!(layout.height(), 0.0);
    assert!(layout.boxes().is_empty());
}
