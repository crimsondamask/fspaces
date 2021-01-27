extern crate glob;
extern crate colored;
use colored::Colorize;
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
        writeln!(std::io::stderr(), "Error occured while parsing arguments")
            .unwrap();
        writeln!(std::io::stderr(),
                "USAGE:\nfspaces [PATTERN]").unwrap();
        std::process::exit(1);
    }

    let pattern = &args[1];

    //println!("Files found matching the pattern: \n{:#?}", filenames(&pattern[..]));

    rename(filenames(&pattern[..]))
        .expect("Couldn't rename files");
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

    println!("{:#?}", filenames);

    for path in filenames.iter() {

        if path.to_string_lossy().contains(" ") || path.to_string_lossy().contains("&") {

            let new_name = str::replace(&path.to_string_lossy(), " ", "_");

            fs::rename(path, &new_name)?;

            println!("{} =====> {}", &path.to_string_lossy().magenta(), new_name.yellow());

            count += 1;
        }
        

    }

    println!("{} files found matching pattern, {} files renamed.", filenames.len(), count);

    Ok(())
    
}
