use std::{env, fs, collections::HashSet};

// PART 1
// const MARKER_SIZE : usize = 4;

// PART 2
const MARKER_SIZE : usize = 14;

fn main() {
    let args: Vec<String> = env::args().collect() ;
    let filename = &args[1];

    let content = fs::read_to_string(filename).expect("Unable to read file");

    let mut marker : [char; MARKER_SIZE] = content
        .chars()
        .take(MARKER_SIZE)
        .collect::<Vec<char>>()
        .try_into()
        .unwrap();

    for (i, c) in content.chars().skip(MARKER_SIZE).enumerate() {

        if HashSet::from(marker).len() == MARKER_SIZE {
            println!("start ok packet : {}", i+MARKER_SIZE);
            return;
        }
        marker[i % MARKER_SIZE] = c; 
    }
}
