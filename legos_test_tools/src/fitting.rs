extern crate nalgebra as na;
use na::linalg::QR;
use na::{Dynamic, Matrix, VecStorage, U1, U2};

type StimulusMatrix = Matrix<f32, Dynamic, U2, VecStorage<f32, Dynamic, U2>>;
type ResponseMatrix = Matrix<f32, Dynamic, U1, VecStorage<f32, Dynamic, U1>>;

pub fn linear_regression(samples: &Vec<(f32, f32)>) -> Option<(f32, f32)> {
    let stimulus = StimulusMatrix::from_fn(
        samples.len(),
        |r, c| {
            if c == 0 {
                1.0
            } else {
                samples[r].0
            }
        },
    );
    let response = ResponseMatrix::from_fn(samples.len(), |r, _| samples[r].1);
    let inv = QR::new(stimulus.transpose() * stimulus.clone()).try_inverse()?;
    let b = inv * stimulus.transpose() * response;
    let (intercept, slope) = (b[0], b[1]);
    Some((intercept, slope))
}

#[cfg(test)]
mod tests {
    // TODO: add
}
