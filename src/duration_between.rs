use std::time::SystemTime;

/// Time units for calculating duration.
pub enum DurationUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

impl DurationUnit {
    /// Returns the number of seconds in one unit.
    fn seconds_per_unit(&self) -> u64 {
        match self {
            DurationUnit::Seconds => 1,
            DurationUnit::Minutes => 60,
            DurationUnit::Hours => 3600,
            DurationUnit::Days => 86_400,
            DurationUnit::Weeks => 604_800,
            DurationUnit::Months => 2_629_746, // Average month (30.44 days)
            DurationUnit::Years => 31_557_600, // 365.25 days
        }
    }
}

/// Returns the absolute difference between two dates in the specified unit.
///
/// The calculation is based on approximations for months and years.
///
/// # Arguments
/// * `date1` - The first date.
/// * `date2` - The second date.
/// * `unit` - The unit of time for the returned difference.
///
/// # Returns
/// * `u64` - The absolute difference between the two dates in the specified unit.
///
/// # Examples
/// ```rust
/// use std::time::{SystemTime, Duration};
/// use lowdash::{duration_between, DurationUnit};
///
/// let epoch = SystemTime::UNIX_EPOCH;
/// let one_year = Duration::from_secs(31_557_600);
/// let later = epoch + one_year;
/// // Difference in years
/// assert_eq!(duration_between(epoch, later, DurationUnit::Years), 1);
///
/// let one_day = Duration::from_secs(86_400);
/// let day_later = epoch + one_day;
/// // Difference in days
/// assert_eq!(duration_between(epoch, day_later, DurationUnit::Days), 1);
/// ```
pub fn duration_between(date1: SystemTime, date2: SystemTime, unit: DurationUnit) -> u64 {
    let duration = if date1 > date2 {
        date1.duration_since(date2).expect("Time went backwards")
    } else {
        date2.duration_since(date1).expect("Time went backwards")
    };
    duration.as_secs() / unit.seconds_per_unit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_same_date() {
        let now = SystemTime::now();
        assert_eq!(duration_between(now, now, DurationUnit::Seconds), 0);
    }

    #[test]
    fn test_minutes() {
        let epoch = SystemTime::UNIX_EPOCH;
        let one_minute = Duration::from_secs(60);
        let later = epoch + one_minute;
        assert_eq!(duration_between(epoch, later, DurationUnit::Minutes), 1);
    }

    #[test]
    fn test_hours() {
        let epoch = SystemTime::UNIX_EPOCH;
        let two_hours = Duration::from_secs(3600 * 2);
        let later = epoch + two_hours;
        assert_eq!(duration_between(epoch, later, DurationUnit::Hours), 2);
    }

    #[test]
    fn test_days() {
        let epoch = SystemTime::UNIX_EPOCH;
        let three_days = Duration::from_secs(86_400 * 3);
        let later = epoch + three_days;
        assert_eq!(duration_between(epoch, later, DurationUnit::Days), 3);
    }

    #[test]
    fn test_weeks() {
        let epoch = SystemTime::UNIX_EPOCH;
        let two_weeks = Duration::from_secs(604_800 * 2);
        let later = epoch + two_weeks;
        assert_eq!(duration_between(epoch, later, DurationUnit::Weeks), 2);
    }

    #[test]
    fn test_months() {
        let epoch = SystemTime::UNIX_EPOCH;
        let one_month = Duration::from_secs(2_629_746);
        let later = epoch + one_month;
        assert_eq!(duration_between(epoch, later, DurationUnit::Months), 1);
    }

    #[test]
    fn test_years() {
        let epoch = SystemTime::UNIX_EPOCH;
        let one_year = Duration::from_secs(31_557_600);
        let later = epoch + one_year;
        assert_eq!(duration_between(epoch, later, DurationUnit::Years), 1);
    }
}
