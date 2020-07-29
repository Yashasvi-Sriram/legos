//! - C: implicit
//! - T: O(2^ordered_set.len()) implicit
//! - S: O(2^ordered_set.len())
//!     - 2^N subsets + 2 * (2^(N-1) + 2^(N-2) + ... 2^1) clones in worst case
//!     - 2^N + 2^N + ... 2^1
//!     - O(2^N)

fn pack_recur(
    index: usize,
    size: usize,
    parent: &Vec<usize>,
    capacity: usize,
    inventory: &Vec<super::KnapsackItem>,
) -> Option<Vec<usize>> {
    // Pruning
    if parent
        .iter()
        .map(|index| inventory[*index].weight)
        .sum::<usize>()
        > capacity
    {
        return None;
    }
    if index == size {
        return Some(parent.clone());
    }

    let left = {
        let mut clone = parent.clone();
        clone.push(index);
        clone
    };
    let left_best_pack = pack_recur(index + 1, size, &left, capacity, inventory);

    let right = parent.clone();
    let right_best_pack = pack_recur(index + 1, size, &right, capacity, inventory);

    // Implicit comparision
    match (left_best_pack, right_best_pack) {
        (Some(left), Some(right)) => {
            let left_value: i32 = left.iter().map(|index| inventory[*index].value).sum();
            let right_value: i32 = right.iter().map(|index| inventory[*index].value).sum();
            if right_value > left_value {
                return Some(right);
            } else {
                return Some(left);
            }
        }
        (Some(left), None) => return Some(left),
        (None, Some(right)) => return Some(right),
        (None, None) => return None,
    }
}

pub fn pack(capacity: usize, inventory: &Vec<super::KnapsackItem>, debug: bool) -> Vec<usize> {
    if debug {
        println!(
            "#elements = {}\nweights = {:?}\nvalues = {:?}",
            inventory.len(),
            inventory.iter().map(|item| item.weight).collect::<Vec<_>>(),
            inventory.iter().map(|item| item.value).collect::<Vec<_>>()
        );
    }
    // Power set of indices
    let best_combination =
        pack_recur(0usize, inventory.len(), &vec![], capacity, inventory).unwrap();
    if debug {
        println!(
        "highest value combination =\n\t{:?}\n\t{:?}\nweights\t{:?}\nsum\t{}\nvalues\t{:?}\nsum\t{}",
        best_combination,
        best_combination
            .iter()
            .map(|index| inventory[*index].name)
            .collect::<Vec<_>>(),
        best_combination
            .iter()
            .map(|index| inventory[*index].weight)
            .collect::<Vec<_>>(),
        best_combination
            .iter()
            .map(|index| inventory[*index].weight)
            .sum::<usize>(),
        best_combination
            .iter()
            .map(|index| inventory[*index].value)
            .collect::<Vec<_>>(),
        best_combination
            .iter()
            .map(|index| inventory[*index].value)
            .sum::<i32>()
        );
    }
    best_combination.clone()
}

#[cfg(test)]
mod tests {
    use crate::data::Input;

    #[test]
    fn full_inventory() {
        let (capacity, inventory) = Input::full();
        let best_combination = super::pack(capacity, &inventory, false);
        assert_eq!(
            vec![0, 1, 2, 3, 4, 6, 10, 15, 16, 17, 18, 20],
            best_combination
        );
    }
}
