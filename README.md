# fspaces
## A fast command-line tool for removing unsafe characters from filenames

White spaces in filenames are a pain in the ass when working with the command line. fspaces is a tool that 
renames any number of files that match some provided pattern. It is written in Rust and optimized for speed.

## Usage

fspaces takes the file name as the first argument, and it accespts familiar wildcards such as "*"
```
fspaces "*.pdf"
```
Comparing it to a bash script that uses the ```mv``` command, fspaces renamed 50 pdf files ten times faster.

Bash script execution time:
```Executed in   69.74 millis    fish           external 
   usr time   55.57 millis  690.00 micros   54.88 millis 
   sys time   17.50 millis  406.00 micros   17.09 millis 
```
fspaces execution time:
```Executed in    5.22 millis    fish           external 
   usr time    5.21 millis  366.00 micros    4.84 millis 
   sys time    0.21 millis  208.00 micros    0.00 millis 
```
