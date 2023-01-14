use image::*;
use crate::hash::SmallXXHash;

pub struct HashVisualization;

impl HashVisualization {
    pub fn generate_image(width : u32, height: u32, path: String)
    {
        let mut imgbuf: image::ImageBuffer<Luma<u8>, Vec<_>> = image::ImageBuffer::new(width, height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let u = x as i32 - (width / 2) as i32;
            let v = (height / 2) as i32 - y as i32;
            let g = SmallXXHash::new(0).eat(u).eat(v).get() as u8;
            println!("U: '{u}' V: '{v}' Grayscale: '{g}'");
            *pixel = image::Luma([g]);
        }

        imgbuf.save(path).unwrap();
    }
}