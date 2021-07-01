use std::fs;
use std::fs::metadata;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut dir = String::from("");
    let mut filename = String::from("");
    let mut files: Vec<String> = Vec::new();

    print!("Directory: ");

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut dir)
        .expect("Error reading user input.");

    print!("Filename: ");

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut filename)
        .expect("Error reading user input.");

    dir.pop();
    filename.pop();

    check_dir(&mut files, &dir, &filename);

    println!("Possible files founded:");

    for file in files {
        println!("  {}", file);
    }
}

fn check_dir(files: &mut Vec<String>, dir_path: &str, search_by: &str) {
    println!("Scanning '{}' directory", dir_path);
    let paths = match fs::read_dir(dir_path) {
        Ok(val) => val,
        Err(_) => return
    };
    for path in paths {
        let path = path.unwrap().path();
        let file_path = path.to_str().unwrap();

        let md = match metadata(file_path) {
            Ok(val) => val,
            Err(_) => return
        };

        if md.is_dir() {
            check_dir(files, &file_path, &search_by);
        }
        if path.file_name().unwrap() == search_by {
            files.push(String::from(file_path));
        }
    }
}
