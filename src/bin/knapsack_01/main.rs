mod data;
use data::{get_inventory, KnapsackItem};

mod exponential;
mod exponential_with_pruning;
mod exponential_with_pruning_and_implicit_comparision;

fn main() {
    let (capacity, inventory) = get_inventory(22);
    exponential::pack(capacity, &inventory, true);
    exponential_with_pruning::pack(capacity, &inventory, true);
    exponential_with_pruning_and_implicit_comparision::pack(capacity, &inventory, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponential() {
        let (capacity, inventory) = get_inventory(22);
        let best_combination = exponential::pack(capacity, &inventory, false);
        assert_eq!(
            vec![9, 13, 153, 50, 15, 27, 11, 42, 43, 22, 7, 4],
            best_combination
                .iter()
                .map(|index| inventory[*index].weight)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![150, 35, 200, 160, 60, 60, 70, 70, 75, 80, 20, 50],
            best_combination
                .iter()
                .map(|index| inventory[*index].value)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn exponential_with_pruning() {
        let (capacity, inventory) = get_inventory(22);
        let best_combination = exponential_with_pruning::pack(capacity, &inventory, false);
        assert_eq!(
            vec![9, 13, 153, 50, 15, 27, 11, 42, 43, 22, 7, 4],
            best_combination
                .iter()
                .map(|index| inventory[*index].weight)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![150, 35, 200, 160, 60, 60, 70, 70, 75, 80, 20, 50],
            best_combination
                .iter()
                .map(|index| inventory[*index].value)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn exponential_with_pruning_and_implicit_comparision() {
        let (capacity, inventory) = get_inventory(22);
        let best_combination =
            exponential_with_pruning_and_implicit_comparision::pack(capacity, &inventory, false);
        assert_eq!(
            vec![9, 13, 153, 50, 15, 27, 11, 42, 43, 22, 7, 4],
            best_combination
                .iter()
                .map(|index| inventory[*index].weight)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![150, 35, 200, 160, 60, 60, 70, 70, 75, 80, 20, 50],
            best_combination
                .iter()
                .map(|index| inventory[*index].value)
                .collect::<Vec<_>>()
        );
    }

    use std::time::Instant;

    #[test]
    fn is_runtime_improved() {
        // TODO: add test to verify improvement in time complexity and visualize it
        // Parameters
        let offset = 5usize;
        let num_sampling_points = 10usize;
        let batch_size = 20usize;
        let rms_threshold = 1e-1;
        let mam_threshold = 1e-2;

        // Get runtimes
        let samples = (offset..(offset + num_sampling_points))
            .map(|n| {
                (0..batch_size)
                    .map(|_| {
                        let (capacity, inventory) = get_inventory(n);
                        let before = Instant::now();
                        let best_combination =
                            exponential_with_pruning_and_implicit_comparision::pack(
                                capacity, &inventory, false,
                            );
                        let after = Instant::now();
                        let duration_as_nanos = after.duration_since(before).as_nanos();
                        let duration = duration_as_nanos as f32;
                        duration
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        println!("{:?}", samples);
        // assert_eq!(
        //     vec![9, 13, 153, 50, 15, 27, 11, 42, 43, 22, 7, 4],
        //     best_combination
        //         .iter()
        //         .map(|index| inventory[*index].weight)
        //         .collect::<Vec<_>>()
        // );
        // assert_eq!(
        //     vec![150, 35, 200, 160, 60, 60, 70, 70, 75, 80, 20, 50],
        //     best_combination
        //         .iter()
        //         .map(|index| inventory[*index].value)
        //         .collect::<Vec<_>>()
        // );
    }
}
