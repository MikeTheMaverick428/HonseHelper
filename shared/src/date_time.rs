use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

/// An inclusive after/before timestamp range used by date filters.
///
/// Values are stored as normalized `"YYYY-MM-DD HH:MM:SS"` strings (UTC-naive),
/// matching how `capture_time`/`created_at` are stored in the app database so
/// lexicographic comparison works.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DateTimeRange {
    pub after: Option<String>,
    pub before: Option<String>,
}

impl DateTimeRange {
    pub fn is_empty(&self) -> bool {
        self.after.is_none() && self.before.is_none()
    }

    pub fn has_value(&self) -> bool {
        !self.is_empty()
    }

    /// True when a value is set and the range is not inverted.
    pub fn is_valid(&self) -> bool {
        match (&self.after, &self.before) {
            (Some(a), Some(b)) => a <= b,
            _ => true,
        }
    }
}

/// Normalize a raw date / date-time string into `"YYYY-MM-DD HH:MM:SS"`.
///
/// Accepted inputs:
/// - `"2025-01-01"` — date only; resolves to `00:00:00` for `after`, `23:59:59` for `before`
/// - `"2025-01-01 14:30"`, `"2025-01-01 14:30:45"` — space separator
/// - `"2025-01-01T14:30"`, `"2025-01-01T14:30:45"` — `T` separator (e.g. `datetime-local`)
///
/// Returns `None` for empty or unparseable input.
pub fn normalize_bound(raw: &str, is_after: bool) -> Option<String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }

    let datetime = parse_naive(raw);
    let dt = match datetime {
        Some(dt) => dt,
        None => {
            let date = NaiveDate::parse_from_str(raw, "%Y-%m-%d").ok()?;
            let time = if is_after {
                NaiveTime::from_hms_opt(0, 0, 0).unwrap()
            } else {
                NaiveTime::from_hms_opt(23, 59, 59).unwrap()
            };
            date.and_time(time)
        }
    };

    Some(dt.format("%Y-%m-%d %H:%M:%S").to_string())
}

fn parse_naive(raw: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M"))
        .or_else(|_| NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%S"))
        .or_else(|_| NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M"))
        .ok()
}

/// Build a `DateTimeRange` from raw after/before strings.
pub fn range_from_raw(after: Option<String>, before: Option<String>) -> DateTimeRange {
    DateTimeRange {
        after: after.as_deref().and_then(|v| normalize_bound(v, true)),
        before: before.as_deref().and_then(|v| normalize_bound(v, false)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_only_after_starts_at_midnight() {
        assert_eq!(normalize_bound("2025-01-01", true), Some("2025-01-01 00:00:00".into()));
    }

    #[test]
    fn date_only_before_ends_at_day_end() {
        assert_eq!(normalize_bound("2025-01-01", false), Some("2025-01-01 23:59:59".into()));
    }

    #[test]
    fn datetime_space_separator() {
        assert_eq!(normalize_bound("2025-01-01 14:30", true), Some("2025-01-01 14:30:00".into()));
        assert_eq!(normalize_bound("2025-01-01 14:30:45", false), Some("2025-01-01 14:30:45".into()));
    }

    #[test]
    fn datetime_t_separator() {
        assert_eq!(normalize_bound("2025-01-01T14:30", true), Some("2025-01-01 14:30:00".into()));
        assert_eq!(normalize_bound("2025-01-01T14:30:45", false), Some("2025-01-01 14:30:45".into()));
    }

    #[test]
    fn empty_and_invalid_are_none() {
        assert_eq!(normalize_bound("", true), None);
        assert_eq!(normalize_bound("  ", false), None);
        assert_eq!(normalize_bound("not-a-date", true), None);
    }

    #[test]
    fn range_from_raw_combines_bounds() {
        let r = range_from_raw(Some("2025-01-01".into()), Some("2025-01-02 10:00".into()));
        assert_eq!(r.after.as_deref(), Some("2025-01-01 00:00:00"));
        assert_eq!(r.before.as_deref(), Some("2025-01-02 10:00:00"));
        assert!(r.has_value() && r.is_valid());
    }

    #[test]
    fn inverted_range_is_invalid() {
        let r = range_from_raw(Some("2025-02-01".into()), Some("2025-01-01".into()));
        assert!(!r.is_valid());
    }

    #[test]
    fn empty_range() {
        let r = range_from_raw(None, None);
        assert!(r.is_empty() && !r.has_value() && r.is_valid());
    }
}
