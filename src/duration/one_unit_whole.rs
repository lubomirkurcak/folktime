use super::{Duration, DAY, HOUR, MIN, MONTH, MS, US, WEEK, YEAR};

impl Duration {
    pub fn fmt_one_unit_whole(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let secs = self.0.as_secs();
        let ns = self.0.subsec_nanos();

        if secs < 1 {
            if ns < US {
                if ns == 0 {
                    write!(f, "0s")
                } else {
                    write!(f, "{ns}ns")
                }
            } else if ns < MS {
                let us = ns / US;
                write!(f, "{us}us")
            } else {
                let ms = ns / MS;
                write!(f, "{ms}ms")
            }
        } else if secs < MIN {
            write!(f, "{secs}s")
        } else if secs < HOUR {
            let mins = secs / MIN;
            write!(f, "{mins}m")
        } else if secs < DAY {
            let hours = secs / HOUR;
            write!(f, "{hours}h")
        } else if secs < WEEK {
            let days = secs / DAY;
            write!(f, "{days}d")
        } else if secs < MONTH {
            let weeks = secs / WEEK;
            write!(f, "{weeks}w")
        } else if secs < YEAR {
            let months = secs / MONTH;
            write!(f, "{months}mo")
        } else {
            let years = secs / YEAR;
            write!(f, "{years}y")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{duration::Style, Folktime};

    const STYLE: Style = Style::OneUnitWhole;

    #[test]
    fn zero() {
        let d = Folktime::duration(std::time::Duration::new(0, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "0s");
    }
    #[test]
    fn max() {
        let d =
            Folktime::duration(std::time::Duration::new(u64::MAX, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "584531858607y");
    }
    #[test]
    fn test() {
        let d = Folktime::duration(std::time::Duration::from_secs(12345689)).with_style(STYLE);
        assert_eq!(format!("{}", d), "4mo");
    }
    #[test]
    fn test2() {
        let d = Folktime::duration(std::time::Duration::from_secs(1234568)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2w");
    }

    #[test]
    fn ns_0() {
        let d = Folktime::duration(std::time::Duration::new(0, 1)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1ns");
    }
    #[test]
    fn ns_1() {
        let d = Folktime::duration(std::time::Duration::new(0, 999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "999ns");
    }

    #[test]
    fn us_0() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1us");
    }
    #[test]
    fn us_1() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_001)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1us");
    }
    #[test]
    fn us_2() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1us");
    }
    #[test]
    fn us_3() {
        let d = Folktime::duration(std::time::Duration::new(0, 999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "999us");
    }

    #[test]
    fn ms_0() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_000_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1ms");
    }
    #[test]
    fn ms_1() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_000_001)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1ms");
    }
    #[test]
    fn ms_2() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_000_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1ms");
    }
    #[test]
    fn ms_3() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_001_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1ms");
    }
    #[test]
    fn ms_4() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1ms");
    }
    #[test]
    fn ms_5() {
        let d = Folktime::duration(std::time::Duration::new(0, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "999ms");
    }

    #[test]
    fn s_0() {
        let d = Folktime::duration(std::time::Duration::new(1, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1s");
    }
    #[test]
    fn s_1() {
        let d = Folktime::duration(std::time::Duration::new(1, 999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1s");
    }
    #[test]
    fn s_2() {
        let d = Folktime::duration(std::time::Duration::new(1, 1_000_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1s");
    }
    #[test]
    fn s_3() {
        let d = Folktime::duration(std::time::Duration::new(59, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "59s");
    }

    #[test]
    fn m_0() {
        let d = Folktime::duration(std::time::Duration::new(60, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1m");
    }
    #[test]
    fn m_1() {
        let d = Folktime::duration(std::time::Duration::new(60, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1m");
    }
    #[test]
    fn m_2() {
        let d = Folktime::duration(std::time::Duration::new(61, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1m");
    }
    #[test]
    fn m_3() {
        let d =
            Folktime::duration(std::time::Duration::new(HOUR - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "59m");
    }

    #[test]
    fn h_0() {
        let d = Folktime::duration(std::time::Duration::new(HOUR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1h");
    }
    #[test]
    fn h_1() {
        let d = Folktime::duration(std::time::Duration::new(HOUR + MIN - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1h");
    }
    #[test]
    fn h_2() {
        let d = Folktime::duration(std::time::Duration::new(HOUR + MIN, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1h");
    }
    #[test]
    fn h_3() {
        let d =
            Folktime::duration(std::time::Duration::new(DAY - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "23h");
    }

    #[test]
    fn d_0() {
        let d = Folktime::duration(std::time::Duration::new(DAY, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1d");
    }
    #[test]
    fn d_1() {
        let d = Folktime::duration(std::time::Duration::new(DAY + HOUR - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1d");
    }
    #[test]
    fn d_2() {
        let d = Folktime::duration(std::time::Duration::new(DAY + HOUR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1d");
    }
    #[test]
    fn d_3() {
        let d =
            Folktime::duration(std::time::Duration::new(WEEK - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "6d");
    }

    #[test]
    fn w_0() {
        let d = Folktime::duration(std::time::Duration::new(WEEK, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1w");
    }
    #[test]
    fn w_1() {
        let d = Folktime::duration(std::time::Duration::new(WEEK + DAY - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1w");
    }
    #[test]
    fn w_2() {
        let d = Folktime::duration(std::time::Duration::new(WEEK + DAY, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1w");
    }
    #[test]
    fn w_3() {
        let d =
            Folktime::duration(std::time::Duration::new(MONTH - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "4w");
    }

    #[test]
    fn mo_0() {
        let d = Folktime::duration(std::time::Duration::new(MONTH, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1mo");
    }
    #[test]
    fn mo_1() {
        let d = Folktime::duration(std::time::Duration::new(MONTH + DAY - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1mo");
    }
    #[test]
    fn mo_2() {
        let d = Folktime::duration(std::time::Duration::new(MONTH + DAY, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1mo");
    }
    #[test]
    fn mo_3() {
        let d =
            Folktime::duration(std::time::Duration::new(YEAR - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "11mo");
    }

    #[test]
    fn y_0() {
        let d = Folktime::duration(std::time::Duration::new(YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1y");
    }
    #[test]
    fn y_1() {
        let d = Folktime::duration(std::time::Duration::new(YEAR + MONTH - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1y");
    }
    #[test]
    fn y_2() {
        let d = Folktime::duration(std::time::Duration::new(YEAR + MONTH, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1y");
    }
    #[test]
    fn y_3() {
        let d = Folktime::duration(std::time::Duration::new(2 * YEAR - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1y");
    }
    #[test]
    fn y_4() {
        let d = Folktime::duration(std::time::Duration::new(2 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2y");
    }
    #[test]
    fn y_5() {
        let d = Folktime::duration(std::time::Duration::new(1000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1000y");
    }
}
