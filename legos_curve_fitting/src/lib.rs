extern crate nalgebra as na;
use na::linalg::QR;
use na::{Dynamic, Matrix, VecStorage, U1, U2};

type StimulusMatrix = Matrix<f32, Dynamic, U2, VecStorage<f32, Dynamic, U2>>;
type ResponseMatrix = Matrix<f32, Dynamic, U1, VecStorage<f32, Dynamic, U1>>;

pub fn linear_regression(sizewise_samples: &Vec<(f32, Vec<f32>)>) -> Option<(f32, f32, f32)> {
    let flattened_samples = sizewise_samples
        .iter()
        .map(|(size, samples)| {
            samples
                .iter()
                .map(|&sample| (*size, sample))
                .collect::<Vec<(f32, f32)>>()
        })
        .flatten()
        .collect::<Vec<(f32, f32)>>();
    let stimulus = StimulusMatrix::from_fn(flattened_samples.len(), |r, c| {
        if c == 0 {
            1.0
        } else {
            flattened_samples[r].0
        }
    });
    let response = ResponseMatrix::from_fn(flattened_samples.len(), |r, _| flattened_samples[r].1);
    let inv = QR::new(stimulus.transpose() * stimulus.clone()).try_inverse()?;
    let b = inv * stimulus.transpose() * response;
    let (intercept, slope) = (b[0], b[1]);
    let sum_sqaured_residuals = flattened_samples
        .iter()
        .map(|&(x, y)| intercept + slope * x - y)
        .map(|x| x * x)
        .sum::<f32>();
    let mean_of_residuals = sizewise_samples
        .iter()
        .map(|(x, ys)| {
            ys.iter()
                .map(|&y| y - (intercept + slope * *x))
                .sum::<f32>()
                .abs()
        })
        .collect::<Vec<f32>>();
    println!("{:?}", mean_of_residuals);
    Some((intercept, slope, sum_sqaured_residuals))
}

#[cfg(test)]
mod tests {}
