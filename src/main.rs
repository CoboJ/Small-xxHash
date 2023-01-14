mod hash;
mod hash_visualization;
use hash_visualization::HashVisualization;

fn main() {
    HashVisualization::generate_image(32, 32, String::from("test.png"));
    println!("Done! Image ready!");
}
