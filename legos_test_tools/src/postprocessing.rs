pub fn sqrt_mean_squared(values: &Vec<f32>) -> f32 {
    let num_values = values.len();
    let sum_squared_residues = values.iter().map(|y| y * y).sum::<f32>();
    let mean_squared_residues = sum_squared_residues / num_values as f32;
    mean_squared_residues.sqrt()
}

pub fn max_abs_median_batched(values: &Vec<f32>, batch_size: usize) -> f32 {
    assert_eq!(values.len() % batch_size, 0);
    let num_batches = values.len() / batch_size;
    let abs_median_deflatten_residues = (0..num_batches)
        .map(|batch_i| {
            let mut at_x_batch = (0..batch_size)
                .map(|iter_i| values[batch_i * batch_size + iter_i])
                .collect::<Vec<f32>>();
            at_x_batch.sort_by(|a, b| a.partial_cmp(b).unwrap());
            at_x_batch[at_x_batch.len() / 2]
        })
        .map(|signed_median| signed_median.abs())
        .collect::<Vec<f32>>();
    let max = abs_median_deflatten_residues
        .iter()
        .fold(
            f32::NEG_INFINITY,
            |max, &element| {
                if element > max {
                    element
                } else {
                    max
                }
            },
        );
    max
}
