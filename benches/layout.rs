#![feature(test)]
#![cfg(not(target_arch = "wasm32"))]

extern crate test;

use justified_layout::{Layout, LayoutOptions};
use test::Bencher;

fn generate_aspect_ratios(n: usize) -> Vec<f32> {
    let mut ratios = Vec::with_capacity(n);
    let mut seed: u32 = 42;
    for _ in 0..n {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        ratios.push(0.5 + 2.0 * (seed >> 16) as f32 / 65536.0);
    }
    ratios
}

const OPTIONS: LayoutOptions = LayoutOptions::new(235.0, 1000.0, 2.0, 0.15);

#[bench]
fn layout_100(b: &mut Bencher) {
    let input = generate_aspect_ratios(100);
    b.iter(|| Layout::new(test::black_box(&input), &OPTIONS));
}

#[bench]
fn layout_1000(b: &mut Bencher) {
    let input = generate_aspect_ratios(1_000);
    b.iter(|| Layout::new(test::black_box(&input), &OPTIONS));
}

#[bench]
fn layout_10000(b: &mut Bencher) {
    let input = generate_aspect_ratios(10_000);
    b.iter(|| Layout::new(test::black_box(&input), &OPTIONS));
}

#[bench]
fn layout_100000(b: &mut Bencher) {
    let input = generate_aspect_ratios(100_000);
    b.iter(|| Layout::new(test::black_box(&input), &OPTIONS));
}

#[bench]
fn layout_1000000(b: &mut Bencher) {
    let input = generate_aspect_ratios(1_000_000);
    b.iter(|| Layout::new(test::black_box(&input), &OPTIONS));
}
