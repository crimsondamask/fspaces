extern crate glob;
extern crate colored;

use std::io;
use colored::Colorize;
use std::error::Error;
use std::str;
use std::path::PathBuf;
use glob::glob;
use std::io::Write;
use std::fs;
use std::env;

fn main() {
    
    let patterns: Vec<String> = env::args().skip(1).collect();

    if patterns.len() == 1 && patterns[0] == "-p" {

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

    } else if patterns.len() != 0 {

        for pattern in patterns.iter() {
            
            rename(filenames(&pattern[..]))
                .expect("Couldn't rename files");

        }

    } else {
        
        writeln!(std::io::stderr(), "fspaces: Error occured while parsing arguments")
            .unwrap();
        writeln!(std::io::stderr(),
                "USAGE:\nfspaces [PATTERN]").unwrap();
        std::process::exit(1);

    }

    //println!("Files found matching the pattern: \n{:#?}", filenames(&pattern[..]));

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

            println!("{} =====> {}", &path.to_string_lossy().magenta(), new_name.yellow());

            count += 1;
        }
        

    }

    println!("{} files found matching pattern, {} files renamed.", filenames.len(), count);

    Ok(())
    
}
