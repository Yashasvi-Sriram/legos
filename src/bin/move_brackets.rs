//! <https://codeforces.com/problemset/problem/1374/C>
use std::io::{self, BufRead};

/// # Proof of correctness
/// 1. An input with given constrains shall have a zero final sum.
/// 1. An input that is a regular bracket sequence (rbs), shall have a non-negative sum-from-start at any index. This serves as a verification method.
/// 1. To make non-rbs an rbs, one way is to somehow make all negative sum-from-start be non-negative.
/// 1. With the permitted operations, this can be done by doing, for each bracket with negative sum-from-start
///     - `(` => move to the beggining
///     - or
///     - `)` => move to the end
/// 1. With each move, the min sum-from-start increases by one
/// 1. Min number of moves to make min(sum-from-start) non-negative is abs(min(sum-from-start)).
/// 1. Hence proved.
fn min_moves_for_regular_bracket_sequence(brackets: &str) -> Option<u32> {
    let mut lowest_sum = 0i32;
    let mut sum = 0i32;
    for bracket in brackets.chars() {
        if bracket == '(' {
            sum += 1;
        } else if bracket == ')' {
            sum -= 1;
        } else {
            return None;
        }

        if sum < lowest_sum {
            lowest_sum = sum;
        }
    }
    Some(lowest_sum.abs() as u32)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let _num_testcases = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    for (i, line) in lines.enumerate() {
        if i % 2 == 1 {
            let ans = min_moves_for_regular_bracket_sequence(&line.unwrap()).unwrap();
            println!("{}", ans);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_testcases() {
        assert_eq!(min_moves_for_regular_bracket_sequence(")(").unwrap(), 1);
        assert_eq!(min_moves_for_regular_bracket_sequence("()()").unwrap(), 0);
        assert_eq!(
            min_moves_for_regular_bracket_sequence("())()()(").unwrap(),
            1
        );
        assert_eq!(
            min_moves_for_regular_bracket_sequence(")))((((())").unwrap(),
            3
        );
    }
}
