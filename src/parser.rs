mod timecode;

use anyhow::Result;
use regex::Regex;
use std::sync::LazyLock;
use timecode::TimecodeLine;

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
    Counter(u16),
    TimecodeLine(TimecodeLine),
    TextLine(String),
    Break,
}

pub fn parse(text: String) {
    let mut state = SrtState::ExpectStartBlock;

    let lines: Vec<&str> = text.lines().collect();

    for line in lines {
        let next_token = parse_line(String::from(line));
    }
}

/// Parse a single line of text into a token
fn parse_line(line: String) -> Result<SrtToken> {
    if RE_COUNTER.is_match(line.as_str()) {
        Ok(SrtToken::Counter(line.parse()?))
    } else if RE_TIMECODE_LINE.is_match(line.as_str()) {
        Ok(SrtToken::TimecodeLine(TimecodeLine::new(line.as_str())?))
    } else {
        Ok(SrtToken::Break)
    }
}

pub struct SrtParser {}

impl SrtParser {
    pub fn new(filepath: &str) -> Self {
        SrtParser {}
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    #[test]
    fn test_lex() {}
}
