extern crate toml;

//use std::process::Command;
use std::fmt;
use toml::Value as Toml;

const REPO_LOCATION: &str = "resources/repositories.toml";

struct RepoData {
    name: String,
    url: String,
    build_command: String
}
impl fmt::Display for RepoData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(name: {}\nurl: {}\nbuild_command: {})", self.name, self.url, self.build_command)
    }
}


fn main() {
    let repositories = read_repository_toml(REPO_LOCATION);
    for repository in repositories {
        println!("{}", repository);
    }
    //Perform Build Commands

}


fn read_repository_toml(file_location: &str) -> Vec<RepoData> {
    use std::fs::File;
    use std::io::Read;

    let repositories = Vec::new();

    let mut file = File::open(file_location).expect("Could not find repository file");
    
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to parse the repository file");

    let toml: Toml;
    match file_contents.parse::<Toml>() {
        Ok(parsed_data) => {
            toml = parsed_data;
        }
        Err(error) => {
            println!("Failed to parse the repositories file: {}", error);
            panic!();
        }
    }
    println!("{}", toml["repositories"].is_table());
    return repositories;
}
/*
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
*/
