use image::Rgb;

extern crate image;
/*
struct HashVisualization {
    resolution : i32,
    hashes : Vec<u32>,
}*/

fn main() {
    let mut imgbuf: image::ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::new(16 as u32, 16 as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = ((x as f32/16.0) * 255.0) as u8;
        let g = ((y as f32/16.0) * 255.0) as u8;
        *pixel = image::Rgb([r, g, 0]);
    }
    //let v : i32 =  
    //let u : i32 = 
    imgbuf.save("test.png").unwrap();

    println!("Done! Image ready!");
}
