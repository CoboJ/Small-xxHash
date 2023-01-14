mod hash;
mod hash_visualization;
use hash_visualization::HashVisualization;

fn main() {
    HashVisualization::
        color_img(256, 256, String::from("colored_hash.png"));
    HashVisualization::
        noise_img(256, 256, String::from("uncolored_hash.png"));

    println!("Done! Image ready!");
}
