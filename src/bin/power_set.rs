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

    extern crate plotters;

    use legos_curve_fitting::linear_regression;
    use legos_utils::function;
    use plotters::prelude::*;
    use std::time::Instant;

    #[test]
    fn time_complexity() {
        // Get runtimes
        let num_samples = 15u32;
        let offset = 5u32;
        let runtimes = (offset..(offset + num_samples))
            .map(|n| {
                let before = Instant::now();
                power_set_of(&(0..n).collect(), 0, &vec![]);
                let after = Instant::now();
                let duration_as_nanos = after.duration_since(before).as_nanos();
                duration_as_nanos as f32
                // println!("n={}, t={}ns", n, duration_as_nanos);
            })
            .collect::<Vec<f32>>();

        // Normalize samples
        let runtimes_norm = runtimes.iter().map(|&x| x * x).sum::<f32>().sqrt();
        let normalized_samples = runtimes
            .iter()
            .map(|&x| x / runtimes_norm)
            .enumerate()
            .map(|(i, runtime)| ((i as f32) / (num_samples as f32), runtime))
            .collect::<Vec<(f32, f32)>>();

        // FIXME: impl a exponential curve fitter to verify time complexity and visualize it
        let log_normalized_samples = normalized_samples
            .iter()
            .map(|(x, y)| (*x, y.log(std::f32::consts::E)))
            .collect::<Vec<(f32, f32)>>();
        let (intercept, slope, sum_squared_residuals) =
            linear_regression(&log_normalized_samples).unwrap();

        // Plot
        let img_path = format!("test_logs/{}.png", function!());
        let img = BitMapBackend::new(&img_path, (900, 900)).into_drawing_area();
        img.fill(&YELLOW).unwrap();
        let mut chart = ChartBuilder::on(&img)
            .caption(
                format!(
                    "fit = {} * e^{}x, sum squared residuals = {}",
                    intercept.exp(),
                    slope,
                    sum_squared_residuals
                ),
                ("Arial", 20),
            )
            .set_label_area_size(LabelAreaPosition::Left, 30)
            .set_label_area_size(LabelAreaPosition::Bottom, 30)
            .build_ranged(0f32..1f32, 0f32..1f32)
            .unwrap();
        chart.configure_mesh().draw().unwrap();
        chart
            .draw_series(LineSeries::new(normalized_samples.clone(), &RED))
            .unwrap()
            .label("power set")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        chart
            .draw_series(LineSeries::new(
                (0..=50)
                    .map(|x| x as f32 / 50.0)
                    .map(|x| (x, (intercept + x * slope).exp())),
                &BLACK,
            ))
            .unwrap()
            .label("power set fit")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));
        chart
            .draw_series(LineSeries::new(
                (0..=50).map(|x| (x as f32 / 50.0, x as f32 / 50.0)),
                &GREEN,
            ))
            .unwrap()
            .label("y = x")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
        chart
            .draw_series(LineSeries::new(
                (0..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &BLUE,
            ))
            .unwrap()
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();
    }
}
