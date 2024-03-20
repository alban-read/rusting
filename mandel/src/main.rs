use image::{ImageBuffer, Rgb};
use num_cpus;
use std::sync::{Arc, Mutex};
use num_complex::Complex;

fn get() -> usize {
    num_cpus::get()
}

fn time_function<F, R, Args>(func: F, args: Args) -> R
    where // consume captured arguments
        F: FnOnce(Args) -> R,
{
    use std::time::Instant;
    let func_type_name = std::any::type_name::<F>();
    let start_time = Instant::now();
    //
    let result = func(args);
    //
    let elapsed_time = start_time.elapsed();
    println!("Function {} executed in {:.2?} seconds.", func_type_name, elapsed_time);
    result
}

const HEIGHT: usize = 1080;
const WIDTH: usize = 1920;

fn create_rgb_vector() -> Vec<(u8, u8, u8)> {
    let mut color_vector: Vec<(u8, u8, u8)> = Vec::with_capacity(256);

    // Shades of green
    for i in 0..32 {
        color_vector.push((0, i * 8, 0));
    }
    // Shades of blue
    for i in 0..32 {
        color_vector.push((0, 0, i * 8));
    }
    // Shades of magenta
    for i in 0..32 {
        color_vector.push((i * 8, 0, i * 8));
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

fn mandelbrot(x: i32, y: i32) -> u8 {
    const MAX_ITERATIONS: u32 = 64;
    let c = Complex::new(
        (x as f64 - WIDTH as f64 / 2.0) * 4.0 / WIDTH as f64,
        (y as f64 - HEIGHT as f64 / 2.0) * 4.0 / HEIGHT as f64
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

    // Map the number of iterations to a color gradient
    // return (255.0 * (n as f64 / MAX_ITERATIONS as f64)) as u8;
    return n as u8; // index into colour
}

#[derive(Clone)]
struct SharedState {
    data: Vec<u8>,
}

fn worker(id: usize, state: Arc<Mutex<SharedState>>) {

    for y in (id..HEIGHT).step_by(get())  {
        for x in 0..WIDTH {
            let value = mandelbrot(x as i32, y as i32);
            let mut data = state.lock().unwrap();
            data.data[y * WIDTH + x] = value;
        }
    }
}

fn make_mandelbrot(thread_count:usize) -> (Vec<(u8, u8, u8)>, Arc<Mutex<SharedState>>) {
    let colours = create_rgb_vector();
    let state = Arc::new(Mutex::new(SharedState { data: vec![0; WIDTH * HEIGHT] }));

    // Start one worker per CPU.
    let workers: Vec<_> = (0..thread_count).map(|id| {
        let state_clone = Arc::clone(&state);
        std::thread::spawn(move || worker(id, state_clone))
    }).collect();

    // Wait for all workers to finish.
    for worker in workers {
        worker.join().unwrap();
    }
    (colours, state)
}

fn main() {

    let (colours, state) = time_function(make_mandelbrot, get());

    let guard = state.lock().unwrap();
    let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let data = &guard.data;
        let index  = [data[(y as usize) * WIDTH + x as usize]][0];
        if let Some(color) = colours.get(index as usize) {
            let (r, g, b) = color;
            *pixel = Rgb([*r, *g, *b]);
        }
    }

    image.save("mandelbrot.png").unwrap();
}

