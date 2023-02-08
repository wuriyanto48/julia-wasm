use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use wasm_bindgen::prelude::*;
use image;

mod base64;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, drawx!");
}

#[wasm_bindgen]
pub fn hello(s: &str) -> String {
    let mut out_encode: Vec<u8> = Vec::new();
    if let Err(_) = base64::encoder::encode(&mut s.as_bytes(), &mut out_encode) {
        return String::from("error");
    }

    let out_encode_str = match String::from_utf8(out_encode) {
        Ok(o) => o,
        Err(_) => {
            return String::from("error");
        }
    };

    out_encode_str
}

#[wasm_bindgen]
pub fn generate_julia(real_number: f32, im_number: f32, width: u32, height: u32) -> String {
    // let real_number = -0.4;
    // let im_number = 0.6;

    let imgx = width;
    let imgy = height;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let escape_radius = 50.0;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // let r = (0.5 * x as f32) as u8;
        // let b = (0.8 * y as f32) as u8;

        // black
        *pixel = image::Rgb([0, 0, 0]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(real_number, im_number);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= escape_radius {
                // quadratic julia
                z = z * z + c;
                
                // sin julia
                // z = z.sin() * c;

                // cos julia
                // z = z.cos() * c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let data = (*pixel as image::Rgb<u8>).0;

            let red = (i << 3) as u8;
            let green = data[1];
            let blue = (i << 2) as u8;
            
            *pixel = image::Rgb([red, green, blue]);
        }
    }

    let mut img_c = Cursor::new(Vec::new());
    if let Err(_) = imgbuf.write_to(&mut img_c, image::ImageFormat::Png) {
        return String::from("error write image to buffer");
    }

    if let Err(_) = img_c.seek(SeekFrom::Start(0)) {
        return String::from("error seeking buffer");
    }

    let mut out_encode: Vec<u8> = Vec::new();
    let mut img_v = Vec::new();

    if let Err(_) = img_c.read_to_end(&mut img_v){
        return String::from("error convert cursor buffer to buffer");
    }

    if let Err(_) = base64::encoder::encode(&mut &img_v[..], &mut out_encode) {
        return String::from("error");
    }

    let out_encode_str = match String::from_utf8(out_encode) {
        Ok(o) => o,
        Err(_) => {
            return String::from("error");
        }
    };

    out_encode_str
}