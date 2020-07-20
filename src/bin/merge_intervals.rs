//! <https://leetcode.com/problems/merge-intervals/>

struct Solution;

impl Solution {
    pub fn merge_sort(list: &[Vec<i32>]) -> Vec<Vec<i32>> {
        if list.len() == 1 {
            return vec![list[0].clone()];
        }

        let middle = list.len() / 2;
        let sorted_left = Self::merge_sort(&list[..middle]);
        let sorted_right = Self::merge_sort(&list[middle..]);

        let mut l = 0usize;
        let mut r = 0usize;
        let mut local_merge = vec![];
        loop {
            if l == sorted_left.len() && r == sorted_right.len() {
                break;
            } else if l < sorted_left.len() && r == sorted_right.len() {
                local_merge.push(sorted_left[l].clone());
                l += 1;
            } else if l == sorted_left.len() && r < sorted_right.len() {
                local_merge.push(sorted_right[r].clone());
                r += 1;
            } else {
                let left = sorted_left[l].clone();
                let right = sorted_right[r].clone();
                if left[0] < right[0] {
                    local_merge.push(left);
                    l += 1;
                } else {
                    local_merge.push(right);
                    r += 1;
                }
            }
        }
        local_merge
    }

    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return vec![];
        }
        let sorted_intervals = Self::merge_sort(&intervals);
        // println!("{:?}", sorted_intervals);
        let mut merged_intervals = vec![];
        let mut n = 0usize;
        let mut k = 1usize;
        let mut start = sorted_intervals[n][0];
        let mut end = sorted_intervals[n][1];
        loop {
            if n + k >= sorted_intervals.len() {
                break;
            }
            if sorted_intervals[n + k][0] <= end {
                end = end.max(sorted_intervals[n + k][1]);
                k += 1;
            } else {
                merged_intervals.push(vec![start, end]);
                n += k;
                k = 1;
                start = sorted_intervals[n][0];
                end = sorted_intervals[n][1];
            }
        }
        merged_intervals.push(vec![start, end]);
        merged_intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcases() {
        assert_eq!(Solution::merge(vec![]).len(), 0);
        assert_eq!(Solution::merge(vec![vec![-1, 4]]), vec![vec![-1, 4]]);
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![3, 10], vec![10, 18]]),
            vec![vec![1, 18]]
        );
        assert_eq!(
            Solution::merge(vec![
                vec![1, 7],
                vec![3, 4],
                vec![4, 5],
                vec![9, 11],
                vec![-2, -1],
            ]),
            vec![vec![-2, -1], vec![1, 7], vec![9, 11]]
        );
        assert_eq!(
            Solution::merge(vec![
                vec![1, 3],
                vec![8, 10],
                vec![15, 18],
                vec![2, 6],
                vec![-2, -1],
            ]),
            vec![vec![-2, -1], vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }
}
