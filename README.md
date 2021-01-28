# fspaces
## A fast command-line tool for removing unsafe characters from filenames

White spaces in filenames are a pain in the ass when working with the command line. fspaces is a tool that 
renames any number of files that match some provided pattern. It is written in Rust and optimized for speed.

## Usage

fspaces takes the file name as the first argument, and it accespts familiar wildcards such as "*"
```
fspaces "*.pdf"
```
