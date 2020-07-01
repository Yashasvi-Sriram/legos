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
        let num_iterations = 10u32;
        let mut sum_squared_runtimes = 0f32;
        let runtimes = (offset..(offset + num_sizes))
            .map(|n| {
                let size_n_runtimes = (0..num_iterations)
                    .map(|_| {
                        let before = Instant::now();
                        power_set_of(&(0..n).collect(), 0, &vec![]);
                        let after = Instant::now();
                        let duration_as_nanos = after.duration_since(before).as_nanos();
                        let duration = duration_as_nanos as f32;
                        sum_squared_runtimes = duration * duration;
                        duration
                    })
                    .collect::<Vec<f32>>();
                size_n_runtimes
            })
            .collect::<Vec<Vec<f32>>>();

        // Normalize and log samples
        let runtimes_norm = sum_squared_runtimes.sqrt();
        let normalized_samples = runtimes
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

        // FIXME:
        // - [x] impl a exponential curve fitter use linear regression on log values
        // - [ ] verify fit by
        //      for each i ( mean_over_j (T_j(i) - T_hat(i)) < th_1 )
        //      MSE < N * (th_2)
        //      use exponenetiated losses
        // - [x] visualize; need interactive better graphs
        let (intercept, slope, sum_squared_residuals) =
            linear_regression(&log_normalized_samples).unwrap();

        // Plot
        let flattened_samples = normalized_samples
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
                flattened_samples.iter().map(|(x, _)| x),
                flattened_samples.iter().map(|(_, y)| y),
                &[
                    Caption("samples"),
                    PointSymbol('O'),
                    Color("red"),
                    PointSize(0.25),
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
                    PointSymbol('o'),
                    Color("magenta"),
                    PointSize(0.25),
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
            .set_title(
                &format!("fit: y = {}e^{{ {} x}}", intercept.exp(), slope),
                &[],
            );
        fg.echo_to_file(&format!("test_logs/{}.gnuplot", function!()));
    }
}
