// https://codeforces.com/problemset/problem/1352/A

use std::io::{self, BufRead};

fn decompose_as_round_numbers(num: u32) {
    let mut rem = num;
    let mut round_numbers: Vec<u32> = vec![];
    let mut place_value = 1;
    while rem > 0 {
        let digit = rem % 10;
        if digit > 0 {
            round_numbers.push(digit * place_value);
        }

        // Loop update
        place_value *= 10;
        rem /= 10;
    }
    println!("{:?}", round_numbers.len());
    for round_number in round_numbers.into_iter() {
        print!("{:?} ", round_number);
    }
    println!("");
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // 1 <= num_inputs <= 1e4
    let _num_inputs = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    // 1 <= input <= 1e4
    let mut inputs: Vec<u32> = vec![];
    for line in lines {
        inputs.push(line.unwrap().parse::<u32>().unwrap());
    }
    // println!("{:?}", inputs);
    for input in inputs.into_iter() {
        decompose_as_round_numbers(input);
    }
}

