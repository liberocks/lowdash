use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the latest `SystemTime` from the provided arguments.
/// If no arguments are provided, returns `SystemTime::UNIX_EPOCH`.
///
/// # Arguments
/// * `times` - A slice of `SystemTime` instances.
///
/// # Returns
/// * `SystemTime` - The latest `SystemTime` among the provided arguments.
/// * If no arguments are provided, returns `SystemTime::UNIX_EPOCH`.
///
/// # Examples
/// ```rust
/// use std::time::{SystemTime, Duration};
/// use lowdash::latest;
///
/// let now = SystemTime::now();
/// let later = now + Duration::new(10, 0);
/// let latest_time = latest(&[now, later]);
/// assert_eq!(latest_time, later);
/// ```
pub fn latest(times: &[SystemTime]) -> SystemTime {
    if times.is_empty() {
        return UNIX_EPOCH;
    }

    let mut max = times[0];
    for &item in &times[1..] {
        if item > max {
            max = item;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_latest() {
        let base = UNIX_EPOCH + Duration::new(0, 0);
        let time1 = base + Duration::new(100, 0);
        let time2 = base + Duration::new(200, 0);
        let time3 = base + Duration::new(300, 0);
        let result = latest(&[time1, time2, time3]);
        assert_eq!(result, time3);
    }

    #[test]
    fn test_latest_single() {
        let time = UNIX_EPOCH + Duration::new(100, 0);
        let result = latest(&[time]);
        assert_eq!(result, time);
    }

    #[test]
    fn test_latest_empty() {
        let result = latest(&[]);
        assert_eq!(result, UNIX_EPOCH);
    }

    #[test]
    fn test_latest_with_duplicates() {
        let base = UNIX_EPOCH + Duration::new(0, 0);
        let time1 = base + Duration::new(100, 0);
        let time2 = base + Duration::new(200, 0);
        let time3 = base + Duration::new(200, 0);
        let result = latest(&[time1, time2, time3]);
        assert_eq!(result, time2);
    }

    #[test]
    fn test_latest_mixed_order() {
        let base = UNIX_EPOCH + Duration::new(0, 0);
        let time1 = base + Duration::new(300, 0);
        let time2 = base + Duration::new(100, 0);
        let time3 = base + Duration::new(200, 0);
        let result = latest(&[time1, time2, time3]);
        assert_eq!(result, time1);
    }

    #[test]
    fn test_latest_all_same() {
        let time = UNIX_EPOCH + Duration::new(150, 0);
        let result = latest(&[time, time, time]);
        assert_eq!(result, time);
    }

    #[test]
    fn test_latest_with_future_times() {
        let now = SystemTime::now();
        let future1 = now + Duration::new(1000, 0);
        let future2 = now + Duration::new(2000, 0);
        let future3 = now + Duration::new(3000, 0);
        let result = latest(&[future1, future2, future3]);
        assert_eq!(result, future3);
    }

    #[test]
    fn test_latest_with_past_times() {
        let now = SystemTime::now();
        let past1 = now - Duration::new(3000, 0);
        let past2 = now - Duration::new(2000, 0);
        let past3 = now - Duration::new(1000, 0);
        let result = latest(&[past1, past2, past3]);
        assert_eq!(result, past3);
    }
}
