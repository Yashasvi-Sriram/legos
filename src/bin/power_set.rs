// TODO: use ordered set impl instead of assuming vec is a set
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
    use legos_test_tools::fitting::linear_regression;
    use legos_test_tools::function_path;
    use legos_test_tools::preprocessing::pretty_scale;
    use std::time::Instant;

    #[test]
    fn is_runtime_exponential() {
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
                        let before = Instant::now();
                        power_set_of(&(0..(n as u32)).collect(), 0, &vec![]);
                        let after = Instant::now();
                        let duration_as_nanos = after.duration_since(before).as_nanos();
                        let duration = duration_as_nanos as f32;
                        duration
                    })
                    .collect::<Vec<f32>>()
            })
            .flatten()
            .collect::<Vec<f32>>();

        // Normalize and log samples
        let normalized_samples = pretty_scale(&samples, num_sampling_points, batch_size);

        // Fit
        let log_normalized_samples = normalized_samples
            .iter()
            .map(|(x, y)| (*x, y.log(std::f32::consts::E)))
            .collect::<Vec<(f32, f32)>>();
        let (intercept, slope) = linear_regression(&log_normalized_samples).unwrap();

        // Errors
        let residues = normalized_samples
            .iter()
            .map(|(x, y)| (*x, y - ((intercept + x * slope).exp())))
            .collect::<Vec<(f32, f32)>>();
        let sqrt_mean_squared_residues = {
            let sum_squared_residues = residues.iter().map(|(_, y)| y * y).sum::<f32>();
            let mean_squared_residues = sum_squared_residues / num_sampling_points as f32;
            mean_squared_residues.sqrt()
        };
        let max_abs_medians_residues = {
            let abs_median_deflatten_residues = (0..num_sampling_points)
                .map(|sample_i| {
                    let mut at_x_batch = (0..batch_size)
                        .map(|iter_i| residues[sample_i * batch_size + iter_i].1)
                        .collect::<Vec<f32>>();
                    at_x_batch.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    at_x_batch[at_x_batch.len() / 2]
                })
                .map(|signed_median| signed_median.abs())
                .collect::<Vec<f32>>();
            let mut max = f32::NEG_INFINITY;
            for &mean in abs_median_deflatten_residues.iter() {
                if mean > max {
                    max = mean;
                }
            }
            max
        };

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
