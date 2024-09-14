use util::{Color, Complex, MandelbrotImage, Point};

mod util;

// The number of iterations used to find the edge of the mandelbrot set.
// Note that larger numbers here will be slower to calculate, but lead to more detailed results.
const NUM_ITERATIONS: u32 = 1000;
const INV_NUM_INTERATIONS: f32 = 1.0 / NUM_ITERATIONS as f32;
// The resolution of the image in pixels.
// Final image will have IMAGE_SIZE x IMAGE_SIZE pixels.
const IMAGE_SIZE: u32 = 2048;
// Where the rendered set is centered.
// A value of (-0.625, 0.0) will center the set and (0.25, 0.0) will zoom in on main inset.
const CENTER: Complex<f32> = Complex::new(-0.7525073, 0.040764004);
// The scale of the rendered set.
// A smaller scale zooms in more.
// When the center is (-0.625, 0.0), a scale of 0.7 will contain the entire set.
const SCALE: f32 = 0.00025;

fn mandelbrot(c: Complex<f32>) -> f32 {
    // Initialize `z` at `c`.
    let mut z_re = c.re;
    let mut z_im = c.im;

    // Initialize cached squares of z_re and z_im.
    let mut z_re_2 = z_re * z_re;
    let mut z_im_2 = z_im * z_im;

    // Loop through the following for NUM_ITERATIONS times.
    for n in 0..NUM_ITERATIONS {
        // Update `z` via the Mandelbrot function:
        // z = z² + c
        //
        // By some algebriac simplification this reduces down to:
        // y = Im(z² + c)
        //   = Im(x² - y² + 2ixy + x₀ + iy₀)  <-- Because we only want imaginary component, we only
        //                  ^^^^        ^^^       care about terms with `i` in them.
        //   = 2xy + y₀
        //
        // x = Re(z² + c)
        //   = Re(x² - y² + 2ixy + x₀ + iy₀)  <-- Because we only want real component, we only
        //        ^^^^^^^          ^^             care about terms without `i` in them.
        //   = x² - y² + x₀
        //
        // where:
        // z = x + iy
        // z² = (x² + iy²) = x² - y² + 2ixy
        // c = x₀ + y₀
        z_im = 2.0 * z_re * z_im + c.im;
        z_re = z_re_2 - z_im_2 + c.re;

        // Update cached squares of z_re and z_im.
        z_re_2 = z_re * z_re;
        z_im_2 = z_im * z_im;

        // Compute the square of the absolute value (magnitude) of `z`.
        // This is equivalent to square of its distance from the origin.
        // This is faster than computing just the magnitude because we remove the need for a sqrt()
        // which is incredibly slow in comparison to addition and multiplication.
        // Here, the squared magnitude is computed via the pythagorean theorem, a² + b² = c²
        // where a = z_re, b = z_im, and c = z_mag.
        let z_mag_2 = z_re_2 + z_im_2;

        // If `z` escapes the set, exit.
        // Since we are now testing the square of `z_mag`, we also make sure we square the opposite
        // side of the inequality (2² = 4).
        // z_mag > 2
        // z_mag² > 2²
        if z_mag_2 > 4.0 {
            // Return the weight based on orbit trapping.
            // Coloring based on the square of the distance changes virtually nothing, even causing
            // a nicer falloff, so its fine to just replace `z_mag` here.
            return weight(n, z_mag_2);
        }
    }

    // If `z` never escapes the set, return 0
    0.0
}

#[inline]
fn weight(n: u32, z_mag_2: f32) -> f32 {
    // Color based on distance from the set.
    // Logarithms ensure a nice falloff. 
    let nu = f32::log2(f32::log10(z_mag_2) / 2.0);
    // Weight the color also based on the number of iterations to demonstrate the finer details
    // that come with higher iteration counts.
    let w = ((n + 1) as f32 - nu) * INV_NUM_INTERATIONS;
    // Clamp to ensure no pixels are too bright or dark.
    w.clamp(0.0, 1.0)
}

#[inline]
fn transform(mut c: Complex<f32>) -> Complex<f32> {
    // Transform c to be in [-2, 2] in both real and imaginary axes.
    c = c * 4.0 - 2.0;
    // Transform c based on the dictated scale and center.
    c * SCALE + CENTER
}

#[inline]
fn palette(mut t: f32, phase: f32) -> Color {
    // DC Offset
    const A: Color = Color::new(0.5, 0.5, 0.5);
    // Amplitude
    const B: Color = Color::new(0.5, 0.5, 0.5);
    // Frequency
    const C: Color = Color::new(1.0, 1.0, 1.0);
    // Phase
    const D: Color = Color::new(0.0, 0.1, 0.2);

    // Shift `t` by the `phase` and wrap around at the integer bound.
    t = (t + phase).fract();

    // Procedural palette generator by Inigo Quilez.
    // See: http://iquilezles.org/articles/palettes/
    //
    // Generate new palettes using: http://dev.thi.ng/gradients/
    A + B * Color::cos(std::f32::consts::TAU * (C * t + D))
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

            // Find the color associated with weight `m` on the mandelbrot set.
            let rgb = palette(m, 0.4);

            // Plot the result at the pixel coordinates.
            im.put_pixel(x, y, rgb.into());
        }
    }

    // Save the image to the file 'mandelbrot.png'
    im.save("mandelbrot.png").unwrap()
}
