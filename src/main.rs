struct HashVisualization {
    resolution : i32,
    hashes : Vec<u32>,
}

fn main() {
    let hashVisualization = HashVisualization {
        resolution : 16,
        hashes : vec![0; 16*16]
    };

    println!("Hello, world!");
}
