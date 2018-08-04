use std::fs;

///An under-optimized pre-build script which copies the repositories file from src to the target folders
fn main() {
    //Create resource folders if they do not exist
    fs::create_dir_all("target/debug/resources").expect("Could not create /target/debug/resources");
    fs::create_dir_all("target/release/resources").expect("Could not create /target/release/resources");

    //Copy repository file from source to target folders (Even if the exist because this isn't optimized)
    fs::copy("src/resources/repositories.toml", "target/debug/resources/repositories.toml").expect("could not copy repositories file to debug");
    fs::copy("src/resources/repositories.toml", "target/release/resources/repositories.toml").expect("could not copy repositories file to release");
}
