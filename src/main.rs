extern crate glob;
extern crate colored;
extern crate lazy_static;

use std::io;
use colored::Colorize;
use std::error::Error;
use std::str;
use std::path::PathBuf;
use glob::glob_with;
use glob::MatchOptions;
//use std::io::Write;
use std::fs;
//use std::env;
use clap::{App, load_yaml};
use lazy_static::lazy_static;
use regex::Regex;

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
    
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut files = Vec::new();
    for file in glob_with(pattern, options).unwrap(){
        match file {
            Ok(path) => files.push(path),
            Err(e) => eprintln!("Error{}", e)
        }
    };

    files
}

fn rename(filenames: Vec<PathBuf>) -> Result<(), Box <dyn Error>> {

    lazy_static! {
        static ref RE: Regex = Regex::new("[\\\\ $@~+&'\\{\\}\\[\\]%^*#]|[A-Z]").unwrap();
    };

    if filenames.len() == 0 {
        println!("No files found");
        std::process::exit(1);
    }

    //println!("{:#?}", filenames);

    let mut count = 0_i32;
    for path in filenames.iter() {

        if RE.is_match(&path.to_string_lossy()) {

            let new_name: String = path.to_string_lossy()
                .replace(&[' ', '$', '@', '~', '=', '&', '#', '*', '\'', '\"', '\\', '{', '}', '%', '[', ']', '^'][..], "")
                .to_lowercase();

            fs::rename(path, &new_name)?;
            count += 1;

            println!("{}\t{}\twas found and was renamed to\t{}", count.to_string().green(), &path.to_string_lossy().magenta(), new_name.yellow());
        }
        

    }
    Ok(())
}
