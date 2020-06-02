// https://codeforces.com/contest/4/problem/A

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .unwrap();

    // 1 <= kilos <= 100
    let kilos = buffer
                    .trim()
                    .parse::<i32>()
                    .unwrap();

    // Odd => NO
    if kilos % 2 == 1 {
        println!("NO");
        return Ok(());
    }

    // Exception
    if kilos < 3 {
        println!("NO");
        return Ok(());
    }

    println!("YES");
    return Ok(());
}
