//! A library for formatting time-related values in a human-friendly way.

pub mod duration;

use duration::Duration;

/// A library for formatting time-related values in a human-friendly way.
///
/// # Example
/// ```
/// use std::time::Duration;
/// use folktime::Folktime;
///
/// let d = Folktime::duration(Duration::from_secs(5));
/// assert_eq!(format!("{}", d), "5.00s");
///
/// let s = d.to_string();
/// assert_eq!(s, "5.00s");
/// ```
pub struct Folktime;

impl Folktime {
    /// Used for formatting [std::time::Duration] in a human-friendly way.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    ///
    /// let d = Folktime::duration(Duration::from_secs(5));
    /// assert_eq!(format!("{}", d), "5.00s");
    ///
    /// let s = d.to_string();
    /// assert_eq!(s, "5.00s");
    /// ```
    ///
    /// # Note
    /// Formatting only shows the most significant digits:
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    ///
    /// let d = Folktime::duration(Duration::new(0, 123_456_789));
    /// assert_eq!(format!("{}", d), "123ms");
    ///
    /// let d = Folktime::duration(Duration::new(1, 123_456_789));
    /// assert_eq!(format!("{}", d), "1.12s");
    ///
    /// let d = Folktime::duration(Duration::new(12, 123_456_789));
    /// assert_eq!(format!("{}", d), "12.1s");
    ///
    /// let d = Folktime::duration(Duration::new(123, 123_456_789));
    /// assert_eq!(format!("{}", d), "2.05m");
    /// ```
    ///
    /// # Formatting
    /// There are several styles for formatting:
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::new(0, 12_056_999));
    /// assert_eq!(format!("{}", d), "12.0ms");
    ///
    /// let d = Folktime::duration(Duration::new(0, 12_056_999)).with_style(Style::OneUnitWhole);
    /// assert_eq!(format!("{}", d), "12ms");
    ///
    /// let d = Folktime::duration(Duration::new(0, 12_056_999)).with_style(Style::TwoUnitsWhole);
    /// assert_eq!(format!("{}", d), "12ms 56us");
    /// ```
    pub const fn duration(d: std::time::Duration) -> Duration {
        Duration(d, duration::Style::OneUnitThreeDigits)
    }
}
