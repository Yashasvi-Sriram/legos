pub struct KnapsackItem {
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

mod exponential {
    // FIXME: use ordered set impl instead of assuming vec is a set
    fn pack_recur(
        index: usize,
        size: usize,
        parent: &Vec<usize>,
        capacity: usize,
        inventory: &Vec<super::KnapsackItem>,
    ) -> Option<Vec<usize>> {
        if parent
            .iter()
            .map(|index| inventory[*index].weight)
            .collect::<Vec<usize>>()
            .iter()
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

        match (left_best_pack, right_best_pack) {
            (Some(left), Some(right)) => {
                let left_value: i32 = left
                    .iter()
                    .map(|index| inventory[*index].value)
                    .collect::<Vec<i32>>()
                    .iter()
                    .sum();
                let right_value: i32 = right
                    .iter()
                    .map(|index| inventory[*index].value)
                    .collect::<Vec<i32>>()
                    .iter()
                    .sum();
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
        let best_combination =
            pack_recur(0usize, inventory.len(), &vec![], capacity, inventory).unwrap();
        if debug {
            println!(
        "highest value combination =\n\t{:?}\n\t{:?}\nweights\t{:?}\nsum\t{}\nvalues\t{:?}\nsum\t{}",
        best_combination,
        best_combination
            .iter()
            .map(|index| inventory[*index].name)
            .collect::<Vec<&str>>(),
        best_combination
            .iter()
            .map(|index| inventory[*index].weight)
            .collect::<Vec<usize>>(),
        best_combination
            .iter()
            .map(|index| inventory[*index].weight)
            .collect::<Vec<usize>>()
            .iter()
            .sum::<usize>(),
        best_combination
            .iter()
            .map(|index| inventory[*index].value)
            .collect::<Vec<i32>>(),
        best_combination
            .iter()
            .map(|index| inventory[*index].value)
            .collect::<Vec<i32>>()
            .iter()
            .sum::<i32>()
        );
        }
        best_combination.clone()
    }
}

fn main() {
    let capacity = 400usize;
    let inventory = get_inventory();
    exponential::pack(capacity, &inventory, true);
}

#[cfg(test)]
mod knapsack_01 {
    use super::*;

    #[test]
    fn survival_kit_exponential() {
        let capacity = 400usize;
        let inventory = get_inventory();
        let best_combination = exponential::pack(capacity, &inventory, false);
        assert_eq!(
            vec![9, 13, 153, 50, 15, 27, 11, 42, 43, 22, 7, 4],
            best_combination
                .iter()
                .map(|index| inventory[*index].weight)
                .collect::<Vec<usize>>()
        );
        assert_eq!(
            vec![150, 35, 200, 160, 60, 60, 70, 70, 75, 80, 20, 50],
            best_combination
                .iter()
                .map(|index| inventory[*index].value)
                .collect::<Vec<i32>>()
        );
    }
}
