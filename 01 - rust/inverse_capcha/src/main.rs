use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let files: Vec<String> = env::args().collect(); 
    for file in &files[1..] {
        println!("Solving {file}.");
        println!("Part 1: {}", part1(file));
        println!("Part 2: {}", part2(&file));
    }
}

fn part1(file: &String) -> u32 {
    let mut file = File::open(file).expect("Error opening file.");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Error reading file");

    let mut digits: Vec<u32> = content.trim().chars().map(|c| c.to_digit(10).expect("Error parsing digit")).collect();
    digits.push(digits[0]);

    let mut sum: u32 = 0;
    for i in 0..digits.len() - 1 {
        if digits[i] == digits[i + 1] {
            sum += digits[i];
        }
    }

    sum
}

fn part2(file: &String) -> u32 {
    let mut file = File::open(file).expect("Error opening file.");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Error reading file");
    let digits: Vec<u32> = content.trim().chars().map(|c| c.to_digit(10).expect("Error parsing digit")).collect();

    let mut sum: u32 = 0;
    let len = digits.len();
    let distance = len / 2;
    for i in 0..digits.len() {
        let next_i = (i + distance) % len;
        if digits[i] == digits[next_i] {
            sum += digits[i]
        }
    }

    sum
}

