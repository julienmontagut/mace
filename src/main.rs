use clap::{arg, value_parser, Command};
use std::{env::current_dir, path::PathBuf};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = Command::new(APP_NAME)
        .version(APP_VERSION)
        .author("Your Name")
        .about("A simple shell")
        .args(&[
            arg!(-c --config <PATH> "Path to the config file"),
            arg!([PATH] "A file or folder to open")
                .id("path")
                .value_parser(value_parser!(PathBuf)),
        ])
        .get_matches();

    if let Some(text) = matches.get_one::<String>("config") {
        println!("{}", text);
    }

    let mut workspace = current_dir().ok();
    let mut open_files: Vec<PathBuf> = vec![];

    if let Some(path) = matches.get_one::<PathBuf>("path") {
        if path.is_dir() {
            workspace = Some(path.to_owned());
        } else {
            open_files.push(path.clone());
        }
    }

    println!("Welcome to ted");
    println!("Workspace: {:?}", workspace);
    println!("Files: {:?}", open_files);
}
