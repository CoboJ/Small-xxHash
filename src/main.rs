mod hash;
mod hash_visualization;
use hash_visualization::HashVisualization;

fn main() {
    HashVisualization::
        color_img(32, 32, String::from("test.png"));
    println!("Done! Image ready!");
}
