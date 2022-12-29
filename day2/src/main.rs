use std::{env, fs};

const WIN_SCORE : u32   = 6;
const DRAW_SCORE : u32  = 3;
const LOOSE_SCORE : u32 = 0;

fn find_shape(s1: char, s2: char) -> (u32, char) {
    return match s2 {
        'X' => (LOOSE_SCORE,    if s1 == 'A' {'C'} else if s1 == 'B' {'A'} else {'B'}),
        'Y' => (DRAW_SCORE,     s1),
        'Z' => (WIN_SCORE,      if s1 == 'A' {'B'} else if s1 == 'B' {'C'} else {'A'}),
         _  => (LOOSE_SCORE,    '-'),
    };
}


fn main() {
    
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    
    let content = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let mut score = 0;

    for line in content.lines() {
        if line.is_empty() {continue;}

        let mut shapes : Vec<char> = line.trim().chars().collect();
        shapes.remove(1);
        let s1 = shapes[0];
        let (win_score, s2) = find_shape(s1, shapes[1]);

        score +=  win_score + if s2 == 'A' {1} else if s2 == 'B' {2} else {3};
    }
    println!("score = {score}");

}

