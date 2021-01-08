/// Divide an integer total in to parts based on ratios
pub fn ratio_reduce(total: i32, ratios: &[i32], maximums: &[i32], values: &[i32]) -> Vec<i32> {
    let ratios = ratios
        .iter()
        .zip(maximums.iter())
        .map(|(ratio, max)| if *max > 0 { *ratio } else { 0 });
    let mut total_ratio: i32 = ratios.clone().sum();
    if total_ratio <= 0 {
        return values.to_vec();
    }
    let mut total_remaining = total;
    let mut result: Vec<i32> = Vec::new();
    for (ratio, maximum, value) in itertools::izip!(ratios, maximums, values) {
        if ratio > 0 && total_ratio > 0 {
            let maybe_min = ((ratio * total_remaining) as f32 / total_ratio as f32).round() as i32;
            let distributed = maximum.min(&maybe_min);
            result.push(value - distributed);
            total_remaining -= distributed;
            total_ratio -= ratio;
        }
    }
    result
}

pub fn ratio_distribute(total: i32, ratios: &[i32], minimums: Option<&[i32]>) -> Vec<i32> {
    let ratios = if minimums.is_some() {
        ratios
            .iter()
            .zip(minimums.unwrap().iter())
            .map(|(ratio, min)| if *min > 0 { *ratio } else { 0 })
            .collect()
    } else {
        ratios.to_vec()
    };

    let mut total_ratio: i32 = ratios.iter().sum();
    assert!(total_ratio > 0, "sum of ratios must be > 0");

    let mut total_remaining = total;
    let mut distributed_total: Vec<i32> = Vec::new();
    let minimums = if let Some(minimums) = minimums {
        minimums.to_vec()
    } else {
        (0..ratios.len()).map(|_| 0).collect::<Vec<i32>>()
    };

    for (ratio, minimum) in ratios.iter().zip(minimums.iter()) {
        let distributed = if total_ratio > 0 {
            let maybe_minimum =
                (*ratio as f32 * total_remaining as f32 / total_ratio as f32).ceil() as i32;
            *minimum.max(&maybe_minimum)
        } else {
            total_remaining
        };
        distributed_total.push(distributed);
        total_ratio -= ratio;
        total_remaining -= distributed;
    }

    distributed_total
}

#[cfg(test)]
mod tests {
    use crate::ratio::{ratio_distribute, ratio_reduce};

    #[test]
    fn test_ratio_reduce() {
        let cases = [
            (20, [2, 4], [20, 20], [5, 5], [-2, -8]),
            (20, [2, 4], [1, 1], [5, 5], [4, 4]),
            (20, [2, 4], [1, 1], [2, 2], [1, 1]),
            (3, [2, 4], [3, 3], [2, 2], [1, 0]),
            (3, [2, 4], [3, 3], [0, 0], [-1, -2]),
            (3, [0, 0], [3, 3], [4, 4], [4, 4]),
        ];

        for (total, ratios, maximums, values, result) in &cases {
            assert_eq!(ratio_reduce(*total, ratios, maximums, values), result);
        }
    }

    #[test]
    fn test_ratio_distribute() {
        assert_eq!(ratio_distribute(10, &[1], None), [10]);
        assert_eq!(ratio_distribute(10, &[1, 1], None), [5, 5]);
        assert_eq!(ratio_distribute(12, &[1, 3], None), [3, 9]);
        assert_eq!(ratio_distribute(0, &[1, 3], None), [0, 0]);
        assert_eq!(ratio_distribute(0, &[1, 3], Some(&[1, 1])), [1, 1]);
        assert_eq!(ratio_distribute(10, &[1, 0], None), [10, 0]);
    }
}
