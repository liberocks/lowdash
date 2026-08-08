/// Returns the compensated sum of pairwise products from two equal-length
/// slices.
///
/// Empty equal slices return `Some(0.0)`. A length mismatch returns `None`.
/// NaN and infinity terms follow IEEE-754 propagation.
///
/// # Complexity
///
/// Runs in `O(n)` time and `O(1)` additional space.
///
/// # Examples
///
/// ```rust
/// use lowdash::sum_products;
///
/// assert_eq!(sum_products(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]), Some(32.0));
/// assert_eq!(sum_products(&[], &[]), Some(0.0));
/// ```
pub fn sum_products(p: &[f64], q: &[f64]) -> Option<f64> {
    if p.len() != q.len() {
        return None;
    }

    let mut has_positive_infinity = false;
    let mut has_negative_infinity = false;
    let mut maximum_magnitude = 0.0_f64;

    for (&left, &right) in p.iter().zip(q) {
        let product = left * right;
        if product.is_nan() {
            return Some(f64::NAN);
        }
        if product == f64::INFINITY {
            has_positive_infinity = true;
        } else if product == f64::NEG_INFINITY {
            has_negative_infinity = true;
        } else {
            maximum_magnitude = maximum_magnitude.max(product.abs());
        }
    }

    if has_positive_infinity && has_negative_infinity {
        return Some(f64::NAN);
    }
    if has_positive_infinity {
        return Some(f64::INFINITY);
    }
    if has_negative_infinity {
        return Some(f64::NEG_INFINITY);
    }
    if maximum_magnitude == 0.0 {
        return Some(0.0);
    }

    let result = compensated_product_sum(p, q, 1.0);
    if result.is_finite() {
        Some(result)
    } else {
        Some(compensated_product_sum(p, q, maximum_magnitude) * maximum_magnitude)
    }
}

fn compensated_product_sum(p: &[f64], q: &[f64], scale: f64) -> f64 {
    let mut sum = 0.0;
    let mut compensation = 0.0;

    for (&left, &right) in p.iter().zip(q) {
        let product = left * right;
        if !product.is_finite() {
            continue;
        }

        let product = product / scale;
        let next = sum + product;
        if sum.abs() >= product.abs() {
            compensation += (sum - next) + product;
        } else {
            compensation += (product - next) + sum;
        }
        sum = next;
    }

    sum + compensation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_products_known_values() {
        assert_eq!(sum_products(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]), Some(32.0));
    }

    #[test]
    fn test_sum_products_empty_and_mismatched_inputs() {
        assert_eq!(sum_products(&[], &[]), Some(0.0));
        assert_eq!(sum_products(&[1.0], &[]), None);
        assert_eq!(sum_products(&[], &[1.0]), None);
    }

    #[test]
    fn test_sum_products_compensates_cancellation() {
        assert_eq!(
            sum_products(&[1.0e16, 1.0, -1.0e16], &[1.0, 1.0, 1.0]),
            Some(1.0)
        );
    }

    #[test]
    fn test_sum_products_handles_finite_overflow_and_cancellation() {
        assert_eq!(
            sum_products(&[f64::MAX, f64::MAX], &[1.0, 1.0]),
            Some(f64::INFINITY)
        );
        assert_eq!(
            sum_products(&[f64::MAX, f64::MAX, -f64::MAX], &[1.0, 1.0, 1.0]),
            Some(f64::MAX)
        );
    }

    #[test]
    fn test_sum_products_infinity_and_nan_behavior() {
        assert_eq!(sum_products(&[f64::INFINITY], &[1.0]), Some(f64::INFINITY));
        assert!(sum_products(&[f64::INFINITY], &[0.0]).unwrap().is_nan());
        assert!(
            sum_products(&[f64::INFINITY, f64::NEG_INFINITY], &[1.0, 1.0])
                .unwrap()
                .is_nan()
        );
    }
}
