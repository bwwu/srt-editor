use anyhow::Result;
use regex::Regex;
use std::sync::LazyLock;

static RE_TIMECODE_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d{2}):(\d{2}):(\d{2}),(\d{3})\s+-->\s+(\d{2}):(\d{2}):(\d{2}),(\d{3})").unwrap()
});

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Timecode {
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub millis: u16,
}

impl Timecode {
    pub fn new(hour: u8, min: u8, sec: u8, millis: u16) -> Self {
        Self {
            hour,
            min,
            sec,
            millis,
        }
    }

    /// Add a positive offset
    pub fn offset(&mut self, rhs: Timecode) -> &Self {
        self.millis += rhs.millis;
        let millis_overflow = self.millis / 1000;
        self.millis %= 1000;

        self.sec += rhs.sec + millis_overflow as u8;
        let sec_overflow = self.sec / 60;
        self.sec %= 60;

        self.min += rhs.min + sec_overflow;
        let min_overflow = self.min / 60;
        self.min %= 60;

        self.hour += rhs.hour + min_overflow;

        self
    }
}

impl std::fmt::Display for Timecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02},{:03}",
            self.hour, self.min, self.sec, self.millis
        )
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TimecodeLine {
    pub(crate) start: Timecode,
    pub(crate) end: Timecode,
}

impl TimecodeLine {
    pub fn new(line: &str) -> Result<Self> {
        match RE_TIMECODE_LINE.captures(line) {
            None => Err(anyhow::anyhow!("invalid timecode line \"{}\"", line)),
            Some(caps) => Ok(TimecodeLine {
                start: Timecode {
                    hour: caps
                        .get(1)
                        .ok_or(anyhow::anyhow!(""))?
                        .as_str()
                        .parse::<u8>()?,
                    min: caps
                        .get(2)
                        .ok_or(anyhow::anyhow!(""))?
                        .as_str()
                        .parse::<u8>()?,
                    sec: caps
                        .get(3)
                        .ok_or(anyhow::anyhow!(""))?
                        .as_str()
                        .parse::<u8>()?,
                    millis: caps
                        .get(4)
                        .ok_or(anyhow::anyhow!(""))?
                        .as_str()
                        .parse::<u16>()?,
                },
                end: Timecode {
                    hour: caps
                        .get(5)
                        .ok_or(anyhow::anyhow!(""))?
                        .as_str()
                        .parse::<u8>()?,
                    min: caps
                        .get(6)
                        .ok_or(anyhow::anyhow!(""))?
                        .as_str()
                        .parse::<u8>()?,
                    sec: caps
                        .get(7)
                        .ok_or(anyhow::anyhow!(""))?
                        .as_str()
                        .parse::<u8>()?,
                    millis: caps
                        .get(8)
                        .ok_or(anyhow::anyhow!(""))?
                        .as_str()
                        .parse::<u16>()?,
                },
            }),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::parser::timecode::{Timecode, TimecodeLine};

    #[test]
    fn test_new_is_ok() {
        assert_eq!(
            TimecodeLine::new("01:02:03,456 --> 17:18:19,012").unwrap(),
            TimecodeLine {
                start: Timecode::new(1, 2, 3, 456),
                end: Timecode::new(17, 18, 19, 12),
            }
        );
    }

    #[test]
    fn test_offset_no_overflow() {
        let mut timecode = Timecode::new(45, 20, 33, 100);
        let offset = Timecode::new(1, 39, 26, 899);
        assert_eq!(timecode.offset(offset), &Timecode::new(46, 59, 59, 999));
    }

    #[test]
    fn test_offset_with_overflow() {
        let mut timecode = Timecode::new(0, 1, 1, 2);
        let offset = Timecode::new(0, 58, 59, 999);
        assert_eq!(timecode.offset(offset), &Timecode::new(1, 0, 1, 1));
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Timecode::new(1, 22, 3, 44).to_string(),
            String::from("01:22:03,044")
        );
        assert_eq!(
            Timecode::new(11, 22, 33, 444).to_string(),
            String::from("11:22:33,444")
        );
    }
}
