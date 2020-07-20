//! <https://codeforces.com/problemset/problem/1352/A>
//!
//! | CP | SP | CT | ST |
//! | --- | --- | --- | --- |
//! | Y | Y | Y | N |

use std::io::{self, BufRead};

fn decompose_as_round_numbers(num: u32) -> Vec<u32> {
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
    round_numbers
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
        let round_numbers = decompose_as_round_numbers(input);
        println!("{:?}", round_numbers.len());
        for round_number in round_numbers.into_iter() {
            print!("{:?} ", round_number);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_testcase() {
        assert_eq!(decompose_as_round_numbers(5009u32), vec![9u32, 5000]);
        assert_eq!(decompose_as_round_numbers(7u32), vec![7u32]);
        assert_eq!(
            decompose_as_round_numbers(9876u32),
            vec![6u32, 70, 800, 9000]
        );
        assert_eq!(decompose_as_round_numbers(10000u32), vec![10000u32]);
        assert_eq!(decompose_as_round_numbers(10u32), vec![10u32]);
    }
}
