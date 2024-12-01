use std::time::SystemTime;

/// Returns the item from the collection for which the iteratee returns the latest `SystemTime`.
/// If the collection is empty, returns the default value of `T`.
///
/// # Arguments
/// * `collection` - A slice of items.
/// * `iteratee` - A function that takes a reference to an item and returns a `SystemTime`.
///
/// # Returns
/// * `T` - The item with the latest `SystemTime` as determined by the iteratee.
/// * If the collection is empty, returns `T::default()`.
///
/// # Examples
///
/// ```rust
/// use std::time::{SystemTime, Duration};
/// use lowdash::latest_by;
///
/// #[derive(Debug, PartialEq, Clone)]
/// struct Record {
///     id: u32,
///     timestamp: SystemTime,
/// }
///
/// impl Default for Record {
///     fn default() -> Self {
///         Record {
///             id: 0,
///             timestamp: SystemTime::UNIX_EPOCH,
///         }
///     }
/// }
///
/// let records = vec![
///     Record {
///         id: 1,
///         timestamp: SystemTime::UNIX_EPOCH + Duration::new(100, 0),
///     },
///     Record {
///         id: 2,
///         timestamp: SystemTime::UNIX_EPOCH + Duration::new(200, 0),
///     },
///     Record {
///         id: 3,
///         timestamp: SystemTime::UNIX_EPOCH + Duration::new(150, 0),
///     },
/// ];
///
/// let latest_record = latest_by(&records, |r| r.timestamp);
/// assert_eq!(latest_record.id, 2);
/// ```
pub fn latest_by<T, F>(collection: &[T], iteratee: F) -> T
where
    F: Fn(&T) -> SystemTime,
    T: Clone + Default,
{
    if collection.is_empty() {
        return T::default();
    }

    let mut latest = collection[0].clone();
    let mut latest_time = iteratee(&latest);

    for item in &collection[1..] {
        let item_time = iteratee(item);
        if item_time > latest_time {
            latest = item.clone();
            latest_time = item_time;
        }
    }

    latest
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[derive(Debug, PartialEq, Clone)]
    struct Event {
        name: String,
        time: SystemTime,
    }

    impl Default for Event {
        fn default() -> Self {
            Self {
                name: "".to_string(),
                time: UNIX_EPOCH,
            }
        }
    }

    #[test]
    fn test_latest_by() {
        let event1 = Event {
            name: "Event 1".to_string(),
            time: UNIX_EPOCH + Duration::new(100, 0),
        };
        let event2 = Event {
            name: "Event 2".to_string(),
            time: UNIX_EPOCH + Duration::new(200, 0),
        };
        let event3 = Event {
            name: "Event 3".to_string(),
            time: UNIX_EPOCH + Duration::new(150, 0),
        };
        let events = vec![event1.clone(), event2.clone(), event3.clone()];

        let latest_event = latest_by(&events, |e| e.time);
        assert_eq!(latest_event, event2);
    }

    #[test]
    fn test_latest_by_single_item() {
        let event = Event {
            name: "Only Event".to_string(),
            time: UNIX_EPOCH + Duration::new(100, 0),
        };
        let events = vec![event.clone()];

        let latest_event = latest_by(&events, |e| e.time);
        assert_eq!(latest_event, event);
    }

    #[test]
    fn test_latest_by_empty_collection() {
        let events: Vec<Event> = vec![];
        let latest_event = latest_by(&events, |e| e.time);
        assert_eq!(latest_event, Event::default());
    }

    #[test]
    fn test_latest_by_with_duplicates() {
        let event1 = Event {
            name: "Event 1".to_string(),
            time: UNIX_EPOCH + Duration::new(100, 0),
        };
        let event2 = Event {
            name: "Event 2".to_string(),
            time: UNIX_EPOCH + Duration::new(200, 0),
        };
        let event3 = Event {
            name: "Event 3".to_string(),
            time: UNIX_EPOCH + Duration::new(200, 0),
        };
        let events = vec![event1.clone(), event2.clone(), event3.clone()];

        let latest_event = latest_by(&events, |e| e.time);
        assert_eq!(latest_event, event2);
    }

    #[test]
    fn test_latest_by_mixed_order() {
        let event1 = Event {
            name: "Event 1".to_string(),
            time: UNIX_EPOCH + Duration::new(300, 0),
        };
        let event2 = Event {
            name: "Event 2".to_string(),
            time: UNIX_EPOCH + Duration::new(100, 0),
        };
        let event3 = Event {
            name: "Event 3".to_string(),
            time: UNIX_EPOCH + Duration::new(200, 0),
        };
        let events = vec![event1.clone(), event2.clone(), event3.clone()];

        let latest_event = latest_by(&events, |e| e.time);
        assert_eq!(latest_event, event1);
    }

    #[test]
    fn test_latest_by_all_same() {
        let event = Event {
            name: "Same Event".to_string(),
            time: UNIX_EPOCH + Duration::new(150, 0),
        };
        let events = vec![event.clone(), event.clone(), event.clone()];

        let latest_event = latest_by(&events, |e| e.time);
        assert_eq!(latest_event, event);
    }

    #[test]
    fn test_latest_by_with_future_times() {
        let now = SystemTime::now();
        let future1 = Event {
            name: "Future 1".to_string(),
            time: now + Duration::new(1000, 0),
        };
        let future2 = Event {
            name: "Future 2".to_string(),
            time: now + Duration::new(2000, 0),
        };
        let future3 = Event {
            name: "Future 3".to_string(),
            time: now + Duration::new(3000, 0),
        };
        let events = vec![future1.clone(), future2.clone(), future3.clone()];

        let latest_event = latest_by(&events, |e| e.time);
        assert_eq!(latest_event, future3);
    }

    #[test]
    fn test_latest_by_with_past_times() {
        let now = SystemTime::now();
        let past1 = Event {
            name: "Past 1".to_string(),
            time: now - Duration::new(3000, 0),
        };
        let past2 = Event {
            name: "Past 2".to_string(),
            time: now - Duration::new(2000, 0),
        };
        let past3 = Event {
            name: "Past 3".to_string(),
            time: now - Duration::new(1000, 0),
        };
        let events = vec![past1.clone(), past2.clone(), past3.clone()];

        let latest_event = latest_by(&events, |e| e.time);
        assert_eq!(latest_event, past3);
    }

    #[test]
    fn test_latest_by_with_structs() {
        let event1 = Event {
            name: "Alpha".to_string(),
            time: UNIX_EPOCH + Duration::new(100, 0),
        };
        let event2 = Event {
            name: "Beta".to_string(),
            time: UNIX_EPOCH + Duration::new(200, 0),
        };
        let event3 = Event {
            name: "Gamma".to_string(),
            time: UNIX_EPOCH + Duration::new(300, 0),
        };
        let events = vec![event1.clone(), event2.clone(), event3.clone()];

        let latest_event = latest_by(&events, |e| e.time);
        assert_eq!(latest_event, event3);
    }
}
