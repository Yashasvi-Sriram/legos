struct KnapsackItem {
    name: &'static str,
    weight: usize,
    value: i32,
}

// Example from https://rosettacode.org/wiki/Knapsack_problem/0-1#C
fn get_inventory() -> Vec<KnapsackItem> {
    vec![
        KnapsackItem {
            name: "map",
            weight: 9,
            value: 150,
        },
        KnapsackItem {
            name: "compass",
            weight: 13,
            value: 35,
        },
        KnapsackItem {
            name: "water",
            weight: 153,
            value: 200,
        },
        KnapsackItem {
            name: "sandwich",
            weight: 50,
            value: 160,
        },
        KnapsackItem {
            name: "glucose",
            weight: 15,
            value: 60,
        },
        KnapsackItem {
            name: "tin",
            weight: 68,
            value: 45,
        },
        KnapsackItem {
            name: "banana",
            weight: 27,
            value: 60,
        },
        KnapsackItem {
            name: "apple",
            weight: 39,
            value: 40,
        },
        KnapsackItem {
            name: "cheese",
            weight: 23,
            value: 30,
        },
        KnapsackItem {
            name: "beer",
            weight: 52,
            value: 10,
        },
        KnapsackItem {
            name: "suntan cream",
            weight: 11,
            value: 70,
        },
        KnapsackItem {
            name: "camera",
            weight: 32,
            value: 30,
        },
        KnapsackItem {
            name: "T-shirt",
            weight: 24,
            value: 15,
        },
        KnapsackItem {
            name: "trousers",
            weight: 48,
            value: 10,
        },
        KnapsackItem {
            name: "umbrella",
            weight: 73,
            value: 40,
        },
        KnapsackItem {
            name: "waterproof trousers",
            weight: 42,
            value: 70,
        },
        KnapsackItem {
            name: "waterproof overclothes",
            weight: 43,
            value: 75,
        },
        KnapsackItem {
            name: "note-case",
            weight: 22,
            value: 80,
        },
        KnapsackItem {
            name: "sunglasses",
            weight: 7,
            value: 20,
        },
        KnapsackItem {
            name: "towel",
            weight: 18,
            value: 12,
        },
        KnapsackItem {
            name: "socks",
            weight: 4,
            value: 50,
        },
        KnapsackItem {
            name: "book",
            weight: 30,
            value: 10,
        },
    ]
}

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
    let capacity = 400usize;
    let inventory = get_inventory();
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
    // Power set
    let all_combinations = power_set_of(0usize, inventory.len(), &vec![]);
    println!("total #combinations = {}", all_combinations.len());
    // Highest value combination
    let mut highest_value_index = 0;
    let mut highest_value = i32::min_value();
    for (cmb_i, cmb) in all_combinations.iter().enumerate() {
        let total_weight: usize = cmb
            .iter()
            .map(|index| inventory[*index].weight)
            .collect::<Vec<usize>>()
            .iter()
            .sum();
        let total_value: i32 = cmb
            .iter()
            .map(|index| inventory[*index].value)
            .collect::<Vec<i32>>()
            .iter()
            .sum();
        if total_weight <= capacity && total_value > highest_value {
            highest_value = total_value;
            highest_value_index = cmb_i;
        }
    }
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
            .collect::<Vec<usize>>()
            .iter()
            .sum::<usize>(),
        all_combinations[highest_value_index]
            .iter()
            .map(|index| inventory[*index].value)
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
        let capacity = 400usize;
        let inventory = get_inventory();
        // Power set
        let all_combinations = power_set_of(0usize, inventory.len(), &vec![]);
        // Highest value combination
        let mut highest_value_index = 0;
        let mut highest_value = i32::min_value();
        for (cmb_i, cmb) in all_combinations.iter().enumerate() {
            let total_weight: usize = cmb
                .iter()
                .map(|index| inventory[*index].weight)
                .collect::<Vec<usize>>()
                .iter()
                .sum();
            let total_value: i32 = cmb
                .iter()
                .map(|index| inventory[*index].value)
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
                .map(|index| inventory[*index].weight)
                .collect::<Vec<usize>>()
        );
        assert_eq!(
            vec![150, 35, 200, 160, 60, 60, 70, 70, 75, 80, 20, 50],
            all_combinations[highest_value_index]
                .iter()
                .map(|index| inventory[*index].value)
                .collect::<Vec<i32>>()
        );
    }
}
