use std::time::SystemTime;

/// Find the earliest time in a collection.
/// If the collection is empty, returns `None`.
///
/// # Arguments
///
/// * `times` - A slice of `SystemTime` items.
///
/// # Returns
///
/// * `Option<SystemTime>` - The earliest `SystemTime`, or `None` if the collection is empty.
///
/// # Examples
///
/// ```rust
/// use std::time::{SystemTime, Duration};
/// use lowdash::earliest;
///
/// let t1 = SystemTime::UNIX_EPOCH;
/// let t2 = t1 + Duration::new(60, 0);
/// let t3 = t1 + Duration::new(120, 0);
/// let times = vec![t2, t1, t3];
/// let earliest_time = earliest(&times);
/// assert_eq!(earliest_time, Some(t1));
/// ```
pub fn earliest(times: &[SystemTime]) -> Option<SystemTime> {
    if times.is_empty() {
        return None;
    }

    let mut min = times[0];

    for &item in &times[1..] {
        if item < min {
            min = item;
        }
    }

    Some(min)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_earliest_with_multiple_times() {
        let t1 = SystemTime::UNIX_EPOCH;
        let t2 = t1 + Duration::new(60, 0);
        let t3 = t1 + Duration::new(120, 0);
        let times = vec![t2, t1, t3];
        let result = earliest(&times);
        assert_eq!(result, Some(t1));
    }

    #[test]
    fn test_earliest_with_single_time() {
        let t = SystemTime::now();
        let times = vec![t];
        let result = earliest(&times);
        assert_eq!(result, Some(t));
    }

    #[test]
    fn test_earliest_with_empty_collection() {
        let times: Vec<SystemTime> = vec![];
        let result = earliest(&times);
        assert_eq!(result, None);
    }

    #[test]
    fn test_earliest_with_all_same_times() {
        let t = SystemTime::now();
        let times = vec![t, t, t];
        let result = earliest(&times);
        assert_eq!(result, Some(t));
    }

    #[test]
    fn test_earliest_with_varied_times() {
        let t1 = SystemTime::UNIX_EPOCH;
        let t2 = t1 + Duration::new(30, 0);
        let t3 = t1 + Duration::new(60, 0);
        let t4 = t1 + Duration::new(90, 0);
        let times = vec![t4, t2, t3, t1];
        let result = earliest(&times);
        assert_eq!(result, Some(t1));
    }
}
