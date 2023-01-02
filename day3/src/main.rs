use std::{env, fs, collections::{HashSet, hash_map::RandomState}};

fn main() {

    let args: Vec<String> = env::args().collect() ;
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Unable to read file");

    let mut priority_sum: u32 = 0;

    // FIRST PART -----------------------------------------
    // for line in content.lines() {
    //     let line = line.trim();

    //     let shared_char = match find_shared(line) {
    //         Ok(c) => c,
    //         Err(_) => '_',
    //     };

    //     priority_sum += get_priority(shared_char);
    // }
    // ----------------------------------------------------


    // SECOND PART ----------------------------------------
    let lines = content.lines();
    let mut group_line = [""; 3];

    for (i, line) in lines.enumerate() {

        group_line[i%3] = line;

        if i % 3 == 2 {
            let group: Vec<&str>  = Vec::from(group_line);
    
            let badge = match find_badge(group) {
                Ok(c) => c,
                Err(_) => '_',
            };

            priority_sum += get_priority(badge);
        }
    
    }
    // ---------------------------------------------------


    println!("{}", priority_sum);

}

// FIRST PART -----------------------------------------
// &str is assumed to have a even length
// fn find_shared(line: &str) -> Result<char, &'static str> {
//     let mid = line.len()/2; 

//     let fist_half: HashSet<char> = HashSet::from_iter(line.chars().take(mid));
//     let second_half: HashSet<char> = HashSet::from_iter(line.chars().skip(mid));
//     let intersection = fist_half.intersection(&second_half);

//     for c in intersection { //if any, return the first one found
//         return Ok(*c);
//     }
    
//     Err("Not found")
// }
// ----------------------------------------------------


// SECOND PART -----------------------------------------
fn find_badge(group_line: Vec<&str>) -> Result<char, &'static str> {

    let mut lines_sets = group_line.iter()
            .map(|s| HashSet::from_iter(s.chars()));
    
    let first_line: HashSet<char, RandomState>= match lines_sets.next() {
        Some(hs) => hs,
        None => return Err("empty group"),
    };

    let cross_set = lines_sets.fold(first_line, |mut acc, hs| {
        if !hs.is_empty() {
            acc.retain(|a| hs.contains(a));
        }
        acc
    });

    for c in cross_set {
        return Ok(c);
    }

    Err("Not found")
}
// ----------------------------------------------------

// Part 1 and 2

fn get_priority(c: char) -> u32 {
    let priority = if c >= 'A' && c <= 'Z' {
        c as u32 - 'A' as u32 + 27   //base + offset = 27 + 1 
    } else if c >= 'a' && c <= 'z' {
        c as u32 - 'a' as u32 + 1    //base + offset = 0  + 1
    } else {
        0
    };

    return priority;
}
