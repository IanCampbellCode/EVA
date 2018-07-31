//extern crate toml;

use std::process::Command;
use std::fmt;
//use toml::Value as Toml;

const REPO_LOCATION: &str = "/resources/repositories.toml";

struct RepoData {
    name: String,
    url: String,
    build_command: String
}
impl fmt::Display for RepoData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(name: {}, url: {}, build_command: {})", self.name, self.url, self.build_command)
    }
}


fn main() {
    //Read repository toml file
    let repositories = read_repository_toml(REPO_LOCATION);
    //Perform Git Calls
    for repo in repositories {

    }
    //Perform Build Commands

}


fn read_repository_toml(file_location: &str) -> Vec<RepoData> {
    use std::fs::File;
    use std::io::Read;

    let mut repositories = Vec::new();
    let mut input = String::new();

    let mut file = File::open(file_location).expect("Failed to open repositories.toml");
    file.read_to_string(&mut input).expect("Failed to read repositories file to string");
    println!("{}", input);

    return repositories;
}

/// Spawns a command process which calls the given git repos
fn call_git(repo: RepoData) {
    //format url into a usable git command
    println!("{}", repo);
}

/// spawns a command process which runs the given git command
fn call_build(build_command: &str) {
    //format build commands into usable command
    println!("{}", build_command);
}

/// checks if the given repository exists
fn repository_exists(directory: &str) {
    
}

fn create_process(command: &str) {
    match Command::new(command)
        .spawn() {
            Ok(_) => {
                println!("okay!");
            },
            Err(err) => {
                println!("Error found! {}",err);
            }
    }
}
