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

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let hash = (x + y) as f32;
        println!("Hash value: '{hash}' based on: '{x} + {y}'");
        let g = (frac(hash * 0.381) * 256.0) as u8;
        *pixel = image::Luma([g]);
    }
    //let v : i32 =  
    //let u : i32 = 
    imgbuf.save("test.png").unwrap();

    println!("Done! Image ready!");
}
