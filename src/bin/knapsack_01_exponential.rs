// FIXME: use ordered set impl instead of assuming vec is a set
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

fn main() {
    // Parameters
    // Example from https://rosettacode.org/wiki/Knapsack_problem/0-1#C
    let capacity = 400usize;
    let weights = vec![
        9usize, 13, 153, 50, 15, 68, 27, 39, 23, 52, 11, 32, 24, 48, 73, 42, 43, 22, 7, 18, 4, 30,
    ];
    let values = vec![
        150i32, 35, 200, 160, 60, 45, 60, 40, 30, 10, 70, 30, 15, 10, 40, 70, 75, 80, 20, 12, 50,
        10,
    ];
    assert_eq!(weights.len(), values.len());
    println!(
        "#elements = {}\nweights = {:?}\nvalues = {:?}",
        weights.len(),
        weights,
        values
    );
    // Power set
    let all_combinations = power_set_of(0usize, weights.len(), &vec![]);
    println!("total #combinations = {}", all_combinations.len());
    // Highest value combination
    let mut highest_value_index = 0;
    let mut highest_value = i32::min_value();
    for (cmb_i, cmb) in all_combinations.iter().enumerate() {
        let total_weight: usize = cmb
            .iter()
            .map(|index| weights[*index])
            .collect::<Vec<usize>>()
            .iter()
            .sum();
        let total_value: i32 = cmb
            .iter()
            .map(|index| values[*index])
            .collect::<Vec<i32>>()
            .iter()
            .sum();
        if total_weight <= capacity && total_value > highest_value {
            highest_value = total_value;
            highest_value_index = cmb_i;
        }
    }
    println!(
        "highest value combination =\n\t{:?}\nweights\t{:?}\nsum\t{}\nvalues\t{:?}\nsum\t{}",
        all_combinations[highest_value_index],
        all_combinations[highest_value_index]
            .iter()
            .map(|index| weights[*index])
            .collect::<Vec<usize>>(),
        all_combinations[highest_value_index]
            .iter()
            .map(|index| weights[*index])
            .collect::<Vec<usize>>()
            .iter()
            .sum::<usize>(),
        all_combinations[highest_value_index]
            .iter()
            .map(|index| values[*index])
            .collect::<Vec<i32>>(),
        highest_value
    );
}

#[cfg(test)]
mod knapsack_01_exponential {
    use super::*;

    #[test]
    fn survival_kit() {
        // Parameters
        // Example from https://rosettacode.org/wiki/Knapsack_problem/0-1#C
        let capacity = 400usize;
        let weights = vec![
            9usize, 13, 153, 50, 15, 68, 27, 39, 23, 52, 11, 32, 24, 48, 73, 42, 43, 22, 7, 18, 4,
            30,
        ];
        let values = vec![
            150i32, 35, 200, 160, 60, 45, 60, 40, 30, 10, 70, 30, 15, 10, 40, 70, 75, 80, 20, 12,
            50, 10,
        ];
        assert_eq!(weights.len(), values.len());
        // Power set
        let all_combinations = power_set_of(0usize, weights.len(), &vec![]);
        // Highest value combination
        let mut highest_value_index = 0;
        let mut highest_value = i32::min_value();
        for (cmb_i, cmb) in all_combinations.iter().enumerate() {
            let total_weight: usize = cmb
                .iter()
                .map(|index| weights[*index])
                .collect::<Vec<usize>>()
                .iter()
                .sum();
            let total_value: i32 = cmb
                .iter()
                .map(|index| values[*index])
                .collect::<Vec<i32>>()
                .iter()
                .sum();
            if total_weight <= capacity && total_value > highest_value {
                highest_value = total_value;
                highest_value_index = cmb_i;
            }
        }
        assert_eq!(
            vec![9, 13, 153, 50, 15, 27, 11, 42, 43, 22, 7, 4],
            all_combinations[highest_value_index]
                .iter()
                .map(|index| weights[*index])
                .collect::<Vec<usize>>()
        );
        assert_eq!(
            vec![150, 35, 200, 160, 60, 60, 70, 70, 75, 80, 20, 50],
            all_combinations[highest_value_index]
                .iter()
                .map(|index| values[*index])
                .collect::<Vec<i32>>()
        );
    }
}
