use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution(stacks: &mut Vec<Vec<char>>, re: &Regex) {
    let input_file = File::open("./move_operations.txt").expect("Cannot open input file.");
    let file_reader = BufReader::new(&input_file);

    for l in file_reader.lines() {
        if let Ok(line) = l {
            if line.len() != 0 {
                let mut op_values: Vec<i32> = Vec::new();
                for cap in re.captures_iter(&line) {
                    match cap.get(0) {
                        Some(caps) => op_values.push(caps.as_str().parse().unwrap()),
                        None => panic!("input is invalid"),
                    }
                }
                assert_eq!(op_values.len(), 3);
                let num_to_move = op_values[0];
                let origin = op_values[1] as usize;
                let destination = op_values[2] as usize;
                let mut temp_vec: Vec<char> = Vec::new();
                for _i in 1..=num_to_move {
                    let character = match stacks[origin].pop() {
                        Some(character) => character,
                        None => break,
                    };
                    temp_vec.push(character);
                }
                for _i in 0..temp_vec.len() {
                    let character = temp_vec.pop().unwrap();
                    stacks[destination].push(character);
                }
            }
        }
    }

    print!("A2 // After the rearrangement procedure completes, the crates that end up on top of each stack are ");
    for i in 1..stacks.len() {
        match stacks[i].pop() {
            Some(character) => print!("{}", character),
            None => print!(" "),
        }
    }
    println!();
}
