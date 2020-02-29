use crate::general_category::GeneralCategory;
use core::convert::TryFrom;

pub trait Codepoint: Copy {
    fn value(self) -> u32;

    fn general_category_unstable(self) -> Option<GeneralCategory> {
        Some(match self.value() {
            0x000000..=0x00001f => GeneralCategory::Cc,
            0x00007f..=0x00009f => GeneralCategory::Cc,
            0x00d800..=0x00dfff => GeneralCategory::Cs,
            0x00e000..=0x00f8ff => GeneralCategory::Co,
            0x00fdd0..=0x00fdef => GeneralCategory::Cn,
            0x00fffe..=0x00ffff => GeneralCategory::Cn,
            0x01fffe..=0x01ffff => GeneralCategory::Cn,
            0x02fffe..=0x02ffff => GeneralCategory::Cn,
            0x03fffe..=0x03ffff => GeneralCategory::Cn,
            0x04fffe..=0x04ffff => GeneralCategory::Cn,
            0x05fffe..=0x05ffff => GeneralCategory::Cn,
            0x06fffe..=0x06ffff => GeneralCategory::Cn,
            0x07fffe..=0x07ffff => GeneralCategory::Cn,
            0x08fffe..=0x08ffff => GeneralCategory::Cn,
            0x09fffe..=0x09ffff => GeneralCategory::Cn,
            0x0afffe..=0x0affff => GeneralCategory::Cn,
            0x0bfffe..=0x0bffff => GeneralCategory::Cn,
            0x0cfffe..=0x0cffff => GeneralCategory::Cn,
            0x0dfffe..=0x0dffff => GeneralCategory::Cn,
            0x0efffe..=0x0effff => GeneralCategory::Cn,
            0x0f0000..=0x0ffffd => GeneralCategory::Co,
            0x0ffffe..=0x0fffff => GeneralCategory::Cn,
            0x100000..=0x10fffd => GeneralCategory::Co,
            0x10fffe..=0x10ffff => GeneralCategory::Cn,
            _ => return None,
        })
    }
}

/// Represents a Unicode codepoint, i.e. a number between 0 and 0x10FFFF (inclusive).
#[derive(Copy, Clone)]
pub struct UnicodeCodepoint(u32);

impl Codepoint for UnicodeCodepoint {
    fn value(self) -> u32 {
        self.0
    }
}

impl Codepoint for char {
    fn value(self) -> u32 {
        self as u32
    }
}

impl From<UnicodeCodepoint> for u32 {
    fn from(codepoint: UnicodeCodepoint) -> u32 {
        codepoint.0
    }
}

impl TryFrom<u32> for UnicodeCodepoint {
    type Error = ();

    fn try_from(i: u32) -> Result<UnicodeCodepoint, Self::Error> {
        if i > 0x10FFFF {
            Err(())
        } else {
            Ok(UnicodeCodepoint(i))
        }
    }
}

impl From<char> for UnicodeCodepoint {
    fn from(ch: char) -> UnicodeCodepoint {
        UnicodeCodepoint(ch as u32)
    }
}

impl TryFrom<UnicodeCodepoint> for char {
    type Error = <char as TryFrom<u32>>::Error;

    fn try_from(codepoint: UnicodeCodepoint) -> Result<char, <char as TryFrom<u32>>::Error> {
        char::try_from(u32::from(codepoint))
    }
}
