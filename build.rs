use std::env;
use std::path::Path;
use std::process::Command;
use std::fs;

fn main() {
    println!("cargo::rerun-if-changed=src/components");
    println!("cargo::rerun-if-changed=src/views");
    println!("cargo::rerun-if-changed=input.css");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=public/scripts");

    let source_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    // Create tailwind css
    // This is nice
    let tailwind_input = Path::new(&source_dir).join("input.css");
    let tailwind_output = Path::new(&source_dir).join("public/styles/main.css");
    Command::new("tailwindcss").args(["-i", &tailwind_input.display().to_string(), "-o", &tailwind_output.display().to_string()]).output().expect("Failed to copy public");
    
    // Copy css
    let source_file = Path::new(&source_dir).join("public/styles/main.css");
    let destination_folder = Path::new(&source_dir).join("target/debug/public/styles");
    let destination_file = Path::new(&destination_folder).join("main.css");
    if !destination_folder.exists() {
        fs::create_dir_all(&destination_folder).expect("Unable to create destination folder");
    }
    fs::copy(source_file, &destination_file).expect("Error copying css");

    // Copy js
    let source_file = Path::new(&source_dir).join("public/js/store.js");
    let destination_folder = Path::new(&source_dir).join("target/debug/public/js");
    if !destination_folder.exists() {
        fs::create_dir_all(&destination_folder).expect("Unable to create destination folder");
    }
    let destination_file = Path::new(&destination_folder).join("stores.js");
    fs::copy(source_file, &destination_file).expect("Error copying css");
}