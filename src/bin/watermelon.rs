//! <https://codeforces.com/contest/4/problem/A>

use std::io::{self, Read};

fn can_split_into_even_parts(num: u32) -> bool {
    // Odd => NO
    if num % 2 == 1 {
        return false;
    }

    // Exception
    if num < 3 {
        return false;
    }

    return true;
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    // 1 <= kilos <= 100
    let kilos = buffer.trim().parse::<u32>().unwrap();

    if can_split_into_even_parts(kilos) {
        println!("YES");
    } else {
        println!("NO");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use legos_test_tools::proof::{BigO, ComplexityProof, CorrectnessProof};
    use legos_test_tools::test_suite;

    test_suite!(one_to_nine, ten_to_nineteen, twenty_to_twentynine);

    fn cp() -> CorrectnessProof {
        CorrectnessProof::Inferred
    }

    fn tp() -> ComplexityProof {
        ComplexityProof::Inferred(BigO::C)
    }

    fn sp() -> ComplexityProof {
        ComplexityProof::Inferred(BigO::C)
    }

    fn tt() {
        // TODO
    }

    fn one_to_nine() {
        assert_eq!(can_split_into_even_parts(1u32), false);
        assert_eq!(can_split_into_even_parts(2u32), false);
        assert_eq!(can_split_into_even_parts(3u32), false);
        assert_eq!(can_split_into_even_parts(5u32), false);
        assert_eq!(can_split_into_even_parts(7u32), false);
        assert_eq!(can_split_into_even_parts(9u32), false);

        assert_eq!(can_split_into_even_parts(4u32), true);
        assert_eq!(can_split_into_even_parts(6u32), true);
        assert_eq!(can_split_into_even_parts(8u32), true);
    }

    fn ten_to_nineteen() {
        assert_eq!(can_split_into_even_parts(11u32), false);
        assert_eq!(can_split_into_even_parts(13u32), false);
        assert_eq!(can_split_into_even_parts(15u32), false);
        assert_eq!(can_split_into_even_parts(17u32), false);
        assert_eq!(can_split_into_even_parts(19u32), false);

        assert_eq!(can_split_into_even_parts(10u32), true);
        assert_eq!(can_split_into_even_parts(12u32), true);
        assert_eq!(can_split_into_even_parts(14u32), true);
        assert_eq!(can_split_into_even_parts(16u32), true);
        assert_eq!(can_split_into_even_parts(18u32), true);
    }

    fn twenty_to_twentynine() {
        assert_eq!(can_split_into_even_parts(21u32), false);
        assert_eq!(can_split_into_even_parts(23u32), false);
        assert_eq!(can_split_into_even_parts(25u32), false);
        assert_eq!(can_split_into_even_parts(27u32), false);
        assert_eq!(can_split_into_even_parts(29u32), false);

        assert_eq!(can_split_into_even_parts(20u32), true);
        assert_eq!(can_split_into_even_parts(22u32), true);
        assert_eq!(can_split_into_even_parts(24u32), true);
        assert_eq!(can_split_into_even_parts(26u32), true);
        assert_eq!(can_split_into_even_parts(28u32), true);
    }
}
