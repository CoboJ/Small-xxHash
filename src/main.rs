mod hash;
mod hash_visualization;
use hash_visualization::HashVisualization;

fn main() {
    HashVisualization::
        color_img(64, 64, String::from("colored_hash.png"));
    HashVisualization::
        noise_img(64, 64, String::from("uncolored_hash.png"));

    println!("Done! Image ready!");
}
