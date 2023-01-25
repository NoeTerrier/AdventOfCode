use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect() ;
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Unable to read file");

    let mut line_it = content.lines();

    let mut crates : Vec<Vec<char>> = Vec::new();
    for _i in 0..=8 {
        crates.push(Vec::new());
    }

    for line in line_it.by_ref().take(8) {
        for (i, c) in parse_crates(line).into_iter().enumerate() {
            if c == ' ' { continue; }
            match crates.get_mut(i) {
                Some(column) => column.push(c),
                None => panic!("panic"),
            }
        }
    }

    crates.iter_mut().for_each(|c| c.reverse());

    for line in line_it.by_ref().skip(2) {
        let (quantity, from, to) = parse_move(line);

        if quantity != 0 {
            // PART 1
            // effect_move(&mut crates, quantity, from-1, to-1);

            // PART 2
            effect_move_v2(&mut crates, quantity, from-1, to-1);
        }
    }

    crates.iter_mut().for_each(|c| print!("{}", c.pop().unwrap_or(' ')));
    println!();
    
}

fn parse_crates(line: &str) -> Vec<char> {

    let mut crates = Vec::new();

    for (i, c) in line.chars().enumerate() {
        if i % 4 == 1 {
            crates.push(c);
        }
    }
    crates
}

fn parse_move(line: &str) -> (usize, usize, usize) {

    let m = line
        .trim()
        .replace("move ", "")
        .replace(" from ", " ")
        .replace(" to ", " ")
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    match m[..] {
        [quantity, from, to] => (quantity, from, to),
        _ => (0,0,0),
    }
}

// PART 1
// fn effect_move(crates: &mut Vec<Vec<char>>, quantity: usize, from: usize, to: usize) {

//     for _ in 0..quantity {
//         let from_column = crates.get_mut(from).expect("column index out of bound");
        
//         let c = match from_column.pop() {
//             Some(c) => c,
//             None => return,
//         };
        
//         crates.get_mut(to)
//             .expect("column index out of bound")
//             .push(c);
//     }
// }


// PART 2
fn effect_move_v2(crates: &mut Vec<Vec<char>>, quantity: usize, from: usize, to: usize) {

    let mut temp = Vec::new();

    let from_column = crates.get_mut(from).expect("column index out of bound");
    let offset = from_column.len() - quantity;

    for _ in 0..quantity {
        temp.push(from_column.remove(offset));
    }
    
    crates.get_mut(to)
        .expect("column index out of bound")
        .append(&mut temp)
}
