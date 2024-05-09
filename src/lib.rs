//! A library for formatting time-related values in a human-friendly way.

pub mod duration;

use duration::FolkDuration;

/// A library for formatting time-related values in a human-friendly way.
///
/// # Example
/// ```
/// use folktime::Folktime;
/// use std::time::Duration;
///
/// let d = Folktime::duration(Duration::from_secs(5));
/// assert_eq!(format!("{}", d), "5s 0ms");
///
/// let s = d.to_string();
/// assert_eq!(s, "5s 0ms");
pub struct Folktime;

impl Folktime {
    /// Used for formatting [std::time::Duration] in a human-friendly way.
    ///
    /// # Example
    /// ```
    /// use folktime::Folktime;
    /// use std::time::Duration;
    ///
    /// let d = Folktime::duration(Duration::from_secs(5));
    /// assert_eq!(format!("{}", d), "5s 0ms");
    ///
    /// let s = d.to_string();
    /// assert_eq!(s, "5s 0ms");
    /// ```
    ///
    /// # Note
    /// Formatting only shows the most significant time units.
    /// ```
    /// use folktime::Folktime;
    /// use std::time::Duration;
    ///
    /// let d = Folktime::duration(Duration::new(1, 123_456_789));
    /// assert_eq!(format!("{}", d), "1s 123ms");
    ///
    /// let d = Folktime::duration(Duration::new(0, 123_456_789));
    /// assert_eq!(format!("{}", d), "123ms 456us");
    /// ```
    ///
    /// # See also
    /// [Folktime::duration_custom]
    pub const fn duration(d: std::time::Duration) -> FolkDuration {
        FolkDuration(d, const_default::ConstDefault::DEFAULT)
    }

    /// Used for formatting [std::time::Duration] in a human-friendly way.
    ///
    /// # Example
    /// ```
    /// use folktime::Folktime;
    /// use std::time::Duration;
    /// use folktime::duration::SignificantUnits;
    ///
    /// let d = Folktime::duration_custom(Duration::from_secs(5), SignificantUnits::One);
    /// assert_eq!(format!("{}", d), "5s");
    ///
    /// let s = d.to_string();
    /// assert_eq!(s, "5s");
    /// ```
    ///
    /// # Note
    /// Formatting only shows the most significant time units.
    /// ```
    /// use folktime::Folktime;
    /// use std::time::Duration;
    /// use folktime::duration::SignificantUnits;
    ///
    /// let d = Folktime::duration_custom(Duration::new(1, 123_456_789), SignificantUnits::One);
    /// assert_eq!(format!("{}", d), "1s");
    ///
    /// let d = Folktime::duration_custom(Duration::new(0, 123_456_789), SignificantUnits::One);
    /// assert_eq!(format!("{}", d), "123ms");
    /// ```
    ///
    /// # See also
    /// [Folktime::duration_custom]
    pub const fn duration_custom(
        d: std::time::Duration,
        units: duration::SignificantUnits,
    ) -> FolkDuration {
        FolkDuration(d, units)
    }
}
