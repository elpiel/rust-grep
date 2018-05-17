#[macro_use] extern crate lazy_static;
extern crate regex;
use std::io::{self, BufRead};
use std::env;
use regex::{ Regex };

fn main() {
    let stdin = io::stdin();
//    let mut regex = String::from("line");
//    let file = String::new();
    let colorful_output = true;
    let mut args = env::args();

    let mut regex = match args.nth(1) {
        Some(regex) => regex,
        None => panic!("Please provide an expression")
    };

    println!("Running regex: {}", regex);

//    let file = match args.nth(1) {
//        Some(file)
//    }
//    for arguments in env::args() {
//
//    }
    let mut lines_count = 0;
    let mut match_count = 0;
    for line in stdin.lock().lines() {
        let line_str = line.unwrap();
        lines_count += 1;

        if is_a_match(&mut regex, &line_str) {
            match_count += 1;
            println!("match num: {} line: {}", &match_count, line_str);
        }
    }

    println!("total lines {} total matched lines {}", lines_count, match_count);
}

fn is_a_match(regex: &String, line: &String) -> bool {
//    lazy_static! {
//        static ref RE: Regex = Regex::new(regex.clone()).unwrap();
//    }
    let re = Regex::new(regex).unwrap();

    re.is_match(line.as_str())
}
