extern crate glob;

use std::error::Error;
use std::str;
use std::path::PathBuf;
use glob::glob;
use std::io::Write;
use std::fs;
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        writeln!(std::io::stderr(),
                "Usage:\nfspaces [PATTERN]").unwrap();
        std::process::exit(1);
    }

    let pattern = &args[1];

    println!("Hello, world! {:?}", filenames(&pattern[..]));

    rename(filenames(&pattern[..]));
}

fn filenames(pattern: &str) -> Vec<PathBuf> {
    
    let mut files = Vec::new();
    for file in glob(pattern).expect("Error") {
        match file {
            Ok(path) => files.push(path),
            Err(e) => println!("{}", e)
        }
    };

    files
}

fn rename(filenames: Vec<PathBuf>) -> Result<(), Box<Error>> {
    if filenames.len() == 0 {
        println!("No files found");
    }

    for path in filenames.iter() {

        if path.to_string_lossy().contains(" ") {

            let new_name = str::replace(&path.to_string_lossy(), " ", "_");

            fs::rename(path, &new_name)?;

            println!("=====> {}", new_name);

        }
        

    }

    Ok(())
    
}
