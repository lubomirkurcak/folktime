pub mod one_unit_frac;
pub mod one_unit_whole;
pub mod two_units_whole;

use std::fmt::Display;

const MIN: u64 = 60;
const HOUR: u64 = 60 * MIN;
const DAY: u64 = 24 * HOUR;
const WEEK: u64 = 7 * DAY;
const MONTH: u64 = 2_629_846; // = ceil(365.256363004*24*60*60/12)
const YEAR: u64 = 31_558_150; // = ceil(365.256363004*24*60*60)
const KILO_YEAR: u64 = 1_000 * YEAR;
const MEGA_YEAR: u64 = 1_000 * KILO_YEAR;
const GIGA_YEAR: u64 = 1_000 * MEGA_YEAR;

const US: u32 = 1_000;
const MS: u32 = 1_000 * US;

#[derive(Default)]
/// Formatting style for [std::time::Duration].
pub enum Style {
    #[default]
    /// Format the duration in the largest possible unit with a fractional part with 3 significant digits.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::OneUnitFrac);
    /// assert_eq!(format!("{}", d), "2.05m");
    OneUnitFrac,
    /// Format the duration in the largest possible unit with a whole number.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::OneUnitWhole);
    /// assert_eq!(format!("{}", d), "2m");
    OneUnitWhole,
    /// Format the duration in the two largest possible units with whole numbers.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::TwoUnitsWhole);
    /// assert_eq!(format!("{}", d), "2m 3s");
    /// ```
    TwoUnitsWhole,
}

pub struct Duration(pub std::time::Duration, pub Style);

impl Duration {
    pub fn new(d: std::time::Duration) -> Self {
        Self(d, Default::default())
    }

    /// Set the formatting style.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::TwoUnitsWhole);
    /// assert_eq!(format!("{}", d), "2m 3s");
    /// ```
    pub fn with_style(self, units: Style) -> Self {
        Self(self.0, units)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.1 {
            Style::OneUnitFrac => self.fmt_one_unit_frac(f),
            Style::OneUnitWhole => self.fmt_one_unit_whole(f),
            Style::TwoUnitsWhole => self.fmt_two_units_whole(f),
        }
    }
}
