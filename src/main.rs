use image::*;
mod hash;
use hash::SmallXXHash;
/*
struct HashVisualization {
    resolution : i32,
    hashes : Vec<u32>,
}*/

fn frac(x: f32) -> f32 {
    x - x.floor()
}

fn main() {
    let mut imgbuf: image::ImageBuffer<Luma<u8>, Vec<_>> = image::ImageBuffer::new(32 as u32, 32 as u32);
    
    /*let resolution = 32 * 32;
    let inv_resolution = 1.0 / resolution as f32;
    for (i, pixel) in imgbuf.pixels_mut().enumerate() {
        let v = (inv_resolution * (i as f32) + 0.00001).floor() as u32;
        let u = (i as u32) - resolution * v;
        let mut hash = SmallXXHash::new(0);
        hash.eat(u);
        hash.eat(v);
        let g = hash.get() as u8;
        println!("U: '{u}' V: '{v}' Grayscale: '{g}'");
        *pixel = image::Luma([g]);
    }*/

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut hash = SmallXXHash::new(0);
        hash.eat((x as i32 - 16) as u32);
        hash.eat((15 - y as i32) as u32);
        let g = hash.get() as u8;
        println!("X: '{x}' Y: '{y}' Grayscale: '{g}'");
        *pixel = image::Luma([g]);
    }

    imgbuf.save("test1.png").unwrap();

    println!("Done! Image ready!");
}
