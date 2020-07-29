//! <https://rosettacode.org/wiki/Knapsack_problem/0-1>
//! - Although this crate is for 0/1 knapsack problem, it can be used for a 0/n_i knapsack problem.
//! - Because 0/n_i knapsack problem can be converted to 0/1 knapsack problem by just flattening the inventory.
//! - Since we use a Vec (instead of a Set) to store the inventory, the same code works.
//! - A 0/n_i knapsack problem is called a bounded knapsack problem.

mod data;
use data::{Input, KnapsackItem};

mod exponential;
mod exponential_with_pruning;
mod exponential_with_pruning_and_implicit_comparision;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dynamic_programming() {
        // TODO
    }

    use gnuplot::*;
    use legos_test_tools::fitting::linear_regression;
    use legos_test_tools::function_path;
    use legos_test_tools::postprocessing::{max_abs_median_batched, sqrt_mean_squared};
    use legos_test_tools::preprocessing::pretty_scale;
    use std::time::Instant;

    #[test]
    fn time_complexity_improvment() {
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
                let (capacity, inventory) = Input::of_size(n);
                (0..batch_size)
                    .map(|_| {
                        let before = Instant::now();
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

        // Normalize and log samples
        let normalized_samples = pretty_scale(&samples, batch_size);

        // Fit
        let log_normalized_samples = normalized_samples
            .iter()
            .map(|(x, y)| (*x, y.log(std::f32::consts::E)))
            .collect::<Vec<_>>();
        let (intercept, slope) = linear_regression(&log_normalized_samples).unwrap();

        // Errors
        let residues = normalized_samples
            .iter()
            .map(|(x, y_observed)| y_observed - (intercept + x * slope).exp())
            .collect::<Vec<_>>();

        let sqrt_mean_squared_residues = sqrt_mean_squared(&residues);
        let max_abs_medians_residues = max_abs_median_batched(&residues, batch_size);

        // Plot
        let mut fg = Figure::new();
        fg.axes2d()
            .points(
                normalized_samples.iter().map(|(x, _)| x),
                normalized_samples.iter().map(|(_, y)| y),
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
                normalized_samples.iter().map(|(x, _)| x),
                residues.iter(),
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
                    sqrt(mean(residue_i^2)) = {}, threshold = {}
                    max(abs(median(residue_i))) = {}, threshold = {}
                    ",
                    intercept.exp(),
                    slope,
                    sqrt_mean_squared_residues,
                    rms_threshold,
                    max_abs_medians_residues,
                    mam_threshold
                ),
                &[],
            );
        fg.echo_to_file(&format!("test_logs/{}.gnuplot", function_path!()));

        // Asserts
        assert!(
            sqrt_mean_squared_residues < rms_threshold,
            "Possible problems: The function might be inappropriate for the data or the noise might have a high variance."
        );
        assert!(
            max_abs_medians_residues < mam_threshold,
            "Possible problems: There may be pattern in residues."
        );
    }
}
