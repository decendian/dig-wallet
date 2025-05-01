use std::env;
use std::path::Path;

fn main() {
    // Get the project root directory path (parent of crates)
    let _out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    // Tell Cargo to re-run this script if the resources directory changes
    println!("cargo:rerun-if-changed=../resources");
    
    // Print the path to resources for debugging
    println!("Resources path: {}", Path::new(&manifest_dir).join("../resources").display());
}