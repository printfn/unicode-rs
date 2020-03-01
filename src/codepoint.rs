use crate::general_category::GeneralCategory;
use core::convert::TryFrom;

pub trait Codepoint: Copy {
    fn value(self) -> u32;

    fn general_category_unstable(self) -> Option<GeneralCategory> {
        Some(match self.value() {
            0x00_0000..=0x00_001f => GeneralCategory::Cc,
            0x00_007f..=0x00_009f => GeneralCategory::Cc,
            0x00_d800..=0x00_dfff => GeneralCategory::Cs,
            0x00_e000..=0x00_f8ff => GeneralCategory::Co,
            0x00_fdd0..=0x00_fdef => GeneralCategory::Cn,
            0x00_fffe..=0x00_ffff => GeneralCategory::Cn,
            0x01_fffe..=0x01_ffff => GeneralCategory::Cn,
            0x02_fffe..=0x02_ffff => GeneralCategory::Cn,
            0x03_fffe..=0x03_ffff => GeneralCategory::Cn,
            0x04_fffe..=0x04_ffff => GeneralCategory::Cn,
            0x05_fffe..=0x05_ffff => GeneralCategory::Cn,
            0x06_fffe..=0x06_ffff => GeneralCategory::Cn,
            0x07_fffe..=0x07_ffff => GeneralCategory::Cn,
            0x08_fffe..=0x08_ffff => GeneralCategory::Cn,
            0x09_fffe..=0x09_ffff => GeneralCategory::Cn,
            0x0a_fffe..=0x0a_ffff => GeneralCategory::Cn,
            0x0b_fffe..=0x0b_ffff => GeneralCategory::Cn,
            0x0c_fffe..=0x0c_ffff => GeneralCategory::Cn,
            0x0d_fffe..=0x0d_ffff => GeneralCategory::Cn,
            0x0e_fffe..=0x0e_ffff => GeneralCategory::Cn,
            0x0f_0000..=0x0f_fffd => GeneralCategory::Co,
            0x0f_fffe..=0x0f_ffff => GeneralCategory::Cn,
            0x10_0000..=0x10_fffd => GeneralCategory::Co,
            0x10_fffe..=0x10_ffff => GeneralCategory::Cn,
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
        if i > 0x10_FFFF {
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
