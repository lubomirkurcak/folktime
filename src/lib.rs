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
/// ```
pub struct Folktime;

impl Folktime {
    /// Used for formatting [std::time::Duration] in a human-friendly way.
    ///
    /// # Example
    /// ```rust
    /// use std::time::Duration;
    /// use folktime::Folktime;
    ///
    /// let d = Folktime::duration(Duration::from_secs(5));
    /// assert_eq!(format!("{}", d), "5.00s");
    /// ```
    ///
    /// # Precision
    ///
    /// Formatting only shows the most significant digits:
    /// ```rust
    /// use std::time::Duration;
    /// use folktime::Folktime;
    ///  
    /// let a = Folktime::duration(Duration::new(0, 123_456_789));
    /// let b = Folktime::duration(Duration::new(1, 123_456_789));
    /// let c = Folktime::duration(Duration::new(12, 123_456_789));
    /// let d = Folktime::duration(Duration::new(123, 123_456_789));
    ///
    /// assert_eq!(format!("{}", a), "123ms");
    /// assert_eq!(format!("{}", b), "1.12s");
    /// assert_eq!(format!("{}", c), "12.1s");
    /// assert_eq!(format!("{}", d), "2.05m");
    /// ```
    ///
    /// # Formatting styles
    /// There are several styles for formatting:
    /// ```rust
    /// use std::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    ///
    /// let a = Folktime::duration(Duration::new(0, 12_056_999));
    /// let b = Folktime::duration(Duration::new(0, 12_056_999)).with_style(Style::OneUnitWhole);
    /// let c = Folktime::duration(Duration::new(0, 12_056_999)).with_style(Style::TwoUnitsWhole);
    ///
    /// assert_eq!(format!("{}", a), "12.0ms");
    /// assert_eq!(format!("{}", b), "12ms");
    /// assert_eq!(format!("{}", c), "12ms 56us");
    /// ```
    pub const fn duration(d: std::time::Duration) -> Duration {
        Duration(d, duration::Style::OneUnitFractional)
    }
}
