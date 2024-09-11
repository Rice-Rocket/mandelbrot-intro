use complex::Complex;
use image::{buffer::ConvertBuffer, GenericImage, ImageBuffer, Rgb, Rgb32FImage, RgbImage, Rgba32FImage};
use rand::{thread_rng, Rng};

mod complex;

const NUM_SAMPLES: usize = 100000000;
const NUM_ITERATIONS: u32 = 100;
const IMAGE_SIZE: u32 = 2048;

fn mandelbrot(c: Complex<f32>, n: u32) -> f32 {
    let mut z = c;

    for _ in 0..n {
        z = z * z + c;
        if z.abs() > 2.0 {
            return 1.0;
        }
    }

    0.0
}

#[inline]
fn transform(c: Complex<f32>) -> Complex<f32> {
    c * 4.0 - 2.0
}

fn main() {
    let mut im = RgbImage::new(IMAGE_SIZE, IMAGE_SIZE);
    let mut rng = thread_rng();
        
    for _ in 0..NUM_SAMPLES {
        let r1 = rng.gen::<f32>();
        let r2 = rng.gen::<f32>();

        let mut c = Complex::new(r1, r2);
        c = transform(c);

        let px = r1 * IMAGE_SIZE as f32;
        let py = r2 * IMAGE_SIZE as f32;

        let m = mandelbrot(c, NUM_ITERATIONS);

        let mc = (m * 255.0) as u8;
        im.put_pixel(px as u32, py as u32, Rgb([mc, mc, mc]));
    }

    im.save("mandelbrot.png").unwrap()
}
