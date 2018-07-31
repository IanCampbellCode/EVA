use std::fs;

fn main() {
    fs::create_dir_all("/target/debug/resources").expect("Could not create /target/debug/resources");
    fs::create_dir_all("/target/release/resources").expect("Could not create /target/release/resources");

    fs::copy("/src/resources/repositories.toml", "/target/debug/resources").expect("could not copy repositories file to debug");
    fs::copy("/src/resources/repositories.toml", "/target/release/resources").expect("could not copy repositories file to release");
}