#![cfg(not(target_arch = "wasm32"))]

use justified_layout::native::get_justified_layout_simd;
use justified_layout::{LayoutOptions, _get_justified_layout};

fn run_both(input: &[f32], options: LayoutOptions) -> Vec<f32> {
    let scalar = _get_justified_layout(input, options);
    let alg = get_justified_layout_simd(input, options);
    assert_eq!(scalar.len(), alg.len(), "Length mismatch");
    for (i, (s, a)) in scalar.iter().zip(alg.iter()).enumerate() {
        assert!(
            (s - a).abs() < 0.5,
            "Algebraic diverged at index {i}: scalar={s}, algebraic={a} (diff={})", (s - a).abs()
        );
    }
    alg
}

#[test]
fn fits_perfectly_on_one_row() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions::new(
        300.0,
        900.0,
        0.0,
        0.0,
    );

    let layout = run_both(&input, options);
    assert_eq!(layout.len(), 16);
    assert!((layout[0] - 900.0).abs() < 0.01);
    assert!((layout[1] - 300.0).abs() < 0.01);
}

#[test]
fn applies_spacing() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions::new(
        300.0,
        904.0,
        2.0,
        0.0,
    );
    run_both(&input, options);
}

#[test]
fn expands_row_based_on_height_tolerance() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions::new(
        300.0,
        994.0,
        2.0,
        0.1,
    );
    run_both(&input, options);
}

#[test]
fn uses_target_height_if_max_height_cannot_fill_row() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions::new(
        300.0,
        1000.0,
        2.0,
        0.1,
    );
    run_both(&input, options);
}

#[test]
fn adds_second_row_due_to_spacing() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions::new(
        300.0,
        900.0,
        2.0,
        0.0,
    );
    run_both(&input, options);
}

#[test]
fn positions_boxes_with_different_aspect_ratios() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let options = LayoutOptions::new(
        300.0,
        900.0,
        2.0,
        0.0,
    );
    run_both(&input, options);
}

#[test]
fn scales_boxes_with_different_aspect_ratios_when_using_height_tolerance() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let options = LayoutOptions::new(
        300.0,
        900.0,
        2.0,
        0.2,
    );
    run_both(&input, options);
}

#[test]
fn one_square_box_on_each_row() {
    let input: Vec<f32> = vec![1.0, 1.0, 1.0];
    let options = LayoutOptions::new(
        300.0,
        599.0,
        2.0,
        0.0,
    );
    run_both(&input, options);
}

#[test]
fn different_shaped_boxes_on_each_row() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let options = LayoutOptions::new(
        300.0,
        600.0,
        2.0,
        0.0,
    );
    run_both(&input, options);
}

#[test]
fn one_box_on_each_row_with_scaling() {
    let input: Vec<f32> = vec![16.0 / 9.0, 2.0, 9.0 / 16.0];
    let options = LayoutOptions::new(
        300.0,
        600.0,
        2.0,
        0.15,
    );
    run_both(&input, options);
}

#[test]
fn add_box_to_full_row_when_it_helps() {
    let input: Vec<f32> = vec![
        1.5, 0.6666666666666666, 1.3274336283185841, 1.3333333333333333,
        0.7516666666666667, 1.5, 0.665, 1.4018691588785046, 1.3392857142857142,
    ];
    let options = LayoutOptions::new(
        75.0,
        350.0,
        4.0,
        0.15,
    );
    run_both(&input, options);
}

#[test]
fn fills_last_row_when_within_max_row_height() {
    let input: Vec<f32> = vec![
        1.5, 0.6666666666666666, 1.3274336283185841, 1.3333333333333333,
        0.7516666666666667, 1.5, 0.665, 1.4018691588785046, 1.3392857142857142,
        0.5625,
    ];
    let options = LayoutOptions::new(
        100.0,
        640.0,
        2.0,
        0.2,
    );
    run_both(&input, options);
}

#[test]
fn empty_input() {
    let scalar = _get_justified_layout(&[], LayoutOptions::new(300.0, 900.0, 0.0, 0.0));
    let alg = get_justified_layout_simd(&[], LayoutOptions::new(300.0, 900.0, 0.0, 0.0));
    assert!(scalar.is_empty());
    assert!(alg.is_empty());
}

#[test]
fn single_item() {
    let input: Vec<f32> = vec![1.5];
    let options = LayoutOptions::new(
        100.0,
        500.0,
        2.0,
        0.0,
    );
    run_both(&input, options);
}

#[test]
fn simd_large_input() {
    let mut input = Vec::with_capacity(10_000);
    let mut seed: u32 = 42;
    for _ in 0..10_000 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let ratio = 0.5 + 2.0 * (seed >> 16) as f32 / 65536.0;
        input.push(ratio);
    }
    let options = LayoutOptions::new(
        235.0,
        1000.0,
        2.0,
        0.15,
    );
    run_both(&input, options);
}