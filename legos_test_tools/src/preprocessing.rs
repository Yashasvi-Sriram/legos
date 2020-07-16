/// - x is min-max scaled, normalizing x values makes graph too skewed
/// - y is normalized, min-max scaling makes some some y values 0. This can cause problems when taking log for fitting exponential function.
/// - 0 to 1 scaling is ensured either way.
pub fn pretty_scale(samples: &Vec<f32>, num_xs: usize, batch_size: usize) -> Vec<(f32, f32)> {
    let y_norm = samples.iter().map(|y_i| y_i * y_i).sum::<f32>().sqrt();
    let normalized_samples = samples
        .iter()
        .enumerate()
        .map(|(i, &y_i)| ((i / batch_size) as f32 / (num_xs as f32), y_i / y_norm))
        .collect::<Vec<(f32, f32)>>();
    normalized_samples
}

#[cfg(test)]
mod tests {
    // TODO: add
}
