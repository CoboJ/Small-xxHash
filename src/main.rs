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
    let mut imgbuf: image::ImageBuffer<Luma<u8>, Vec<_>> = image::ImageBuffer::new(41 as u32, 41 as u32);
    
    for (i, pixel) in imgbuf.pixels_mut().enumerate() {
        let g = (frac(i as f32 * 0.381) * 256.0) as u8;
        *pixel = image::Luma([g]);
    }

    imgbuf.save("test.png").unwrap();

    println!("Done! Image ready!");
}
