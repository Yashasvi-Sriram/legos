// TODO: use ordered set impl instead of assuming vec is a set
fn power_set_of(index: usize, size: usize, parent: &Vec<usize>) -> Vec<Vec<usize>> {
    if index == size {
        return vec![parent.clone()];
    }

    let left = {
        let mut clone = parent.clone();
        clone.push(index);
        clone
    };
    let left_subsets = power_set_of(index + 1, size, &left);

    let right = parent.clone();
    let right_subsets = power_set_of(index + 1, size, &right);

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
            inventory
                .iter()
                .map(|item| item.weight)
                .collect::<Vec<usize>>(),
            inventory
                .iter()
                .map(|item| item.value)
                .collect::<Vec<i32>>()
        );
    }
    // Power set of indices
    let all_combinations = power_set_of(0usize, inventory.len(), &vec![]);
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
            .collect::<Vec<&str>>(),
        all_combinations[highest_value_index]
            .iter()
            .map(|index| inventory[*index].weight)
            .collect::<Vec<usize>>(),
        all_combinations[highest_value_index]
            .iter()
            .map(|index| inventory[*index].weight)
            .sum::<usize>(),
        all_combinations[highest_value_index]
            .iter()
            .map(|index| inventory[*index].value)
            .collect::<Vec<i32>>(),
        highest_value
        );
    }
    all_combinations[highest_value_index].clone()
}
