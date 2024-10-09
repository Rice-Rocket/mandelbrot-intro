#[allow(unused_imports)]
use util::{Color, Complex, MandelbrotImage, Point};

mod util;

// The resolution of the image in pixels.
// Final image will have IMAGE_SIZE x IMAGE_SIZE pixels.
const IMAGE_SIZE: u32 = 2048;

fn main() {
    // Create a new image with a width and height of IMAGE_SIZE.
    let mut im = MandelbrotImage::new(IMAGE_SIZE, IMAGE_SIZE);
        
    // Loop through the x and y dimensions of the image.
    for x in 0..IMAGE_SIZE {
        for y in 0..IMAGE_SIZE {
            // Make every pixel black
            im.put_pixel(x, y, Color::BLACK.into());
        }
    }

    // Save the image to the file 'mandelbrot.png'
    im.save("mandelbrot.png").unwrap()
}
