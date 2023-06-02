use std::env;
use std::path::PathBuf;
extern crate embed_resource;

fn main() {
    // Get the directory of this build script
    let dir = env::current_dir().expect("Failed to get current directory");

    // Construct the path to your resource file relative to the individual Cargo.tomls
    // 1. package -> src
    // 2. src -> root
    // 3. root -> assets
    let mut res_path = PathBuf::from(&dir);
    res_path.push("../../assets/resources.rc");

    // Print out the pathbuf
    println!("Relative path to the resource file: {:?}", res_path);

    // Convert the path to absolute, if it isn't already
    let absolute_res_path = res_path
        .canonicalize()
        .expect("Failed to get absolute path");

    // Print out the absolute path
    println!(
        "Absolute path to the resource file: {:?}",
        absolute_res_path
    );

    embed_resource::compile(absolute_res_path, embed_resource::NONE);
}
