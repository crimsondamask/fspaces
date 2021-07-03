extern crate glob;
extern crate colored;

use std::io;
use colored::Colorize;
use std::error::Error;
use std::str;
use std::path::PathBuf;
use glob::glob;
//use std::io::Write;
use std::fs;
//use std::env;
use clap::{App, load_yaml};

fn main() {
    
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if matches.is_present("input") {

        for pattern in matches.values_of("PATTERN").unwrap() {

            rename(filenames(&pattern[..]))
                .expect("Couldn't rename files");
        }

    } else {

        let mut files = Vec::new();
        loop {

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(len) => if len == 0 {
                    break;
                } else {
                    input = input.trim().to_string();
                    files.push(input);
                }
                Err(er) => {
                    eprintln!("Error: {}", er);
                    return;
                }
            }
        }

        for file in files.iter() {

            rename(filenames(&file[..]))
                .expect("Couldn't rename files");
        }
    } 
}


fn filenames(pattern: &str) -> Vec<PathBuf> {
    
    let mut files = Vec::new();
    for file in glob(pattern).expect("Error") {
        match file {
            Ok(path) => files.push(path),
            Err(e) => eprintln!("Error{}", e)
        }
    };

    files
}

fn rename(filenames: Vec<PathBuf>) -> Result<(), Box <dyn Error>> {

    let mut count = 0;

    if filenames.len() == 0 {
        println!("No files found");
        std::process::exit(1);
    }

    //println!("{:#?}", filenames);

    for path in filenames.iter() {

        if path.to_string_lossy().contains(" ") || 
            path.to_string_lossy().contains("&") || 
            path.to_string_lossy().contains("$") || 
            path.to_string_lossy().contains("~") ||
            path.to_string_lossy().contains("~") ||
            path.to_string_lossy().contains("\'") ||
            path.to_string_lossy().contains("=") ||
            path.to_string_lossy().contains("\"") 
        {

            let new_name: String = path.to_string_lossy()
                .replace(&[' ', '$', '@', '~', '=', '&', '\'', '\"'][..], "");

            fs::rename(path, &new_name)?;

            println!("{}\t ===>\t {}", &path.to_string_lossy().magenta(), new_name.yellow());

            count += 1;
        }
        

    }

    match count {
        1 => println!("1 file found matching pattern and has been renamed."),
        _ => println!("{} files found matching pattern and have been renamed.", count),
    }

    Ok(())
    
}
