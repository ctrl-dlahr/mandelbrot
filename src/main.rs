use std::fs::File;
use std::io::{BufWriter, Write};
use num_complex::Complex;

const IMAGE_WIDTH: i32 = 300;
const IMAGE_HEIGHT: i32 = 300;

fn draw_mandelbrot(val: f32, mut writer: impl Write) {
    write!(writer, "{} 0 0\n", val as i32).expect("unable to write to file.");
}

fn algorithm(x: f32, y: f32) -> f32 {
    let dy = 1.5;
    let dx = 0.5;
    let brightness = 15.0;
    // position (x,y)
    let position = Complex::new((x / IMAGE_WIDTH as f32) - dy, (y/IMAGE_WIDTH as f32) - dx); 
    let mut z = Complex::new(0.0, 0.0);
    let mut fractal_shape = 0.0;
    // moving every point (translation)
    while Complex::norm(z) < 2.0 && fractal_shape <= brightness {
        z = z * z + position;
        fractal_shape += 0.5;
    }
    if fractal_shape < 20.0 {
        return (255.0 * fractal_shape) / brightness;
    } else {
        return 0.0
    }
}

fn main() {
    let file = File::create("img/mandelbrot.ppm").expect("unable to create image.ppm");
    let mut writer = BufWriter::new(&file);
    write!(&mut writer, "P3\n{} {} 255\n", IMAGE_WIDTH, IMAGE_HEIGHT).expect("unable to write to file.");
    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            draw_mandelbrot(algorithm(x as f32, y as f32), &mut writer);
        }
    }
}