mod subtitle;
mod timecode;

use anyhow::Result;
use regex::Regex;
use std::sync::LazyLock;
use subtitle::Subtitle;
use timecode::{Timecode, TimecodeLine};

static RE_BREAK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*$").unwrap());

static RE_COUNTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+$").unwrap());

static RE_TIMECODE_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\d{2}:\d{2}:\d{2},\d{3}\s+-->\s+\d{2}:\d{2}:\d{2},\d{3}").unwrap()
});

#[derive(Debug)]
enum SrtState {
    /// Expecting a Counter line to designate the start of a new subtitle.
    ExpectStartBlock,
    /// Received [SrtToken::Counter] block, expecting the timecode statement.
    ExpectTimecode,
    /// Received [SrtToken::TextLine] expecting one or more textline.
    ExpectTextline,
    /// Received at least one [SrtToken::TextLine], expecting either another TextLine or a
    /// [SrtToken::Break].
    MaybeExpectEndBlock,
}

#[derive(Debug)]
enum SrtToken {
    Counter(u32),
    TimecodeLine(TimecodeLine),
    TextLine(String),
    Break,
}

pub fn parse(text: String) -> Result<Vec<Subtitle>> {
    let mut state = SrtState::ExpectStartBlock;

    let lines: Vec<&str> = text.lines().collect();
    let mut count: u32 = 0;
    let mut timecode_line = TimecodeLine::default();
    let mut text_buf = String::new();

    let mut subtitles: Vec<Subtitle> = vec![];

    for line in lines {
        println!("Line is {}", line);
        let next_token = parse_line(String::from(line));
        match &state {
            SrtState::ExpectStartBlock => {
                if let Ok(SrtToken::Counter(new_count)) = next_token {
                    count = new_count;
                    state = SrtState::ExpectTimecode;
                } else if let Ok(SrtToken::Break) = next_token {
                    // Allow for extraneous newlines at the start or the end of a block
                } else {
                    return Err(anyhow::anyhow!("expected a counter"));
                }
            }
            SrtState::ExpectTimecode => {
                if let Ok(SrtToken::TimecodeLine(new_timecode_line)) = next_token {
                    timecode_line = new_timecode_line;
                    state = SrtState::ExpectTextline;
                } else {
                    return Err(anyhow::anyhow!("expected a timecode line"));
                }
            }
            SrtState::ExpectTextline => {
                if let Ok(SrtToken::TextLine(first_text_line)) = next_token {
                    state = SrtState::MaybeExpectEndBlock;
                    text_buf.clear();
                    text_buf.push_str(first_text_line.as_str());
                } else {
                    return Err(anyhow::anyhow!("expected at least one text line"));
                }
            }
            SrtState::MaybeExpectEndBlock => {
                if let Ok(SrtToken::TextLine(next_text_line)) = next_token {
                    text_buf.push('\n');
                    text_buf.push_str(next_text_line.as_str());
                    println!("More text!");
                } else if let Ok(SrtToken::Break) = next_token {
                    println!("Break");
                    // todo create new subtitle
                    subtitles.push(Subtitle {
                        count,
                        timecode_line,
                        text: text_buf.clone(),
                    });
                    state = SrtState::ExpectStartBlock;
                } else {
                    return Err(anyhow::anyhow!("expected either a text line or a break"));
                }
            }
        }
    }

    Ok(subtitles)
}

/// Parse a single line of text into a token
fn parse_line(line: String) -> Result<SrtToken> {
    if RE_COUNTER.is_match(line.as_str()) {
        Ok(SrtToken::Counter(line.parse()?))
    } else if RE_TIMECODE_LINE.is_match(line.as_str()) {
        Ok(SrtToken::TimecodeLine(TimecodeLine::new(line.as_str())?))
    } else if RE_BREAK.is_match(line.as_str()) {
        Ok(SrtToken::Break)
    } else {
        Ok(SrtToken::TextLine(line))
    }
}

#[derive(Default)]
pub struct SubRip {
    subtitles: Vec<Subtitle>,
}

impl SubRip {
    pub fn from_string(text: String) -> Result<Self> {
        Ok(Self {
            subtitles: parse(text)?,
        })
    }

    pub fn apply_offset(&mut self, offset: Timecode) -> &Self {
        for s in &mut self.subtitles {
            s.timecode_line.start.offset(offset);
            s.timecode_line.end.offset(offset);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        parse,
        subtitle::Subtitle,
        timecode::{Timecode, TimecodeLine},
    };

    #[test]
    fn test_parse_ok_single() {
        let text = String::from(
            "1
00:01:00,000 --> 00:01:03,000
Hello, world!
foo, bar

",
        );
        assert_eq!(
            parse(text).unwrap(),
            vec![Subtitle {
                count: 1,
                timecode_line: TimecodeLine {
                    start: Timecode::new(0, 1, 0, 0),
                    end: Timecode::new(0, 1, 3, 0),
                },
                text: String::from("Hello, world!\nfoo, bar")
            }]
        );
    }

    #[test]
    fn test_parse_ok_multiple() {
        let text = String::from(
            "1
00:01:00,000 --> 00:01:03,000
Hi, how are you?

2
00:03:04,349 --> 00:03:08,788
Great!
how are you?

",
        );
        assert_eq!(
            parse(text).unwrap(),
            vec![
                Subtitle {
                    count: 1,
                    timecode_line: TimecodeLine {
                        start: Timecode::new(0, 1, 0, 0),
                        end: Timecode::new(0, 1, 3, 0)
                    },
                    text: String::from("Hi, how are you?")
                },
                Subtitle {
                    count: 2,
                    timecode_line: TimecodeLine {
                        start: Timecode::new(0, 3, 4, 349),
                        end: Timecode::new(0, 3, 8, 788),
                    },
                    text: String::from("Great!\nhow are you?")
                }
            ]
        );
    }
}
