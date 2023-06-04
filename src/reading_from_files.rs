use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn file_read_manually() {
    // Creates a File object that
    // requires a path argument
    // and error handling if the file
    // does not exist. This program
    // crashes if a readme.md is not
    // present.
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);
    // Reuses a single String
    // object over the lifetime
    // of the program
    let mut line = String::new();
    // Because reading from disk can fail,
    // we need to explicitly handle this. In
    // our case, errors crash the program.
    let len = reader.read_line(&mut line).unwrap();
    if len == 0 {
        return;
    }
    println!("{} ({} bytes long)", line, len);
    // Shrinks the String back to length 0,
    // preventing lines from persisting
    // into the following ones
    line.truncate(0);
}

pub fn file_read_buf_reader() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

    // A subtle behavior change
    // occurs here. BufReader::lines()
    // removes the trailing newline
    // character from each line.
    for line_ in reader.lines() {
        // Unwraps the Result, but
        // at the risk of crashing the
        // program if an error occurs
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}

pub fn read_lines_from_file() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();
    println!("THE FILE = {:?}", input);
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        // line is a String, but
        // re.find() takes an &str
        // as an argument.
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
