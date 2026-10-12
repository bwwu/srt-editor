use anyhow::Result;
use regex::Regex;
use std::sync::LazyLock;

static RE_TIMECODE_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d{2}):(\d{2}):(\d{2}),(\d{3})\s+-->\s+(\d{2}):(\d{2}):(\d{2}),(\d{3})").unwrap()
});

#[derive(Debug, PartialEq)]
pub struct Timecode {
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub millis: u16,
}

#[derive(Debug, PartialEq)]
pub struct TimecodeLine {
    start: Timecode,
    end: Timecode,
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
    fn test_new() {
        assert_eq!(
            TimecodeLine::new("01:02:03,456 --> 17:18:19,012").unwrap(),
            TimecodeLine {
                start: Timecode {
                    hour: 1,
                    min: 2,
                    sec: 3,
                    millis: 456,
                },
                end: Timecode {
                    hour: 17,
                    min: 18,
                    sec: 19,
                    millis: 12
                }
            }
        );
    }
}
