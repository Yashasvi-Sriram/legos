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
mod power_set {
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

    extern crate textplots;

    use std::time::Instant;
    use textplots::{Chart, Plot, Shape};

    fn normalize(samples: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
        let norm_x = samples.iter().map(|&(x, _)| x * x).sum::<f32>().sqrt();
        let norm_y = samples.iter().map(|&(_, y)| y * y).sum::<f32>().sqrt();
        let mut normalized = vec![];
        for &(x, y) in samples.iter() {
            normalized.push((x / norm_x, y / norm_y));
        }
        return normalized;
    }

    fn min_max_scale(samples: &Vec<f32>) -> Vec<f32> {
        let mut min = f32::INFINITY;
        let mut max = f32::NEG_INFINITY;
        for &x in samples.iter() {
            if x > max {
                max = x;
            }
            if x < min {
                min = x;
            }
        }
        let mut scaled = samples.clone();
        for (i, &x) in samples.iter().enumerate() {
            scaled[i] = (x - min) / (max - min);
        }
        scaled
    }

    fn scale(samples: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
        let first = samples.iter().map(|&(x, _)| x).collect::<Vec<f32>>();
        let second = samples.iter().map(|&(_, y)| y).collect::<Vec<f32>>();
        let scaled_first = min_max_scale(&first);
        let scaled_second = min_max_scale(&second);
        let mut scaled = vec![];
        for i in 0..scaled_first.len() {
            scaled.push((scaled_first[i], scaled_second[i]));
        }
        return scaled;
    }

    #[test]
    fn bench() {
        let i1 = Instant::now();
        power_set_of(&vec![1, 2], 0, &vec![]);
        let i2 = Instant::now();
        power_set_of(&vec![1, 2, 3], 0, &vec![]);
        let i3 = Instant::now();
        power_set_of(&vec![1, 2, 3, 4], 0, &vec![]);
        let i4 = Instant::now();
        power_set_of(&vec![1, 2, 3, 4, 5], 0, &vec![]);
        let i5 = Instant::now();
        power_set_of(&vec![1, 2, 3, 4, 5, 6], 0, &vec![]);
        let i6 = Instant::now();
        let samples = vec![
            (2f32, i2.duration_since(i1).as_nanos() as f32),
            (3f32, i3.duration_since(i2).as_nanos() as f32),
            (4f32, i4.duration_since(i3).as_nanos() as f32),
            (5f32, i5.duration_since(i4).as_nanos() as f32),
            (6f32, i6.duration_since(i5).as_nanos() as f32),
        ];
        let normalized_samples = normalize(&samples);
        let scaled_samples = scale(&samples);
        Chart::new(100, 100, 0f32, 1f32)
            .lineplot(Shape::Lines(&normalized_samples))
            .lineplot(Shape::Lines(&scaled_samples))
            .lineplot(Shape::Continuous(|_| 0f32))
            .lineplot(Shape::Continuous(|x| x))
            .lineplot(Shape::Continuous(|x| x * x))
            .display();
        // println!("{}ns", i2.duration_since(i1).as_nanos());
        // println!("{}ns", i3.duration_since(i2).as_nanos());
        // println!("{}ns", i4.duration_since(i3).as_nanos());
        // println!("{}ns", i5.duration_since(i4).as_nanos());
        // println!("{}ns", i6.duration_since(i5).as_nanos());
        // TODO: impl a exponential curve fitter and check if the durations follow an exponenetial curve
    }
}
