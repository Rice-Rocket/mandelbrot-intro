use util::{Color, Complex, MandelbrotImage, Point};

mod util;

// The number of iterations used to find the edge of the mandelbrot set.
// Note that larger numbers here will be slower to calculate, but lead to more detailed results.
const NUM_ITERATIONS: u32 = 100;
// The resolution of the image in pixels.
// Final image will have IMAGE_SIZE x IMAGE_SIZE pixels.
const IMAGE_SIZE: u32 = 2048;
// Where the rendered set is centered.
// A value of (-0.625, 0.0) will center the set and (0.25, 0.0) will zoom in on main inset.
const CENTER: Complex<f32> = Complex::new(0.00164372, -0.8224676);
// The scale of the rendered set.
// A smaller scale zooms in more.
// When the center is (-0.625, 0.0), a scale of 0.7 will contain the entire set.
const SCALE: f32 = 0.1;

fn mandelbrot(c: Complex<f32>) -> f32 {
    // Initialize `z` at `c`.
    let mut z = c;

    // Loop through the following for NUM_ITERATIONS times.
    for n in 0..NUM_ITERATIONS {
        // Update `z` via the Mandelbrot function
        // z = z² + c
        z = z * z + c;

        // If `z` escapes the set, exit
        if z.abs() > 2.0 {
            // Color the pixel based on the number of iterations it completed before escaping.
            // This leads to brighter colors near the edge of the set.
            // The logarithm ensures a nice falloff even with more iterations, while clamping it
            // ensures no pixels are too bright or dark.
            return f32::log(n as f32, NUM_ITERATIONS as f32).clamp(0.0, 1.0);
        }
    }

    // If `z` never escapes the set, return 0
    0.0
}

#[inline]
fn transform(mut c: Complex<f32>) -> Complex<f32> {
    // Transform c to be in [-2, 2] in both real and imaginary axes.
    c = c * 4.0 - 2.0;
    // Transform c based on the dictated scale and center.
    c * SCALE + CENTER
}

fn main() {
    // Create a new image with a width and height of IMAGE_SIZE.
    let mut im = MandelbrotImage::new(IMAGE_SIZE, IMAGE_SIZE);
        
    // Loop through the x and y dimensions of the image.
    for x in 0..IMAGE_SIZE {
        for y in 0..IMAGE_SIZE {
            // Instantiate a new point for the current pixel.
            let p = Point::new(x, y);

            // Convert the pixel position to u, v coordinates in [0, 1] based on the image
            // size.
            let uv = p.to_uv(IMAGE_SIZE);

            // Convert u, v coordinates to a complex number. 
            // This directly maps x position of `uv` to real component of `c`
            // and y position of `uv` to imaginary component of `c`.
            let mut c = Complex::from(uv);

            // Transform `c` into the correct frame.
            c = transform(c);

            // Evaluate the mandelbrot set at `c`.
            let m = mandelbrot(c);

            // Plot the result at the pixel coordinates.
            im.put_pixel(x, y, Color::grayscale(m));
        }
    }

    // Save the image to the file 'mandelbrot.png'
    im.save("mandelbrot.png").unwrap()
}
