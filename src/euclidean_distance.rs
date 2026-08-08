/// Returns the Euclidean distance between two points.
///
/// The points must have equal lengths and contain only finite values. Returns
/// `None` for mismatched lengths, nonfinite input, or a distance that cannot be
/// represented by `f64`. Equal empty points return `Some(0.0)`.
///
/// Each coordinate difference is accumulated with `f64::hypot`, which avoids
/// unnecessary intermediate overflow.
///
/// # Complexity
///
/// Runs in `O(n)` time and `O(1)` additional space.
///
/// # Examples
///
/// ```rust
/// use lowdash::euclidean_distance;
///
/// assert_eq!(euclidean_distance(&[0.0, 0.0], &[3.0, 4.0]), Some(5.0));
/// assert_eq!(euclidean_distance(&[], &[]), Some(0.0));
/// ```
pub fn euclidean_distance(p: &[f64], q: &[f64]) -> Option<f64> {
    if p.len() != q.len() {
        return None;
    }

    let mut distance = 0.0_f64;
    for (&left, &right) in p.iter().zip(q) {
        if !left.is_finite() || !right.is_finite() {
            return None;
        }

        let difference = left - right;
        if !difference.is_finite() {
            return None;
        }

        distance = distance.hypot(difference);
    }

    distance.is_finite().then_some(distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance_known_points() {
        assert_eq!(euclidean_distance(&[0.0, 0.0], &[3.0, 4.0]), Some(5.0));
        assert_eq!(euclidean_distance(&[1.0, -2.0], &[4.0, 2.0]), Some(5.0));
    }

    #[test]
    fn test_euclidean_distance_empty_and_equal_points() {
        assert_eq!(euclidean_distance(&[], &[]), Some(0.0));
        assert_eq!(euclidean_distance(&[1.0, 2.0], &[1.0, 2.0]), Some(0.0));
    }

    #[test]
    fn test_euclidean_distance_mismatched_lengths() {
        assert_eq!(euclidean_distance(&[1.0], &[1.0, 2.0]), None);
    }

    #[test]
    fn test_euclidean_distance_rejects_nonfinite_inputs() {
        assert_eq!(euclidean_distance(&[f64::NAN], &[0.0]), None);
        assert_eq!(euclidean_distance(&[f64::INFINITY], &[0.0]), None);
        assert_eq!(euclidean_distance(&[0.0], &[f64::NEG_INFINITY]), None);
    }

    #[test]
    fn test_euclidean_distance_uses_stable_hypot_accumulation() {
        let result = euclidean_distance(&[1.0e200, 0.0], &[0.0, 1.0e200]).unwrap();
        let expected = 2.0_f64.sqrt() * 1.0e200;

        assert!((result - expected).abs() / expected < 1.0e-15);
    }

    #[test]
    fn test_euclidean_distance_rejects_result_overflow() {
        assert_eq!(euclidean_distance(&[f64::MAX], &[-f64::MAX]), None);
    }
}
