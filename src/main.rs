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
    
    let mut i = 0;
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        let g = (frac(i as f32 * 0.381) * 256.0) as u8;
        println!("Hash value: '{g}' at index: '{i}'");
        *pixel = image::Luma([g]);
        i += 1;
    }

    imgbuf.save("test.png").unwrap();

    println!("Done! Image ready!");
}
