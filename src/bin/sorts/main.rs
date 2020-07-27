//! link TODO

/// - C: TODO
/// - T: TODO
/// - S: TODO
pub fn merge_sort_recursive(original: &[i32]) -> Vec<i32> {
    if original.len() == 1 {
        return vec![original[0]];
    }

    let middle = original.len() / 2;
    let sorted_left = merge_sort_recursive(&original[..middle]);
    let sorted_right = merge_sort_recursive(&original[middle..]);

    let mut l = 0usize;
    let mut r = 0usize;
    let mut local_merge = vec![];
    loop {
        if l == sorted_left.len() && r == sorted_right.len() {
            break;
        } else if l < sorted_left.len() && r == sorted_right.len() {
            local_merge.push(sorted_left[l]);
            l += 1;
        } else if l == sorted_left.len() && r < sorted_right.len() {
            local_merge.push(sorted_right[r]);
            r += 1;
        } else {
            let left = sorted_left[l];
            let right = sorted_right[r];
            if left < right {
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

/// - C: TODO
/// - T: TODO
/// - S: TODO
fn merge_sort_iterative(original: &[i32]) -> Vec<i32> {
    use core::ops::Range;

    #[derive(Clone)]
    struct ElementState {
        val: i32,
        is_visited: bool,
    }

    impl ElementState {
        fn set(&mut self, val: i32) {
            self.val = val;
            self.is_visited = true;
        }
    }

    impl Default for ElementState {
        fn default() -> ElementState {
            ElementState {
                val: 0,
                is_visited: false,
            }
        }
    }
    let mut sorted = vec![ElementState::default(); original.len()];
    // Stack collects ranges.
    // If original.len() == 8, at max it shall have
    // [ 0..8, 0..4, 4..8, 4..6, 6..8, 6..7, 7..8 ]
    // Therefore the following capcity
    let mut stack = Vec::<Range<usize>>::with_capacity(
        1 + 2 * ((original.len() as f32).log2().ceil() as usize),
    );
    stack.push(0..original.len());
    // Used for merging two sorted arrays. Avoids allocating space log(n) times.
    // Required to allocated because cannot do merge with O(1) space and O(n) time.
    let mut temporary_merge_space = Vec::<i32>::with_capacity(original.len());
    while let Some(current_range) = stack.pop() {
        if current_range.len() == 1 {
            let idx = current_range.start;
            sorted[idx].set(original[idx]);
            continue;
        }
        if sorted[current_range.start].is_visited {
            // Range start is visited <=> Range is visited
            // The two halves are sorted, merge them
            let Range { start, end } = current_range;
            let middle = start + current_range.len() / 2;

            temporary_merge_space.clear();
            let mut l = start;
            let mut r = middle;
            loop {
                if l == middle && r == end {
                    break;
                } else if l < middle && r == end {
                    temporary_merge_space.push(sorted[l].val);
                    l += 1;
                } else if l == middle && r < end {
                    temporary_merge_space.push(sorted[r].val);
                    r += 1;
                } else {
                    let left = sorted[l].val;
                    let right = sorted[r].val;
                    if left < right {
                        temporary_merge_space.push(left);
                        l += 1;
                    } else {
                        temporary_merge_space.push(right);
                        r += 1;
                    }
                }
            }
            for (i, idx) in current_range.enumerate() {
                sorted[idx].val = temporary_merge_space[i];
            }
        } else {
            // Range start is unvisited <=> Range is unvisited
            // Split into two
            let Range { start, end } = current_range;
            let middle = start + current_range.len() / 2;
            stack.push(current_range);
            stack.push(start..middle);
            stack.push(middle..end);
        }
    }
    sorted.into_iter().map(|x| x.val).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterative() {
        // TODO: improve
        assert_eq!(
            merge_sort_iterative(vec![7, 5, 3, 2].as_slice()),
            vec![2, 3, 5, 7]
        );
        assert_eq!(
            merge_sort_iterative(vec![-7, -5, 3, 2].as_slice()),
            vec![-7, -5, 2, 3]
        );
    }

    #[test]
    fn recursive() {
        // TODO: improve
        assert_eq!(
            merge_sort_recursive(vec![7, 5, 3, 2].as_slice()),
            vec![2, 3, 5, 7]
        );
        assert_eq!(
            merge_sort_recursive(vec![-7, -5, 3, 2].as_slice()),
            vec![-7, -5, 2, 3]
        );
    }
}
