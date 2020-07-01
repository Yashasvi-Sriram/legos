// FIXME: use ordered set impl instead of assuming vec is a set
fn power_set_of(ordered_set: &Vec<u32>, index: usize, parent: &Vec<u32>) -> Vec<Vec<u32>> {
    if index == ordered_set.len() {
        return vec![parent.clone()];
    }

    let left = parent.clone();
    let left_subsets = power_set_of(ordered_set, index + 1, &left);

    let right = {
        let mut clone = parent.clone();
        clone.push(ordered_set[index]);
        clone
    };
    let right_subsets = power_set_of(ordered_set, index + 1, &right);

    return left_subsets
        .into_iter()
        .chain(right_subsets.into_iter())
        .collect();
}

fn main() {
    let original_set = vec![4u32, 3, 2, 1];
    let power_set = power_set_of(&original_set, 0, &vec![]);
    println!(
        "power_set of {:?} has size {} = {:?}",
        original_set,
        power_set.len(),
        power_set
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(power_set_of(&vec![], 0, &vec![]), vec![vec![]]);
        assert_eq!(power_set_of(&vec![1], 0, &vec![]), vec![vec![], vec![1]]);
        assert_eq!(
            power_set_of(&vec![1, 2], 0, &vec![]),
            vec![vec![], vec![2], vec![1], vec![1, 2]]
        );
        assert_eq!(
            power_set_of(&vec![1, 2, 3], 0, &vec![]),
            vec![
                vec![],
                vec![3],
                vec![2],
                vec![2, 3],
                vec![1],
                vec![1, 3],
                vec![1, 2],
                vec![1, 2, 3]
            ]
        );
    }

    extern crate gnuplot;

    use gnuplot::*;
    use legos_curve_fitting::linear_regression;
    use legos_utils::function;
    use std::time::Instant;

    #[test]
    fn is_runtime_exponential() {
        // Get runtimes
        let offset = 5u32;
        let num_sizes = 10u32;
        let num_iterations = 20u32;
        let sizewise_samples = (offset..(offset + num_sizes))
            .map(|n| {
                let size_n_runtimes = (0..num_iterations)
                    .map(|_| {
                        let before = Instant::now();
                        power_set_of(&(0..n).collect(), 0, &vec![]);
                        let after = Instant::now();
                        let duration_as_nanos = after.duration_since(before).as_nanos();
                        let duration = duration_as_nanos as f32;
                        duration
                    })
                    .collect::<Vec<f32>>();
                size_n_runtimes
            })
            .collect::<Vec<Vec<f32>>>();

        // Normalize and log samples
        let runtimes_norm = sizewise_samples
            .iter()
            .map(|runtimes| {
                runtimes
                    .iter()
                    .map(|&runtime| runtime * runtime)
                    .collect::<Vec<f32>>()
            })
            .flatten()
            .sum::<f32>()
            .sqrt();
        let normalized_samples = sizewise_samples
            .iter()
            .map(|runtimes| {
                runtimes
                    .iter()
                    .map(|&runtime| runtime / runtimes_norm)
                    .collect::<Vec<f32>>()
            })
            .enumerate()
            .map(|(i, runtime)| ((i as f32) / (num_sizes as f32), runtime))
            .collect::<Vec<(f32, Vec<f32>)>>();

        // Fit
        let log_normalized_samples = normalized_samples
            .iter()
            .map(|(x, ys)| {
                (
                    *x,
                    ys.iter()
                        .map(|&y| y.log(std::f32::consts::E))
                        .collect::<Vec<f32>>(),
                )
            })
            .collect::<Vec<(f32, Vec<f32>)>>();
        let flattened_log_normalized_samples = log_normalized_samples
            .iter()
            .map(|(x, ys)| ys.iter().map(|&y| (*x, y)).collect::<Vec<(f32, f32)>>())
            .flatten()
            .collect::<Vec<(f32, f32)>>();
        let (intercept, slope) = linear_regression(&flattened_log_normalized_samples).unwrap();
        let sqrt_mean_squared_residuals = {
            let sum_squared_residuals = normalized_samples
                .iter()
                .map(|(x, ys)| {
                    ys.iter()
                        .map(|&y| y - (intercept + slope * *x).exp())
                        .map(|y| y * y)
                        .sum::<f32>()
                })
                .sum::<f32>();
            let mean_squared_residuals = sum_squared_residuals / normalized_samples.len() as f32;
            mean_squared_residuals.sqrt()
        };
        let max_abs_medians_residuals = {
            let abs_median_residuals = normalized_samples
                .iter()
                .map(|(x, ys)| {
                    let mut residuals_at_x = ys
                        .iter()
                        .map(|&y| y - (intercept + slope * *x).exp())
                        .collect::<Vec<f32>>();
                    residuals_at_x.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    residuals_at_x[residuals_at_x.len() / 2]
                })
                .map(|signed_median| signed_median.abs())
                .collect::<Vec<f32>>();
            let mut max = f32::NEG_INFINITY;
            for &mean in abs_median_residuals.iter() {
                if mean > max {
                    max = mean;
                }
            }
            max
        };

        // Plot
        let flattened_normalized_samples = normalized_samples
            .iter()
            .map(|(size, samples)| {
                samples
                    .iter()
                    .map(|&sample| (*size, sample))
                    .collect::<Vec<(f32, f32)>>()
            })
            .flatten()
            .collect::<Vec<(f32, f32)>>();
        let residues = normalized_samples
            .iter()
            .map(|(x, ys)| {
                ys.iter()
                    .map(|&y| (*x, y - ((intercept + x * slope).exp())))
                    .collect::<Vec<(f32, f32)>>()
            })
            .flatten()
            .collect::<Vec<(f32, f32)>>();

        let mut fg = Figure::new();
        fg.axes2d()
            .points(
                flattened_normalized_samples.iter().map(|(x, _)| x),
                flattened_normalized_samples.iter().map(|(_, y)| y),
                &[
                    Caption("samples"),
                    PointSymbol('x'),
                    Color("blue"),
                    PointSize(0.5),
                ],
            )
            .lines(
                (0..=50).map(|x| x as f32 / 50.0),
                (0..=50)
                    .map(|x| x as f32 / 50.0)
                    .map(|x| (intercept + x * slope).exp()),
                &[Caption("fit"), LineWidth(1.0), Color("black")],
            )
            .points(
                residues.iter().map(|(x, _)| x),
                residues.iter().map(|(_, y)| y),
                &[
                    Caption("residues"),
                    PointSymbol('x'),
                    Color("red"),
                    PointSize(0.5),
                ],
            )
            .set_x_label("scaled problem size", &[])
            .set_y_label("scaled runtime", &[])
            .set_legend(
                Graph(0.5),
                Graph(0.9),
                &[Placement(AlignCenter, AlignTop)],
                &[TextAlign(AlignRight)],
            )
            .set_grid_options(true, &[LineStyle(SmallDot), Color("black")])
            .set_x_grid(true)
            .set_y_grid(true)
            .set_title(
                &format!(
                    "
                    fit: y = {}e^{{ {} x}}
                    sqrt(mean(residual^2)) = {}
                    max(abs(median(residual_i))) = {}
                    ",
                    intercept.exp(),
                    slope,
                    sqrt_mean_squared_residuals,
                    max_abs_medians_residuals
                ),
                &[],
            );
        fg.echo_to_file(&format!("test_logs/{}.gnuplot", function!()));

        // Asserts
        assert!(sqrt_mean_squared_residuals < 1e-1);
        assert!(max_abs_medians_residuals < 1e-2);
    }
}
