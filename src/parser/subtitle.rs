use crate::parser::timecode::TimecodeLine;

#[derive(Debug, PartialEq)]
pub struct Subtitle {
    pub count: u32,
    pub timecode_line: TimecodeLine,
    pub text: String,
}

impl std::fmt::Display for Subtitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{} --> {}\n{}\n",
            self.count, self.timecode_line.start, self.timecode_line.end, self.text
        )
    }
}

#[cfg(test)]
mod test {

    use crate::parser::{
        subtitle::Subtitle,
        timecode::{Timecode, TimecodeLine},
    };

    #[test]
    fn test_to_string() {
        assert_eq!(
            Subtitle {
                count: 22,
                timecode_line: TimecodeLine {
                    start: Timecode::new(1, 2, 3, 4),
                    end: Timecode::new(1, 2, 5, 6),
                },
                text: String::from("Hello\nWorld"),
            }
            .to_string(),
            "22\n01:02:03,004 --> 01:02:05,006\nHello\nWorld\n"
        );
    }
}
