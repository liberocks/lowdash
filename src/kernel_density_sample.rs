/// Draws deterministic approximate samples from a Gaussian kernel density
/// estimate.
///
/// Input data must be nonempty and finite, and `bandwidth` must be finite and
/// positive. Invalid input returns `None`; a valid zero `count` returns an
/// empty vector. The `seed` controls a deterministic, non-cryptographic
/// pseudo-random generator, so the same input and seed produce the same
/// samples. Sampling is approximate and uses Gaussian Box-Muller noise.
///
/// # Complexity
///
/// Runs in `O(count)` time and `O(count)` space.
///
/// # Examples
///
/// ```rust
/// use lowdash::kernel_density_sample;
///
/// let samples = kernel_density_sample(&[0.0, 1.0], 0.25, 3, 42).unwrap();
/// assert_eq!(samples.len(), 3);
/// ```
pub fn kernel_density_sample(
    values: &[f64],
    bandwidth: f64,
    count: usize,
    seed: u64,
) -> Option<Vec<f64>> {
    if values.is_empty()
        || !bandwidth.is_finite()
        || bandwidth <= 0.0
        || values.iter().any(|value| !value.is_finite())
    {
        return None;
    }

    if count == 0 {
        return Some(Vec::new());
    }

    let mut random = Lcg::new(seed);
    let mut samples = Vec::with_capacity(count);
    let mut cached_noise = None;
    for _ in 0..count {
        let center = values[random.index_below(values.len())];
        let noise = if let Some(noise) = cached_noise.take() {
            noise
        } else {
            let (first, second) = standard_normal_pair(&mut random);
            cached_noise = Some(second);
            first
        };
        samples.push(center + bandwidth * noise);
    }

    Some(samples)
}

struct Lcg {
    state: u64,
}

impl Lcg {
    const MULTIPLIER: u64 = 6_364_136_223_846_793_005;
    const INCREMENT: u64 = 1_442_695_040_888_963_407;
    const MANTISSA_DENOMINATOR: f64 = 9_007_199_254_740_992.0;

    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(Self::MULTIPLIER)
            .wrapping_add(Self::INCREMENT);
        self.state
    }

    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / Self::MANTISSA_DENOMINATOR
    }

    fn unit_open(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f64 + 1.0) / (Self::MANTISSA_DENOMINATOR + 1.0)
    }

    fn index_below(&mut self, upper: usize) -> usize {
        let upper = upper as u64;
        if upper <= 1 {
            return 0;
        }

        let threshold = upper.wrapping_neg() % upper;
        loop {
            let candidate = self.next_u64();
            if candidate >= threshold {
                return (candidate % upper) as usize;
            }
        }
    }
}

fn standard_normal_pair(random: &mut Lcg) -> (f64, f64) {
    let radius = (-2.0 * random.unit_open().ln()).sqrt();
    let angle = std::f64::consts::TAU * random.unit();
    (radius * angle.cos(), radius * angle.sin())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_density_sample_reproducibility() {
        let first = kernel_density_sample(&[-1.0, 0.0, 1.0], 0.25, 32, 42).unwrap();
        let second = kernel_density_sample(&[-1.0, 0.0, 1.0], 0.25, 32, 42).unwrap();
        let different_seed = kernel_density_sample(&[-1.0, 0.0, 1.0], 0.25, 32, 43).unwrap();

        assert_eq!(first, second);
        assert_ne!(first, different_seed);
    }

    #[test]
    fn test_kernel_density_sample_count_and_finite_values() {
        let samples = kernel_density_sample(&[0.0, 1.0], 0.25, 128, 7).unwrap();

        assert_eq!(samples.len(), 128);
        assert!(samples.iter().all(|sample| sample.is_finite()));
    }

    #[test]
    fn test_kernel_density_sample_zero_count() {
        assert_eq!(kernel_density_sample(&[0.0], 1.0, 0, 42), Some(Vec::new()));
    }

    #[test]
    fn test_kernel_density_sample_rejects_invalid_input() {
        assert!(kernel_density_sample(&[], 1.0, 1, 42).is_none());
        assert!(kernel_density_sample(&[0.0, f64::NAN], 1.0, 1, 42).is_none());
        assert!(kernel_density_sample(&[0.0, f64::INFINITY], 1.0, 1, 42).is_none());
        assert!(kernel_density_sample(&[0.0], 0.0, 1, 42).is_none());
        assert!(kernel_density_sample(&[0.0], -1.0, 1, 42).is_none());
        assert!(kernel_density_sample(&[0.0], f64::NAN, 1, 42).is_none());
        assert!(kernel_density_sample(&[0.0], f64::INFINITY, 1, 42).is_none());
    }

    #[test]
    fn test_kernel_density_sample_stays_near_broad_data_location() {
        let samples = kernel_density_sample(&[0.0, 10.0], 0.25, 1_024, 123).unwrap();
        let mean = samples.iter().sum::<f64>() / samples.len() as f64;

        assert!(mean > 4.5 && mean < 5.5);
    }
}
