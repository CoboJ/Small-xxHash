use image::*;

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
    let invResolution = 1.0 / resolution as f32;
    for (i, pixel) in imgbuf.pixels_mut().enumerate() {
        let v = (invResolution * i as f32 + 0.00001).floor();
        let u = (i as f32 - resolution as f32 * v);
        let g = (frac(u * v * 0.381) * 256.0) as u8;
        println!("U: '{u}' V: '{v}' Grayscale: '{g}'");
        *pixel = image::Luma([g]);
    }*/

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let g = (frac(x as f32 * (31 - y) as f32 * 0.381) * 256.0) as u8;
        *pixel = image::Luma([g]);
    }

    imgbuf.save("test.png").unwrap();

    println!("Done! Image ready!");
}
