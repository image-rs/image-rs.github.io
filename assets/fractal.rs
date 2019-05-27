//! An example of generating julia fractals.
extern crate image;
extern crate num_complex;

// This is gamma pre-corrected color of the linear gradient background.
const BACKGROUND: u16 = 40;

// Maximum color saturation (as basically 8-bit).
const MAGNITUDE: u8 = 20;

fn main() {
    let imgx = 3200;
    let imgy = 3200/2;

    let scalex = 3.0 / imgx as f32;
    let scaley = 1.5 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    let red = num_complex::Complex::from_polar(&(MAGNITUDE as f32), &0.0);
    let green = num_complex::Complex::from_polar(&(MAGNITUDE as f32), &5.25);
    let blue = num_complex::Complex::from_polar(&(MAGNITUDE as f32), &2.11);

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 0.75;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i: u8 = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([
                brightness((z.norm()*red).re as i8, i),
                brightness((z.norm()*green).re as i8, i),
                brightness((z.norm()*blue).re as i8, i)]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}

fn brightness(input: i8, iter: u8) -> u8 {
    // Available color manipulation depending on iter value.
    const ITER_RANGE: u16 = 255 - BACKGROUND;
    let input = (input as i16 + 128) as u16;
    let iter = u16::from(iter);
    let premultiplied = (iter.saturating_sub(32) * input) / 224;
    // iter scaled into the iter range.
    let in_range = (premultiplied * ITER_RANGE) >> 8;
    (in_range + BACKGROUND) as u8
}
