fn power_set_of(
    index: usize,
    size: usize,
    parent: &Vec<usize>,
    capacity: usize,
    inventory: &Vec<super::KnapsackItem>,
) -> Vec<Vec<usize>> {
    // Pruning
    if parent
        .iter()
        .map(|index| inventory[*index].weight)
        .sum::<usize>()
        > capacity
        || index == size
    {
        return vec![parent.clone()];
    }

    let left = {
        let mut clone = parent.clone();
        clone.push(index);
        clone
    };
    let left_subsets = power_set_of(index + 1, size, &left, capacity, inventory);

    let right = parent.clone();
    let right_subsets = power_set_of(index + 1, size, &right, capacity, inventory);

    return left_subsets
        .into_iter()
        .chain(right_subsets.into_iter())
        .collect();
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
    let all_combinations = power_set_of(0usize, inventory.len(), &vec![], capacity, inventory);
    if debug {
        println!("total #combinations = {}", all_combinations.len());
    }
    // Highest value combination
    let mut highest_value_index = 0;
    let mut highest_value = i32::min_value();
    for (cmb_i, cmb) in all_combinations.iter().enumerate() {
        let total_weight: usize = cmb.iter().map(|index| inventory[*index].weight).sum();
        let total_value: i32 = cmb.iter().map(|index| inventory[*index].value).sum();
        if total_weight <= capacity && total_value > highest_value {
            highest_value = total_value;
            highest_value_index = cmb_i;
        }
    }
    if debug {
        println!(
        "highest value combination =\n\t{:?}\n\t{:?}\nweights\t{:?}\nsum\t{}\nvalues\t{:?}\nsum\t{}",
        all_combinations[highest_value_index],
        all_combinations[highest_value_index]
            .iter()
            .map(|index| inventory[*index].name)
            .collect::<Vec<_>>(),
        all_combinations[highest_value_index]
            .iter()
            .map(|index| inventory[*index].weight)
            .collect::<Vec<_>>(),
        all_combinations[highest_value_index]
            .iter()
            .map(|index| inventory[*index].weight)
            .sum::<usize>(),
        all_combinations[highest_value_index]
            .iter()
            .map(|index| inventory[*index].value)
            .collect::<Vec<_>>(),
        highest_value
        );
    }
    all_combinations[highest_value_index].clone()
}

#[cfg(test)]
mod tests {
    use crate::data::Input;
    use legos_test_tools::proof::{BigO, ComplexityProof, CorrectnessProof};
    use legos_test_tools::test_suite;

    test_suite!(full_inventory,);

    fn cp() -> CorrectnessProof {
        CorrectnessProof::Inferred
    }

    fn tp() -> ComplexityProof {
        ComplexityProof::Inferred(BigO::TwoToN)
    }

    fn sp() -> ComplexityProof {
        ComplexityProof::Because(
            "2^N subsets + max N clones at a time == 2^N + N. Hence proved".to_string(),
            BigO::TwoToN,
        )
    }

    fn tt() {}

    fn full_inventory() {
        let (capacity, inventory) = Input::full();
        let best_combination = super::pack(capacity, &inventory, false);
        assert_eq!(
            vec![0, 1, 2, 3, 4, 6, 10, 15, 16, 17, 18, 20],
            best_combination
        );
    }
}
