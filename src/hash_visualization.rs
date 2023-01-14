use image::*;
use crate::hash::SmallXXHash;

pub struct HashVisualization;

impl HashVisualization {
    pub fn noise_img(width : u32, height: u32, path: String)
    {
        let mut imgbuf: image::ImageBuffer<Luma<u8>, Vec<_>> = image::ImageBuffer::new(width, height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let u = x as i32 - (width / 2) as i32;
            let v = (height / 2) as i32 - y as i32;
            let hash = SmallXXHash::new(1922732698).eat(u).eat(v).get();
            let g = hash as u8;
            *pixel = image::Luma([g]);
        }

        imgbuf.save(path).unwrap();
    }

    pub fn color_img(width : u32, height: u32, path: String)
    {
        let mut imgbuf: image::ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::new(width, height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let u = x as i32 - (width / 2) as i32;
            let v = (height / 2) as i32 - y as i32;
            let hash = SmallXXHash::new(1922732698).eat(u).eat(v).get();
            let r = hash as u8;
            let g = hash.wrapping_shr(8) as u8;
            let b = hash.wrapping_shr(16) as u8;
            *pixel = image::Rgb([r, g, b]);
        }

        imgbuf.save(path).unwrap();
    }
}