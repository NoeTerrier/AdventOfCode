use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect() ;
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Unable to read file");

    let lines = content.lines();

    let mut count_include = 0;
    let mut count_overlap = 0;

    for line in lines {
        let groups : Vec<&str> = line.trim().split(',').collect();

        let mut range_1 = (0, 0);
        let mut range_2 = (0, 0);
        let mut first_exists = false;

        for range in groups {
            if !first_exists {
                range_1 = parse_range(range);
                first_exists = true;
            } else {
                range_2 = parse_range(range);
            }
            
            count_include += if fully_include(range_1, range_2) {1} else {0}; 
            count_overlap += if overlapp(range_1, range_2) {1} else {0};

        }
    }
    
    println!("count fully included : {count_include}");
    println!("count overlaped : {count_overlap}");
}
    

// PART 1
fn fully_include(range_1: (u32, u32), range_2: (u32, u32)) -> bool {
    range_1.0 <= range_2.0 && range_2.1 <= range_1.1 || range_2.0 <= range_1.0 && range_1.1 <= range_2.1
}

// PART 2
fn overlapp(range_1: (u32, u32), range_2: (u32, u32)) -> bool {
    range_2.0 <= range_1.1 && range_1.1 <= range_2.1 || range_1.0 <= range_2.1 && range_2.1 <= range_1.1
}

fn parse_range(s: &str) -> (u32, u32) {
    let range : Vec<u32> = s.split('-').map(|s| s.parse::<u32>().unwrap()).collect();
    match range[..] {
        [from, to] => (from, to),
        _ => panic!("vector does not contain exactly 2 elements"),
    }
}
