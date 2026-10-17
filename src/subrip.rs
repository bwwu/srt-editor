use crate::parser::{parse,subtitle::Subtitle, timecode::Timecode};
use anyhow::Result;

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
mod test {
    use super::SubRip;
    use crate::parser::timecode::Timecode;

    #[test]
    fn test_apply_offset_once() {
    
        let subrip = SubRip::from_string(String::from("1
                00:01:13,445 --> 00:01:17,291
                Hello

        ")).unwrap();

        //assert_eq!(subrip.apply_offset(Timecode::new(0,0,1,8), Sub);

    }
}
