use std::time::SystemTime;

/// Find the earliest item in a collection based on a custom iteratee function.
/// If the collection is empty, returns `None`.
///
/// # Arguments
///
/// * `collection` - A slice of items.
/// * `iteratee` - A function that takes an item and returns a `SystemTime` used for comparison.
///
/// # Returns
///
/// * `Option<T>` - The earliest item based on the iteratee function, or `None` if the collection is empty.
///
/// # Examples
/// ```rust
/// use std::time::{SystemTime, Duration};
/// use lowdash::earliest_by;
///
/// let t1 = SystemTime::UNIX_EPOCH;
/// let t2 = t1 + Duration::new(60, 0);
/// let t3 = t1 + Duration::new(120, 0);
/// let times = vec![t2, t1, t3];
/// let earliest_time = earliest_by(&times, |&t| t);
/// assert_eq!(earliest_time, Some(t1));
///
/// #[derive(Debug, PartialEq, Clone)]
/// struct Event {
///     name: String,
///     timestamp: SystemTime,
/// }
///
/// let events = vec![
///     Event {
///         name: "Event1".to_string(),
///         timestamp: t2,
///     },
///     Event {
///         name: "Event2".to_string(),
///         timestamp: t1,
///     },
///     Event {
///         name: "Event3".to_string(),
///         timestamp: t3,
///     },
/// ];
///
/// let earliest_event = earliest_by(&events, |e| e.timestamp);
/// assert_eq!(
///     earliest_event,
///     Some(Event {
///         name: "Event2".to_string(),
///         timestamp: t1,
///     })
/// );
/// ```
pub fn earliest_by<T, F>(collection: &[T], iteratee: F) -> Option<T>
where
    T: Clone,
    F: Fn(&T) -> SystemTime,
{
    if collection.is_empty() {
        return None;
    }

    let mut earliest = collection[0].clone();
    let mut earliest_time = iteratee(&earliest);

    for item in &collection[1..] {
        let item_time = iteratee(item);
        if item_time < earliest_time {
            earliest = item.clone();
            earliest_time = item_time;
        }
    }

    Some(earliest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[derive(Debug, PartialEq, Clone)]
    struct Event {
        name: String,
        timestamp: SystemTime,
    }

    #[test]
    fn test_earliest_by_with_multiple_items() {
        let t1 = SystemTime::UNIX_EPOCH;
        let t2 = t1 + Duration::new(60, 0);
        let t3 = t1 + Duration::new(120, 0);
        let times = vec![t2, t1, t3];
        let result = earliest_by(&times, |&t| t);
        assert_eq!(result, Some(t1));
    }

    #[test]
    fn test_earliest_by_with_single_item() {
        let t = SystemTime::now();
        let times = vec![t];
        let result = earliest_by(&times, |&t| t);
        assert_eq!(result, Some(t));
    }

    #[test]
    fn test_earliest_by_empty_collection() {
        let times: Vec<SystemTime> = vec![];
        let result = earliest_by(&times, |&t| t);
        assert_eq!(result, None);
    }

    #[test]
    fn test_earliest_by_all_same_times() {
        let t = SystemTime::now();
        let times = vec![t, t, t];
        let result = earliest_by(&times, |&t| t);
        assert_eq!(result, Some(t));
    }

    #[test]
    fn test_earliest_by_custom_struct() {
        let t1 = SystemTime::UNIX_EPOCH;
        let t2 = t1 + Duration::new(30, 0);
        let t3 = t1 + Duration::new(60, 0);
        let t4 = t1 + Duration::new(90, 0);

        let events = vec![
            Event {
                name: "Event1".to_string(),
                timestamp: t2,
            },
            Event {
                name: "Event2".to_string(),
                timestamp: t1,
            },
            Event {
                name: "Event3".to_string(),
                timestamp: t4,
            },
            Event {
                name: "Event4".to_string(),
                timestamp: t3,
            },
        ];

        let result = earliest_by(&events, |e| e.timestamp);
        assert_eq!(
            result,
            Some(Event {
                name: "Event2".to_string(),
                timestamp: t1,
            })
        );
    }

    #[test]
    fn test_earliest_by_varied_times() {
        let t1 = SystemTime::UNIX_EPOCH;
        let t2 = t1 + Duration::new(15, 0);
        let t3 = t1 + Duration::new(45, 0);
        let t4 = t1 + Duration::new(30, 0);
        let t5 = t1 + Duration::new(60, 0);
        let times = vec![t3, t2, t5, t1, t4];
        let result = earliest_by(&times, |&t| t);
        assert_eq!(result, Some(t1));
    }
}
