mod data;
use data::{get_inventory, KnapsackItem};

mod exponential;
mod exponential_with_pruning;
mod exponential_with_pruning_and_implicit_comparision;

fn main() {
    let (capacity, inventory) = get_inventory();
    // exponential::pack(capacity, &inventory, true);
    // exponential_with_pruning::pack(capacity, &inventory, true);
    exponential_with_pruning_and_implicit_comparision::pack(capacity, &inventory, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponential() {
        let (capacity, inventory) = get_inventory();
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

    #[test]
    fn exponential_with_pruning() {
        let (capacity, inventory) = get_inventory();
        let best_combination = exponential_with_pruning::pack(capacity, &inventory, false);
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

    #[test]
    fn exponential_with_pruning_and_implicit_comparision() {
        let (capacity, inventory) = get_inventory();
        let best_combination =
            exponential_with_pruning_and_implicit_comparision::pack(capacity, &inventory, false);
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

    #[test]
    fn is_runtime_improved() {
        // TODO: add test to verify improvement in time complexity and visualize it
    }
}
