pub mod one_unit_fractional;
pub mod one_unit_whole;
pub mod two_units_whole;

use std::fmt::Display;

const MIN: u64 = 60;
const HOUR: u64 = 60 * MIN;
const DAY: u64 = 24 * HOUR;
const WEEK: u64 = 7 * DAY;
const MONTH: u64 = 2629846; // = ceil(365.256363004*24*60*60/12)
const YEAR: u64 = 31558150; // = ceil(365.256363004*24*60*60)
const KILO_YEAR: u64 = 1_000 * YEAR;
const MEGA_YEAR: u64 = 1_000 * KILO_YEAR;
const GIGA_YEAR: u64 = 1_000 * MEGA_YEAR;

const US: u32 = 1_000;
const MS: u32 = 1_000 * US;

#[derive(Default)]
pub enum Style {
    #[default]
    OneUnitFractional,
    OneUnitWhole,
    TwoUnitsWhole,
}

pub struct Duration(pub std::time::Duration, pub Style);

impl Duration {
    pub fn new(d: std::time::Duration) -> Self {
        Self(d, Default::default())
    }

    pub fn with_style(self, units: Style) -> Self {
        Self(self.0, units)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.1 {
            Style::OneUnitFractional => self.fmt_one_unit_fractional(f),
            Style::OneUnitWhole => self.fmt_one_unit_whole(f),
            Style::TwoUnitsWhole => self.fmt_two_units_whole(f),
        }
    }
}
