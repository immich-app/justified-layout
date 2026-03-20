use justified_layout::{Layout, LayoutOptions};

fn generate_aspect_ratios(n: usize) -> Vec<f32> {
    let mut ratios = Vec::with_capacity(n);
    let mut seed: u32 = 42;
    for _ in 0..n {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        ratios.push(0.5 + 2.0 * (seed >> 16) as f32 / 65536.0);
    }
    ratios
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "scalar".into());
    let input = generate_aspect_ratios(1_000_000);
    let options = LayoutOptions::new(235.0, 1000.0, 2.0, 0.15);

    let iterations = 100;

    if mode == "scalar" {
        for _ in 0..iterations {
            std::hint::black_box(Layout::new(
                std::hint::black_box(&input),
                &options,
            ));
        }
    }
}