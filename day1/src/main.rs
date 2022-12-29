use std::fs;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string("./input")
        .expect("Should have been able to read the file");

    
    let mut cal_vect : Vec<u32> = Vec::new();

    let mut curr_sum = 0;

    for line in contents.lines() {
    
        if line.is_empty() {
            cal_vect.push(curr_sum);
            curr_sum = 0;
        } else {
            curr_sum += match line.trim().parse::<u32>() {
                Ok(value) => value,
                Err(_) => continue,
            };
        }
    }

    cal_vect.sort_by(|a, b| b.cmp(a));

    let first: &u32 = &cal_vect[0];
    println!("Max1: {first}");

    let second: &u32 = &cal_vect[1];
    println!("Max2: {second}");

    let third: &u32 = &cal_vect[2];
    println!("Max3: {third}");

    println!("Sum : {}", first+second+third);

}

