/// Returns `n - 1` inclusive, linearly interpolated quantile cut points.
///
/// Cut point `i` uses percentile `100 * i / n`, for `i` in `1..n`, with the
/// same rank convention as [`crate::percentile`]. Empty input, zero `n`, or
/// nonfinite values returns `None`; `n == 1` returns `Some(Vec::new())`.
///
/// **Time Complexity:** O(m log m + n) time and O(m + n) space, where `m` is
/// the input length.
///
/// # Examples
/// ```rust
/// use lowdash::quantiles;
///
/// assert_eq!(quantiles(&[1.0, 2.0, 3.0, 4.0, 5.0], 4), Some(vec![2.0, 3.0, 4.0]));
/// assert_eq!(quantiles(&[1.0, 2.0], 1), Some(Vec::new()));
/// ```
pub fn quantiles(values: &[f64], n: usize) -> Option<Vec<f64>> {
    if values.is_empty() || n == 0 || values.iter().any(|value| !value.is_finite()) {
        return None;
    }
    if n == 1 {
        return Some(Vec::new());
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(|left, right| left.partial_cmp(right).unwrap());
    let mut result = Vec::with_capacity(n - 1);
    let last_index = (sorted.len() - 1) as f64;

    for cut in 1..n {
        let rank = cut as f64 / n as f64 * last_index;
        let lower = rank.floor() as usize;
        let upper = rank.ceil() as usize;
        if lower == upper {
            result.push(sorted[lower]);
        } else {
            let fraction = rank - lower as f64;
            result.push(sorted[lower] + (sorted[upper] - sorted[lower]) * fraction);
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_inclusive_quartiles() {
        assert_eq!(
            quantiles(&[1.0, 2.0, 3.0, 4.0, 5.0], 4),
            Some(vec![2.0, 3.0, 4.0])
        );
    }

    #[test]
    fn linearly_interpolates_cut_points() {
        let result = quantiles(&[1.0, 2.0, 3.0, 4.0], 4).unwrap();

        assert_eq!(result.len(), 3);
        assert!((result[0] - 1.75).abs() < 1e-12);
        assert!((result[1] - 2.5).abs() < 1e-12);
        assert!((result[2] - 3.25).abs() < 1e-12);
    }

    #[test]
    fn handles_empty_zero_and_one_cut_count() {
        assert_eq!(quantiles(&[], 4), None);
        assert_eq!(quantiles(&[1.0, 2.0], 0), None);
        assert_eq!(quantiles(&[1.0, 2.0], 1), Some(Vec::new()));
    }

    #[test]
    fn handles_duplicates_and_negative_values() {
        let result = quantiles(&[-4.0, -2.0, -2.0, 2.0, 4.0], 2).unwrap();

        assert_eq!(result, vec![-2.0]);
        assert_eq!(quantiles(&[1.0, 1.0, 1.0], 4), Some(vec![1.0, 1.0, 1.0]));
    }

    #[test]
    fn rejects_nonfinite_values() {
        assert_eq!(quantiles(&[1.0, f64::NAN], 2), None);
        assert_eq!(quantiles(&[1.0, f64::INFINITY], 2), None);
    }
}
