#![cfg(not(target_arch = "wasm32"))]

use justified_layout::native::get_justified_layout_simd;
use justified_layout::{LayoutOptions, _get_justified_layout};

fn run_both(input: &[f32], options: LayoutOptions) -> (Vec<f32>, Vec<f32>) {
    let scalar = _get_justified_layout(input, options);
    let simd = get_justified_layout_simd(input, options);
    assert_eq!(scalar.len(), simd.len(), "Length mismatch");
    for (i, (s, d)) in scalar.iter().zip(simd.iter()).enumerate() {
        assert!(
            (s - d).abs() < 0.01,
            "SIMD diverged at index {i}: scalar={s}, simd={d}"
        );
    }
    (scalar, simd)
}

#[test]
fn fits_perfectly_on_one_row() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions {
        row_height: 300.0,
        row_width: 900.0,
        spacing: 0.0,
        tolerance: 0.0,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout.len(), 16);
    assert_eq!(layout[0], 900.0);
    assert_eq!(layout[1], 300.0);

    assert_eq!(layout[4], 0.0); // top
    assert_eq!(layout[5], 0.0); // left
    assert_eq!(layout[6], 300.0); // width
    assert_eq!(layout[7], 300.0); // height

    assert_eq!(layout[8], 0.0);
    assert_eq!(layout[9], 300.0);
    assert_eq!(layout[10], 300.0);
    assert_eq!(layout[11], 300.0);

    assert_eq!(layout[12], 0.0);
    assert_eq!(layout[13], 600.0);
    assert_eq!(layout[14], 300.0);
    assert_eq!(layout[15], 300.0);
}

#[test]
fn applies_spacing() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions {
        row_height: 300.0,
        row_width: 904.0,
        spacing: 2.0,
        tolerance: 0.0,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout.len(), 16);
    assert_eq!(layout[0], 904.0);
    assert_eq!(layout[1], 300.0);

    assert_eq!(layout[4], 0.0);
    assert_eq!(layout[5], 0.0);
    assert_eq!(layout[6], 300.0);
    assert_eq!(layout[7], 300.0);

    assert_eq!(layout[9], 302.0); // left = 300 + 2
    assert_eq!(layout[13], 604.0); // left = 300 + 2 + 300 + 2
}

#[test]
fn expands_row_based_on_height_tolerance() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions {
        row_height: 300.0,
        row_width: 994.0,
        spacing: 2.0,
        tolerance: 0.1,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout.len(), 16);
    assert_eq!(layout[0], 994.0);
    assert_eq!(layout[1], 330.0);

    assert_eq!(layout[6], 330.0); // width
    assert_eq!(layout[7], 330.0); // height
}

#[test]
fn uses_target_height_if_max_height_cannot_fill_row() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions {
        row_height: 300.0,
        row_width: 1000.0,
        spacing: 2.0,
        tolerance: 0.1,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout[0], 904.0);
    assert_eq!(layout[1], 300.0);
    assert_eq!(layout[6], 300.0);
    assert_eq!(layout[7], 300.0);
}

#[test]
fn adds_second_row_due_to_spacing() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions {
        row_height: 300.0,
        row_width: 900.0,
        spacing: 2.0,
        tolerance: 0.0,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout[0], 602.0); // max row width
    assert_eq!(layout[1], 602.0); // total height

    // First two items on row 1
    assert_eq!(layout[4], 0.0);
    assert_eq!(layout[8], 0.0);
    // Third item on row 2
    assert_eq!(layout[12], 302.0); // top = 300 + 2
    assert_eq!(layout[13], 0.0); // left = 0
}

#[test]
fn positions_boxes_with_different_aspect_ratios() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let options = LayoutOptions {
        row_height: 300.0,
        row_width: 900.0,
        spacing: 2.0,
        tolerance: 0.0,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout[0], 770.75);
    assert_eq!(layout[1], 602.0);

    assert_eq!(layout[6], 300.0 * (16.0 / 9.0)); // width1
    assert_eq!(layout[7], 300.0);

    assert_eq!(layout[8], 302.0); // top2 = row 2
    assert_eq!(layout[9], 0.0); // left2
    assert_eq!(layout[10], 600.0); // width2 = 300 * 2
}

#[test]
fn scales_boxes_with_different_aspect_ratios_when_using_height_tolerance() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let options = LayoutOptions {
        row_height: 300.0,
        row_width: 900.0,
        spacing: 2.0,
        tolerance: 0.2,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout[0], 900.0);
    assert_eq!(layout[1], 712.439);
    assert_eq!(layout[6], 640.0);
    assert_eq!(layout[7], 360.0);
}

#[test]
fn one_square_box_on_each_row() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions {
        row_height: 300.0,
        row_width: 599.0,
        spacing: 2.0,
        tolerance: 0.0,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout[0], 300.0);
    assert_eq!(layout[1], 904.0);

    assert_eq!(layout[4], 0.0);
    assert_eq!(layout[8], 302.0); // row 2
    assert_eq!(layout[12], 604.0); // row 3
}

#[test]
fn different_shaped_boxes_on_each_row() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let options = LayoutOptions {
        row_height: 300.0,
        row_width: 600.0,
        spacing: 2.0,
        tolerance: 0.0,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout[0], 600.0);
    assert_eq!(layout[1], 904.0);
    assert_eq!(layout[6], 533.3333);
    assert_eq!(layout[10], 600.0);
    assert_eq!(layout[14], 168.75);
}

#[test]
fn one_box_on_each_row_with_scaling() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let options = LayoutOptions {
        row_height: 300.0,
        row_width: 600.0,
        spacing: 2.0,
        tolerance: 0.15,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout[0], 600.0);
    assert_eq!(layout[1], 337.5 + 2.0 + 300.0 + 2.0 + 300.0);
    assert_eq!(layout[6], 600.0);
    assert_eq!(layout[7], 337.5);
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
    let options = LayoutOptions {
        row_height: 75.0,
        row_width: 350.0,
        spacing: 4.0,
        tolerance: 0.15,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout.len(), 40);
    assert!((layout[0] - 350.0).abs() < 0.01);

    // Spot-check a few values
    assert!((layout[6] - 105.02475).abs() < 0.01); // width1
    assert!((layout[7] - 70.0165).abs() < 0.01); // height1
    assert!((layout[20] - (70.0165 + 4.0)).abs() < 0.01); // top5 = row 2
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
    let options = LayoutOptions {
        row_height: 100.0,
        row_width: 640.0,
        spacing: 2.0,
        tolerance: 0.2,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout.len(), 44);
    assert!((layout[0] - 640.0).abs() < 0.01);
    assert!((layout[1] - 230.84763).abs() < 0.01);
}

#[test]
fn empty_input() {
    let input: Vec<f32> = vec![];
    let options = LayoutOptions {
        row_height: 300.0,
        row_width: 900.0,
        spacing: 0.0,
        tolerance: 0.0,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout.len(), 0);
}

#[test]
fn single_item() {
    let input: Vec<f32> = vec![1.5];
    let options = LayoutOptions {
        row_height: 100.0,
        row_width: 500.0,
        spacing: 2.0,
        tolerance: 0.0,
    };

    let (_, layout) = run_both(&input, options);
    assert_eq!(layout.len(), 8);
    assert_eq!(layout[4], 0.0); // top
    assert_eq!(layout[5], 0.0); // left
}

#[test]
fn simd_large_input() {
    // Generate deterministic aspect ratios
    let mut input = Vec::with_capacity(10_000);
    let mut seed: u32 = 42;
    for _ in 0..10_000 {
        // Simple LCG for deterministic "random" values in [0.5, 2.5]
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let ratio = 0.5 + 2.0 * (seed >> 16) as f32 / 65536.0;
        input.push(ratio);
    }

    let options = LayoutOptions {
        row_height: 235.0,
        row_width: 1000.0,
        spacing: 2.0,
        tolerance: 0.15,
    };

    run_both(&input, options);
}
