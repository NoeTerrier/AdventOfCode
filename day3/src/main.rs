use std::{env, fs};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect() ;
    let filename = args[1];

    let content = fs::read_to_string(filename)
        .expect("Unable to read file");

    for line in content.lines() {
        let line = line.trim();
    }

}
    
fn find_shared(line: &str) {
    let lenght = line.len();
    let mid = lenght/2;

    let chars = line.chars();

    for c in chars {
        chars.con
    }

}
