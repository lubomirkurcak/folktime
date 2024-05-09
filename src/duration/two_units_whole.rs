use super::{Duration, DAY, HOUR, MIN, MONTH, MS, US, WEEK, YEAR};

impl Duration {
    pub fn fmt_two_units_whole(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let secs = self.0.as_secs();
        let ns = self.0.subsec_nanos();

        if secs < 1 {
            if ns < US {
                if ns == 0 {
                    write!(f, "0s 0ms")
                } else {
                    write!(f, "{}ns", ns)
                }
            } else if ns < MS {
                let us = ns / US;
                let ns = ns % US;
                write!(f, "{us}us {ns}ns")
            } else {
                let ms = ns / MS;
                let us = (ns % MS) / US;
                write!(f, "{ms}ms {us}us")
            }
        } else if secs < MIN {
            let ms = ns / 1_000_000;
            write!(f, "{secs}s {ms}ms")
        } else if secs < HOUR {
            let mins = secs / MIN;
            let secs = secs % MIN;
            write!(f, "{mins}m {secs}s")
        } else if secs < DAY {
            let hours = secs / HOUR;
            let mins = (secs % HOUR) / MIN;
            write!(f, "{hours}h {mins}m")
        } else if secs < WEEK {
            let days = secs / DAY;
            let hours = (secs % DAY) / HOUR;
            write!(f, "{days}d {hours}h")
        } else if secs < MONTH {
            let weeks = secs / WEEK;
            let days = (secs % WEEK) / DAY;
            write!(f, "{weeks}w {days}d")
        } else if secs < YEAR {
            let months = secs / MONTH;
            let days = (secs % MONTH) / DAY;
            write!(f, "{months}mo {days}d")
        } else {
            let years = secs / YEAR;
            let months = (secs % YEAR) / MONTH;
            write!(f, "{years}y {months}mo")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{duration::Style, Folktime};

    const STYLE: Style = Style::TwoUnitsWhole;

    #[test]
    fn zero() {
        let d = Folktime::duration(std::time::Duration::new(0, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "0s 0ms");
    }
    #[test]
    fn max() {
        let d =
            Folktime::duration(std::time::Duration::new(u64::MAX, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "584531858607y 4mo");
    }
    #[test]
    fn test() {
        let d = Folktime::duration(std::time::Duration::from_secs(12345689)).with_style(STYLE);
        assert_eq!(format!("{}", d), "4mo 21d");
    }
    #[test]
    fn test2() {
        let d = Folktime::duration(std::time::Duration::from_secs(1234568)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2w 0d");
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
        assert_eq!(format!("{}", d), "1us 0ns");
    }
    #[test]
    fn us_1() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_001)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1us 1ns");
    }
    #[test]
    fn us_2() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1us 999ns");
    }
    #[test]
    fn us_3() {
        let d = Folktime::duration(std::time::Duration::new(0, 999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "999us 999ns");
    }

    #[test]
    fn ms_0() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_000_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1ms 0us");
    }
    #[test]
    fn ms_1() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_000_001)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1ms 0us");
    }
    #[test]
    fn ms_2() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_000_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1ms 0us");
    }
    #[test]
    fn ms_3() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_001_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1ms 1us");
    }
    #[test]
    fn ms_4() {
        let d = Folktime::duration(std::time::Duration::new(0, 1_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1ms 999us");
    }
    #[test]
    fn ms_5() {
        let d = Folktime::duration(std::time::Duration::new(0, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "999ms 999us");
    }

    #[test]
    fn s_0() {
        let d = Folktime::duration(std::time::Duration::new(1, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1s 0ms");
    }
    #[test]
    fn s_1() {
        let d = Folktime::duration(std::time::Duration::new(1, 999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1s 0ms");
    }
    #[test]
    fn s_2() {
        let d = Folktime::duration(std::time::Duration::new(1, 1_000_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1s 1ms");
    }
    #[test]
    fn s_3() {
        let d = Folktime::duration(std::time::Duration::new(59, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "59s 999ms");
    }

    #[test]
    fn m_0() {
        let d = Folktime::duration(std::time::Duration::new(60, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1m 0s");
    }
    #[test]
    fn m_1() {
        let d = Folktime::duration(std::time::Duration::new(60, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1m 0s");
    }
    #[test]
    fn m_2() {
        let d = Folktime::duration(std::time::Duration::new(61, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1m 1s");
    }
    #[test]
    fn m_3() {
        let d =
            Folktime::duration(std::time::Duration::new(HOUR - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "59m 59s");
    }

    #[test]
    fn h_0() {
        let d = Folktime::duration(std::time::Duration::new(HOUR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1h 0m");
    }
    #[test]
    fn h_1() {
        let d = Folktime::duration(std::time::Duration::new(HOUR + MIN - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1h 0m");
    }
    #[test]
    fn h_2() {
        let d = Folktime::duration(std::time::Duration::new(HOUR + MIN, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1h 1m");
    }
    #[test]
    fn h_3() {
        let d =
            Folktime::duration(std::time::Duration::new(DAY - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "23h 59m");
    }

    #[test]
    fn d_0() {
        let d = Folktime::duration(std::time::Duration::new(DAY, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1d 0h");
    }
    #[test]
    fn d_1() {
        let d = Folktime::duration(std::time::Duration::new(DAY + HOUR - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1d 0h");
    }
    #[test]
    fn d_2() {
        let d = Folktime::duration(std::time::Duration::new(DAY + HOUR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1d 1h");
    }
    #[test]
    fn d_3() {
        let d =
            Folktime::duration(std::time::Duration::new(WEEK - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "6d 23h");
    }

    #[test]
    fn w_0() {
        let d = Folktime::duration(std::time::Duration::new(WEEK, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1w 0d");
    }
    #[test]
    fn w_1() {
        let d = Folktime::duration(std::time::Duration::new(WEEK + DAY - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1w 0d");
    }
    #[test]
    fn w_2() {
        let d = Folktime::duration(std::time::Duration::new(WEEK + DAY, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1w 1d");
    }
    #[test]
    fn w_3() {
        let d =
            Folktime::duration(std::time::Duration::new(MONTH - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "4w 2d");
    }

    #[test]
    fn mo_0() {
        let d = Folktime::duration(std::time::Duration::new(MONTH, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1mo 0d");
    }
    #[test]
    fn mo_1() {
        let d = Folktime::duration(std::time::Duration::new(MONTH + DAY - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1mo 0d");
    }
    #[test]
    fn mo_2() {
        let d = Folktime::duration(std::time::Duration::new(MONTH + DAY, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1mo 1d");
    }
    #[test]
    fn mo_3() {
        let d =
            Folktime::duration(std::time::Duration::new(YEAR - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "11mo 30d");
    }

    #[test]
    fn y_0() {
        let d = Folktime::duration(std::time::Duration::new(YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1y 0mo");
    }
    #[test]
    fn y_1() {
        let d = Folktime::duration(std::time::Duration::new(YEAR + MONTH - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1y 0mo");
    }
    #[test]
    fn y_2() {
        let d = Folktime::duration(std::time::Duration::new(YEAR + MONTH, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1y 1mo");
    }
    #[test]
    fn y_3() {
        let d = Folktime::duration(std::time::Duration::new(2 * YEAR - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1y 11mo");
    }
    #[test]
    fn y_4() {
        let d = Folktime::duration(std::time::Duration::new(2 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2y 0mo");
    }
    #[test]
    fn y_5() {
        let d = Folktime::duration(std::time::Duration::new(1000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1000y 0mo");
    }
}
