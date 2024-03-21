use image::{ImageBuffer, Rgb};
use num_complex::Complex;
use rayon::prelude::*;

fn time_function<F, R>(func: F) -> R
    where
        F: FnOnce() -> R,
{
    use std::time::Instant;
    let start_time = Instant::now();
    let result = func();
    let elapsed_time = start_time.elapsed();
    println!("Execution time: {:.2?}", elapsed_time);
    result
}

const HEIGHT: usize = 1080;
const WIDTH: usize = 1920;

fn create_rgb_vector() -> Vec<(u8, u8, u8)> {
    let mut color_vector: Vec<(u8, u8, u8)> = Vec::with_capacity(256);

    // Shades of magenta
    for i in 0..32 {
        color_vector.push((i * 8, 0, i * 8));
    }

    // Shades of blue
    for i in 0..32 {
        color_vector.push((0, 0, i * 8));
    }

    // Shades of green
    for i in 0..32 {
        color_vector.push((0, i * 8, 0));
    }

    // Shades of red
    for i in 0..32 {
        color_vector.push((i * 8, 0, 0));
    }
    // Shades of cyan
    for i in 0..32 {
        color_vector.push((0, i * 8, i * 8));
    }
    // Shades of purple
    for i in 0..32 {
        color_vector.push((i * 4, 0, i * 8));
    }
    // Shades of orange
    for i in 0..32 {
        color_vector.push((i * 8, i * 4, 0));
    }
    // Shades of grey
    for i in 0..32 {
        let shade = i * 8;
        color_vector.push((shade, shade, shade));
    }

    color_vector
}

fn mandelbrot(x: usize, y: usize) -> u8 {
    const MAX_ITERATIONS: u32 = 96;
    let c = Complex::new(
        x as f64 / WIDTH as f64 * 3.0 - 2.0,
        y as f64 / HEIGHT as f64 * 2.0 - 1.0,
    );
    let mut z = Complex::new(0.0, 0.0);
    let mut n = 0;

    while z.norm() <= 2.0 && n < MAX_ITERATIONS {
        z = z * z + c;
        n += 1;
    }

    if n == MAX_ITERATIONS {
        return 0;
    }
    return n as u8; // index into colour
}

fn parallel_brot() -> Vec<Vec<u8>> {
    let pixels: Vec<_> = (0..HEIGHT)
        .into_par_iter()
        .map(|y| {
            (0..WIDTH)
                .into_par_iter()
                .map(move |x| mandelbrot(x, y))
                .collect()
        })
        .collect();
    pixels
}

fn main() {
    let colours = create_rgb_vector();
    let pixels = time_function(|| parallel_brot());

    let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let index = pixels[y as usize][x as usize];
        if let Some(color) = colours.get(index as usize) {
            let (r, g, b) = color;
            *pixel = Rgb([*r, *g, *b]);
        }
    }

    image.save("mandelbrot.png").unwrap();
}
