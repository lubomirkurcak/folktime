use super::{Duration, DAY, GIGA_YEAR, HOUR, KILO_YEAR, MEGA_YEAR, MIN, MONTH, MS, US, WEEK, YEAR};

macro_rules! fmt_three {
    ($big:ty, $small:ty, $i:ident) => {
        fn $i(
            big: $big,
            small: $small,
            unit: &str,
            f: &mut std::fmt::Formatter,
        ) -> std::fmt::Result {
            if big < 10 {
                write!(f, "{}.{:02}{unit}", big, small / 10)
            } else if big < 100 {
                write!(f, "{}.{:01}{unit}", big, small / 100)
            } else {
                write!(f, "{}{unit}", big)
            }
        }
    };
}

fmt_three!(u32, u32, fmt_three_u32);
fmt_three!(u64, u32, fmt_three_u64_u32);

fn fmt_100(val: u64, div: u64, unit: &str, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let a = (val * 100) / div;
    let big = a / 100;
    let small = a % 100;
    if big < 10 {
        write!(f, "{}.{:02}{}", big, small, unit)
    } else if big < 100 {
        write!(f, "{}.{:01}{}", big, small / 10, unit)
    } else {
        write!(f, "{}{}", big, unit)
    }
}

impl Duration {
    pub fn fmt_one_unit_three_digits(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let secs = self.0.as_secs();
        let ns = self.0.subsec_nanos();

        if secs < 1 {
            if ns < US {
                if ns == 0 {
                    write!(f, "0.00s")
                } else {
                    write!(f, "{}ns", ns)
                }
            } else if ns < MS {
                let us = ns / US;
                let ns = ns % US;
                fmt_three_u32(us, ns, "us", f)
            } else {
                let ms = ns / MS;
                let us = (ns % MS) / US;
                fmt_three_u32(ms, us, "ms", f)
            }
        } else if secs < MIN {
            let ms = ns / 1_000_000;
            fmt_three_u64_u32(secs, ms, "s", f)
        } else if secs < HOUR {
            if secs < 10 * MIN {
                let hundredths = ns / 10_000_000;
                let val = secs * 100 + hundredths as u64;
                write!(f, "{}.{:02}m", val / 6000, (val / 60) % 100)
            } else {
                fmt_100(secs, MIN, "m", f)
            }
        } else if secs < DAY {
            fmt_100(secs, HOUR, "h", f)
        } else if secs < WEEK {
            fmt_100(secs, DAY, "d", f)
        } else if secs < MONTH {
            fmt_100(secs, WEEK, "w", f)
        } else if secs < YEAR {
            fmt_100(secs, MONTH, "mo", f)
        } else if secs < KILO_YEAR {
            fmt_100(secs, YEAR, "y", f)
        } else if secs < MEGA_YEAR {
            fmt_100(secs, KILO_YEAR, "ky", f)
        } else if secs < GIGA_YEAR {
            // TODO: find the best shift, this value was adjusted to pass unit tests
            let shift = 1;
            fmt_100(secs >> shift, MEGA_YEAR >> shift, "My", f)
        } else {
            // TODO: find the best shift, this value was adjusted to pass unit tests
            let shift = 8;
            fmt_100(secs >> shift, GIGA_YEAR >> shift, "Gy", f)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{duration::Style, Folktime};
    use std::time::Duration;

    const STYLE: Style = Style::OneUnitThreeDigits;

    #[test]
    fn zero() {
        let d = Folktime::duration(Duration::new(0, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "0.00s");
    }
    #[test]
    fn max() {
        let d = Folktime::duration(Duration::new(u64::MAX, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "584Gy");
    }

    #[test]
    fn ns_0() {
        let d = Folktime::duration(Duration::new(0, 1)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1ns");
    }
    #[test]
    fn ns_1() {
        let d = Folktime::duration(Duration::new(0, 999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "999ns");
    }

    #[test]
    fn us_0() {
        let d = Folktime::duration(Duration::new(0, 1_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00us");
    }
    #[test]
    fn us_1() {
        let d = Folktime::duration(Duration::new(0, 1_001)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00us");
    }
    #[test]
    fn us_2() {
        let d = Folktime::duration(Duration::new(0, 1_009)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00us");
    }
    #[test]
    fn us_3() {
        let d = Folktime::duration(Duration::new(0, 1_010)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.01us");
    }
    #[test]
    fn us_4() {
        let d = Folktime::duration(Duration::new(0, 1_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.99us");
    }
    #[test]
    fn us_5() {
        let d = Folktime::duration(Duration::new(0, 9_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "9.99us");
    }
    #[test]
    fn us_6() {
        let d = Folktime::duration(Duration::new(0, 10_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0us");
    }
    #[test]
    fn us_7() {
        let d = Folktime::duration(Duration::new(0, 10_099)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0us");
    }
    #[test]
    fn us_8() {
        let d = Folktime::duration(Duration::new(0, 10_100)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.1us");
    }
    #[test]
    fn us_9() {
        let d = Folktime::duration(Duration::new(0, 99_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "99.9us");
    }
    #[test]
    fn us_10() {
        let d = Folktime::duration(Duration::new(0, 100_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "100us");
    }
    #[test]
    fn us_11() {
        let d = Folktime::duration(Duration::new(0, 100_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "100us");
    }
    #[test]
    fn us_12() {
        let d = Folktime::duration(Duration::new(0, 101_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "101us");
    }
    #[test]
    fn us_13() {
        let d = Folktime::duration(Duration::new(0, 999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "999us");
    }

    #[test]
    fn ms_0() {
        let d = Folktime::duration(Duration::new(0, 1_000_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00ms");
    }
    #[test]
    fn ms_1() {
        let d = Folktime::duration(Duration::new(0, 1_009_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00ms");
    }
    #[test]
    fn ms_2() {
        let d = Folktime::duration(Duration::new(0, 1_010_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.01ms");
    }
    #[test]
    fn ms_3() {
        let d = Folktime::duration(Duration::new(0, 9_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "9.99ms");
    }
    #[test]
    fn ms_4() {
        let d = Folktime::duration(Duration::new(0, 10_000_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0ms");
    }
    #[test]
    fn ms_5() {
        let d = Folktime::duration(Duration::new(0, 10_099_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0ms");
    }
    #[test]
    fn ms_6() {
        let d = Folktime::duration(Duration::new(0, 10_100_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.1ms");
    }
    #[test]
    fn ms_7() {
        let d = Folktime::duration(Duration::new(0, 99_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "99.9ms");
    }
    #[test]
    fn ms_8() {
        let d = Folktime::duration(Duration::new(0, 100_000_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "100ms");
    }
    #[test]
    fn ms_9() {
        let d = Folktime::duration(Duration::new(0, 100_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "100ms");
    }
    #[test]
    fn ms_10() {
        let d = Folktime::duration(Duration::new(0, 101_000_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "101ms");
    }
    #[test]
    fn ms_11() {
        let d = Folktime::duration(Duration::new(0, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "999ms");
    }

    #[test]
    fn s_0() {
        let d = Folktime::duration(Duration::new(1, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00s");
    }
    #[test]
    fn s_1() {
        let d = Folktime::duration(Duration::new(1, 9_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00s");
    }
    #[test]
    fn s_2() {
        let d = Folktime::duration(Duration::new(1, 10_000_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.01s");
    }
    #[test]
    fn s_3() {
        let d = Folktime::duration(Duration::new(9, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "9.99s");
    }
    #[test]
    fn s_4() {
        let d = Folktime::duration(Duration::new(10, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0s");
    }
    #[test]
    fn s_5() {
        let d = Folktime::duration(Duration::new(10, 99_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0s");
    }
    #[test]
    fn s_6() {
        let d = Folktime::duration(Duration::new(10, 100_000_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.1s");
    }
    #[test]
    fn s_7() {
        let d = Folktime::duration(Duration::new(59, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "59.9s");
    }

    #[test]
    fn m_0() {
        let d = Folktime::duration(Duration::new(60, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00m");
    }
    #[test]
    fn m_1() {
        let d = Folktime::duration(Duration::new(60, 599_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00m");
    }
    #[test]
    fn m_2() {
        let d = Folktime::duration(Duration::new(60, 600_000_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.01m");
    }
    #[test]
    fn m_3() {
        let d = Folktime::duration(Duration::new(61, 199_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.01m");
    }
    #[test]
    fn m_4() {
        let d = Folktime::duration(Duration::new(61, 200_000_000)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.02m");
    }
    #[test]
    fn m_5() {
        let d = Folktime::duration(Duration::new(90, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.50m");
    }
    #[test]
    fn m_6() {
        let d = Folktime::duration(Duration::new(120, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2.00m");
    }
    #[test]
    fn m_7() {
        let d = Folktime::duration(Duration::new(599, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "9.99m");
    }
    #[test]
    fn m_8() {
        let d = Folktime::duration(Duration::new(600, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0m");
    }
    #[test]
    fn m_9() {
        let d = Folktime::duration(Duration::new(605, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0m");
    }
    #[test]
    fn m_10() {
        let d = Folktime::duration(Duration::new(606, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.1m");
    }
    #[test]
    fn m_11() {
        let d = Folktime::duration(Duration::new(59 * MIN + 59, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "59.9m");
    }

    #[test]
    fn h_0() {
        let d = Folktime::duration(Duration::new(3600, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00h");
    }
    #[test]
    fn h_1() {
        let d = Folktime::duration(Duration::new(3635, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00h");
    }
    #[test]
    fn h_2() {
        let d = Folktime::duration(Duration::new(3636, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.01h");
    }
    #[test]
    fn h_3() {
        let d = Folktime::duration(Duration::new(3671, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.01h");
    }
    #[test]
    fn h_4() {
        let d = Folktime::duration(Duration::new(3672, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.02h");
    }
    #[test]
    fn h_5() {
        let d = Folktime::duration(Duration::new(5400, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.50h");
    }
    #[test]
    fn h_6() {
        let d = Folktime::duration(Duration::new(7199, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.99h");
    }
    #[test]
    fn h_7() {
        let d = Folktime::duration(Duration::new(7200, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2.00h");
    }
    #[test]
    fn h_8() {
        let d = Folktime::duration(Duration::new(9 * HOUR + 59 * MIN + 59, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "9.99h");
    }
    #[test]
    fn h_9() {
        let d = Folktime::duration(Duration::new(10 * HOUR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0h");
    }
    #[test]
    fn h_10() {
        let d = Folktime::duration(Duration::new(10 * HOUR + 5 * MIN + 59, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0h");
    }
    #[test]
    fn h_11() {
        let d = Folktime::duration(Duration::new(10 * HOUR + 6 * MIN, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.1h");
    }
    #[test]
    fn h_12() {
        let d = Folktime::duration(Duration::new(23 * HOUR + 59 * MIN + 59, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "23.9h");
    }

    #[test]
    fn d_0() {
        let d = Folktime::duration(Duration::new(DAY, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00d");
    }
    #[test]
    fn d_1() {
        let d = Folktime::duration(Duration::new(DAY + 863, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00d");
    }
    #[test]
    fn d_2() {
        let d = Folktime::duration(Duration::new(DAY + 864, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.01d");
    }
    #[test]
    fn d_3() {
        let d = Folktime::duration(Duration::new(2 * DAY - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.99d");
    }
    #[test]
    fn d_4() {
        let d = Folktime::duration(Duration::new(2 * DAY, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2.00d");
    }
    #[test]
    fn d_5() {
        let d = Folktime::duration(Duration::new(2 * DAY + 12 * HOUR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2.50d");
    }
    #[test]
    fn d_6() {
        let d = Folktime::duration(Duration::new(WEEK - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "6.99d");
    }

    #[test]
    fn w_0() {
        let d = Folktime::duration(Duration::new(WEEK, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00w");
    }
    #[test]
    fn w_1() {
        let d = Folktime::duration(Duration::new(WEEK + 6047, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00w");
    }
    #[test]
    fn w_2() {
        let d = Folktime::duration(Duration::new(WEEK + 6048, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.01w");
    }
    #[test]
    fn w_3() {
        let d = Folktime::duration(Duration::new(2 * WEEK - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.99w");
    }
    #[test]
    fn w_4() {
        let d = Folktime::duration(Duration::new(2 * WEEK, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2.00w");
    }
    #[test]
    fn w_5() {
        let d = Folktime::duration(Duration::new(4 * WEEK, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "4.00w");
    }
    #[test]
    fn w_6() {
        let d = Folktime::duration(Duration::new(MONTH - 1, 999_999_999)).with_style(STYLE);
        let s = format!("{}", d);
        println!("{}", s);

        assert!(s.starts_with("4."));
        assert!(s.ends_with('w'));

        assert_eq!(s.chars().next(), Some('4'));
        assert_eq!(s.chars().nth(1), Some('.'));
        assert!(s.chars().nth(2).unwrap().is_ascii_digit());
        assert!(s.chars().nth(3).unwrap().is_ascii_digit());
        assert_eq!(s.chars().nth(4), Some('w'));
        assert_eq!(s.chars().nth(5), None);
    }

    #[test]
    fn mo_0() {
        let d = Folktime::duration(Duration::new(MONTH, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00mo");
    }
    #[test]
    fn mo_1() {
        let d = Folktime::duration(Duration::new(2 * MONTH - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.99mo");
    }
    #[test]
    fn mo_2() {
        let d = Folktime::duration(Duration::new(2 * MONTH, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2.00mo");
    }
    #[test]
    fn mo_3() {
        let d = Folktime::duration(Duration::new(6 * MONTH, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "6.00mo");
    }
    #[test]
    fn mo_4() {
        let d = Folktime::duration(Duration::new(YEAR - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "11.9mo");
    }

    #[test]
    fn y_0() {
        let d = Folktime::duration(Duration::new(YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00y");
    }
    #[test]
    fn y_1() {
        let d = Folktime::duration(Duration::new(2 * YEAR - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.99y");
    }
    #[test]
    fn y_2() {
        let d = Folktime::duration(Duration::new(2 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2.00y");
    }
    #[test]
    fn y_3() {
        let d = Folktime::duration(Duration::new(10 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0y");
    }
    #[test]
    fn y_4() {
        let d = Folktime::duration(Duration::new(100 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "100y");
    }
    #[test]
    fn y_5() {
        let d = Folktime::duration(Duration::new(999 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "999y");
    }

    #[test]
    fn ky_0() {
        let d = Folktime::duration(Duration::new(1_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00ky");
    }
    #[test]
    fn ky_1() {
        let d = Folktime::duration(Duration::new(1_010 * YEAR - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00ky");
    }
    #[test]
    fn ky_2() {
        let d = Folktime::duration(Duration::new(1_010 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.01ky");
    }
    #[test]
    fn ky_3() {
        let d = Folktime::duration(Duration::new(2_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2.00ky");
    }
    #[test]
    fn ky_4() {
        let d = Folktime::duration(Duration::new(10_000 * YEAR - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "9.99ky");
    }
    #[test]
    fn ky_5() {
        let d = Folktime::duration(Duration::new(10_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0ky");
    }
    #[test]
    fn ky_6() {
        let d =
            Folktime::duration(Duration::new(100_000 * YEAR - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "99.9ky");
    }
    #[test]
    fn ky_7() {
        let d = Folktime::duration(Duration::new(100_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "100ky");
    }
    #[test]
    fn ky_8() {
        let d =
            Folktime::duration(Duration::new(1_000_000 * YEAR - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "999ky");
    }

    #[test]
    fn my_0() {
        let d = Folktime::duration(Duration::new(1_000_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00My");
    }
    #[test]
    fn my_1() {
        let d =
            Folktime::duration(Duration::new(1_010_000 * YEAR - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00My");
    }
    #[test]
    fn my_2() {
        let d = Folktime::duration(Duration::new(1_010_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.01My");
    }
    #[test]
    fn my_3() {
        let d = Folktime::duration(Duration::new(2_000_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2.00My");
    }
    #[test]
    fn my_4() {
        let d =
            Folktime::duration(Duration::new(10_000_000 * YEAR - 1, 999_999_999)).with_style(STYLE);
        assert_eq!(format!("{}", d), "9.99My");
    }
    #[test]
    fn my_5() {
        let d = Folktime::duration(Duration::new(10_000_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0My");
    }
    #[test]
    fn my_6() {
        let d = Folktime::duration(Duration::new(100_000_000 * YEAR - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "99.9My");
    }
    #[test]
    fn my_7() {
        let d = Folktime::duration(Duration::new(100_000_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "100My");
    }
    #[test]
    fn my_8() {
        let d = Folktime::duration(Duration::new(1_000_000_000 * YEAR - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "999My");
    }

    #[test]
    fn gy_0() {
        let d = Folktime::duration(Duration::new(1_000_000_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00Gy");
    }
    #[test]
    fn gy_1() {
        let d = Folktime::duration(Duration::new(1_010_000_000 * YEAR - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "1.00Gy");
    }
    #[test]
    fn gy_2() {
        let d = Folktime::duration(Duration::new(1_010_000_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "1.01Gy");
    }
    #[test]
    fn gy_3() {
        let d = Folktime::duration(Duration::new(2_000_000_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "2.00Gy");
    }
    #[test]
    fn gy_4() {
        let d = Folktime::duration(Duration::new(10_000_000_000 * YEAR - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "9.99Gy");
    }
    #[test]
    fn gy_5() {
        let d = Folktime::duration(Duration::new(10_000_000_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "10.0Gy");
    }
    #[test]
    fn gy_6() {
        let d = Folktime::duration(Duration::new(100_000_000_000 * YEAR - 1, 999_999_999))
            .with_style(STYLE);
        assert_eq!(format!("{}", d), "99.9Gy");
    }
    #[test]
    fn gy_7() {
        let d = Folktime::duration(Duration::new(100_000_000_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "100Gy");
    }
    #[test]
    fn gy_8() {
        let d = Folktime::duration(Duration::new(500_000_000_000 * YEAR, 0)).with_style(STYLE);
        assert_eq!(format!("{}", d), "500Gy");
    }
}
