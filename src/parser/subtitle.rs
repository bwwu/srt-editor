use crate::parser::timecode::TimecodeLine;

#[derive(Debug, PartialEq)]
pub struct Subtitle {
    pub count: u32,
    pub timecode_line: TimecodeLine,
    pub text: String,
}
