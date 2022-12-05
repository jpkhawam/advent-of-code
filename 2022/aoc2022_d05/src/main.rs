/*
    Advent of Code 2022 / Day 5: https://adventofcode.com/2022/day/5
    --- Day 5: Supply Stacks ---
    Q1: After the rearrangement procedure completes, what crate ends up on top of each stack?
    Q2: After the rearrangement procedure completes, what crate ends up on top of each stack?
*/

use regex::Regex;

mod solution_one;
mod solution_two;

fn main() {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    // is this cheating?
    stacks.push(Vec::new());
    stacks.push(Vec::from(['B', 'L', 'D', 'T', 'W', 'C', 'F', 'M']));
    stacks.push(Vec::from(['N', 'B', 'L']));
    stacks.push(Vec::from(['J', 'C', 'H', 'T', 'L', 'V']));
    stacks.push(Vec::from(['S', 'P', 'J', 'W']));
    stacks.push(Vec::from(['Z', 'S', 'C', 'F', 'T', 'L', 'R']));
    stacks.push(Vec::from(['W', 'D', 'G', 'B', 'H', 'N', 'Z']));
    stacks.push(Vec::from(['F', 'M', 'S', 'P', 'V', 'G', 'C', 'N']));
    stacks.push(Vec::from(['W', 'Q', 'R', 'J', 'F', 'V', 'C', 'Z']));
    stacks.push(Vec::from(['R', 'P', 'M', 'L', 'H']));

    let re = Regex::new(r"(\d{2})|(\d)").unwrap();

    solution_one::solution(&mut stacks.clone(), &re);
    solution_two::solution(&mut stacks, &re);
}
