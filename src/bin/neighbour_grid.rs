//! <https://codeforces.com/problemset/problem/1375/B>

/// # Proof of correctness
/// - The loop statement in good_version() breaks <==> the grid is good
/// - Why greedy method works?
///     - TODO: find proof and time complexity
mod grid {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Grid {
        num_rows: usize,
        num_cols: usize,
        row_major_values: Vec<u32>,
    }

    impl Grid {
        pub fn new(num_rows: usize, num_cols: usize, row_major_values: &Vec<u32>) -> Option<Grid> {
            if num_rows * num_cols == row_major_values.len() {
                Some(Grid {
                    num_rows: num_rows,
                    num_cols: num_cols,
                    row_major_values: row_major_values.to_vec(),
                })
            } else {
                None
            }
        }

        pub fn good_version(&self) -> Option<Grid> {
            fn situation_at(grid: &Grid, r: usize, c: usize) -> (u32, u32) {
                // If val is 0, no further check needed => early return with zeros
                if grid[(r, c)] == 0 {
                    return (0, 0);
                }
                let mut positive_neighbour_count = 0u32;
                if r > 0 && grid[(r - 1, c)] > 0 {
                    positive_neighbour_count += 1;
                }
                if r < grid.num_rows - 1 && grid[(r + 1, c)] > 0 {
                    positive_neighbour_count += 1;
                }
                if c > 0 && grid[(r, c - 1)] > 0 {
                    positive_neighbour_count += 1;
                }
                if c < grid.num_cols - 1 && grid[(r, c + 1)] > 0 {
                    positive_neighbour_count += 1;
                }
                (grid[(r, c)], positive_neighbour_count)
            }

            fn try_increment_neighbours(
                grid: &mut Grid,
                r: usize,
                c: usize,
                remaining: u32,
            ) -> Option<u32> {
                let mut remaining = remaining;
                if remaining > 0 && r > 0 && grid[(r - 1, c)] == 0 {
                    grid[(r - 1, c)] += 1;
                    remaining -= 1;
                }
                if remaining > 0 && r < grid.num_rows - 1 && grid[(r + 1, c)] == 0 {
                    grid[(r + 1, c)] += 1;
                    remaining -= 1;
                }
                if remaining > 0 && c > 0 && grid[(r, c - 1)] == 0 {
                    grid[(r, c - 1)] += 1;
                    remaining -= 1;
                }
                if remaining > 0 && c < grid.num_cols - 1 && grid[(r, c + 1)] == 0 {
                    grid[(r, c + 1)] += 1;
                    remaining -= 1;
                }
                match remaining {
                    0 => Some(0),
                    _ => None,
                }
            }

            let mut good_version = self.clone();

            loop {
                let mut still_bad_version = false;
                for r in 0..good_version.num_rows {
                    for c in 0..good_version.num_cols {
                        let (val, num_positive_neighbours) = situation_at(&good_version, r, c);
                        if val == 0 {
                            continue;
                        } else if val < num_positive_neighbours {
                            good_version[(r, c)] += num_positive_neighbours - val;
                        } else if val > num_positive_neighbours {
                            try_increment_neighbours(
                                &mut good_version,
                                r,
                                c,
                                val - num_positive_neighbours,
                            )?;
                            still_bad_version = true;
                        }
                    }
                }
                if !still_bad_version {
                    break;
                }
            }

            Some(good_version)
        }
    }

    use std::ops::{Index, IndexMut};
    impl Index<(usize, usize)> for Grid {
        type Output = u32;

        fn index(&self, coordinate: (usize, usize)) -> &Self::Output {
            let (r, c) = coordinate;
            &self.row_major_values[r * self.num_cols + c]
        }
    }

    impl IndexMut<(usize, usize)> for Grid {
        fn index_mut(&mut self, coordinate: (usize, usize)) -> &mut Self::Output {
            let (r, c) = coordinate;
            &mut self.row_major_values[r * self.num_cols + c]
        }
    }

    use std::fmt;
    impl fmt::Display for Grid {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for i in 0..self.num_rows {
                for j in 0..self.num_cols {
                    if j == self.num_cols - 1 {
                        write!(f, "{}", self.row_major_values[i * self.num_cols + j])?;
                    } else {
                        write!(f, "{} ", self.row_major_values[i * self.num_cols + j])?;
                    }
                }
                if i < self.num_rows - 1 {
                    write!(f, "\n")?;
                }
            }
            write!(f, "")
        }
    }
}

fn main() {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let num_testcases = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    for _ in 0..num_testcases {
        let shape = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let (num_rows, num_cols) = (shape[0], shape[1]);
        let rows = (0..num_rows)
            .map(|_| {
                lines
                    .next()
                    .unwrap()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect::<Vec<u32>>();
        let grid = grid::Grid::new(num_rows, num_cols, &rows).unwrap();
        let good_grid = grid.good_version();
        match good_grid {
            Some(good_version) => {
                println!("YES");
                println!("{}", good_version);
            }
            None => println!("NO"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_testcases() {
        assert_eq!(
            grid::Grid::new(3, 4, &vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0])
                .unwrap()
                .good_version(),
            Some(grid::Grid::new(3, 4, &vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]).unwrap())
        );
        assert_eq!(
            grid::Grid::new(2, 2, &vec![3, 0, 0, 0])
                .unwrap()
                .good_version(),
            None
        );
        assert_eq!(
            grid::Grid::new(2, 2, &vec![0, 0, 0, 0])
                .unwrap()
                .good_version(),
            Some(grid::Grid::new(2, 2, &vec![0, 0, 0, 0]).unwrap())
        );
        assert_eq!(
            grid::Grid::new(2, 2, &vec![0, 0, 0, 0])
                .unwrap()
                .good_version(),
            Some(grid::Grid::new(2, 2, &vec![0, 0, 0, 0]).unwrap())
        );
        assert_eq!(
            grid::Grid::new(2, 3, &vec![0, 0, 0, 0, 4, 0])
                .unwrap()
                .good_version(),
            None
        );
        assert_eq!(
            grid::Grid::new(4, 4, &vec![0, 0, 0, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0])
                .unwrap()
                .good_version(),
            Some(
                grid::Grid::new(4, 4, &vec![0, 1, 0, 1, 0, 2, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0])
                    .unwrap()
            )
        );
    }

    #[test]
    fn some_of_my_testcases() {
        assert_eq!(
            grid::Grid::new(4, 3, &vec![0, 0, 0, 0, 4, 2, 0, 3, 0, 0, 0, 0])
                .unwrap()
                .good_version(),
            Some(grid::Grid::new(4, 3, &vec![0, 2, 2, 2, 4, 2, 2, 3, 0, 0, 1, 0]).unwrap())
        );
        assert_eq!(
            grid::Grid::new(1, 1, &vec![0]).unwrap().good_version(),
            Some(grid::Grid::new(1, 1, &vec![0]).unwrap())
        );
        assert_eq!(
            grid::Grid::new(1, 1, &vec![1]).unwrap().good_version(),
            None
        );
        assert_eq!(
            grid::Grid::new(1, 2, &vec![1, 0]).unwrap().good_version(),
            Some(grid::Grid::new(1, 2, &vec![1, 1]).unwrap())
        );
        assert_eq!(
            grid::Grid::new(1, 2, &vec![1, 2]).unwrap().good_version(),
            None
        );
    }
}
