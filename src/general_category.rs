use crate::codepoint_type::CodepointType;
use core::str::FromStr;

/// Values for the General_Category (gc) character property.
/// These values are fixed; no new values will be added.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GeneralCategory {
    /// Letter, uppercase
    Lu,
    /// Letter, lowercase
    Ll,
    /// Letter, titlecase
    Lt,
    /// Letter, modifier
    Lm,
    /// Letter, other
    Lo,
    /// Mark, nonspacing
    Mn,
    /// Mark, spacing combining
    Mc,
    /// Mark, enclosing
    Me,
    /// Number, decimal digit
    Nd,
    /// Number, letter
    Nl,
    /// Number, other
    No,
    /// Punctuation, connector
    Pc,
    /// Punctuation, dash
    Pd,
    /// Punctuation, open
    Ps,
    /// Punctuation, close
    Pe,
    /// Punctuation, initial quote
    Pi,
    /// Punctuation, final quote
    Pf,
    /// Punctuation, other
    Po,
    /// Symbol, math
    Sm,
    /// Symbol, currency
    Sc,
    /// Symbol, modifier
    Sk,
    /// Symbol, other
    So,
    /// Separator, space
    Zs,
    /// Separator, line
    Zl,
    /// Separator, paragraph
    Zp,
    /// Other, control
    Cc,
    /// Other, format
    Cf,
    /// Other, surrogate
    Cs,
    /// Other, private use
    Co,
    /// Other, not assigned
    Cn,
}

impl GeneralCategory {
    /// This method returns the short name (e.g. "Lu", "Ll") for the given General_Category.
    /// The values are stable; they will not change in future Unicode versions.
    pub fn short_name(&self) -> &'static str {
        match self {
            GeneralCategory::Lu => "Lu",
            GeneralCategory::Ll => "Ll",
            GeneralCategory::Lt => "Lt",
            GeneralCategory::Lm => "Lm",
            GeneralCategory::Lo => "Lo",
            GeneralCategory::Mn => "Mn",
            GeneralCategory::Mc => "Mc",
            GeneralCategory::Me => "Me",
            GeneralCategory::Nd => "Nd",
            GeneralCategory::Nl => "Nl",
            GeneralCategory::No => "No",
            GeneralCategory::Pc => "Pc",
            GeneralCategory::Pd => "Pd",
            GeneralCategory::Ps => "Ps",
            GeneralCategory::Pe => "Pe",
            GeneralCategory::Pi => "Pi",
            GeneralCategory::Pf => "Pf",
            GeneralCategory::Po => "Po",
            GeneralCategory::Sm => "Sm",
            GeneralCategory::Sc => "Sc",
            GeneralCategory::Sk => "Sk",
            GeneralCategory::So => "So",
            GeneralCategory::Zs => "Zs",
            GeneralCategory::Zl => "Zl",
            GeneralCategory::Zp => "Zp",
            GeneralCategory::Cc => "Cc",
            GeneralCategory::Cf => "Cf",
            GeneralCategory::Cs => "Cs",
            GeneralCategory::Co => "Co",
            GeneralCategory::Cn => "Cn",
        }
    }

    /// This method returns the long name (e.g. "Letter, uppercase") for the given General_Category.
    /// The values are stable; they will not change in future Unicode versions.
    pub fn long_name(&self) -> &'static str {
        match self {
            GeneralCategory::Lu => "Letter, uppercase",
            GeneralCategory::Ll => "Letter, lowercase",
            GeneralCategory::Lt => "Letter, titlecase",
            GeneralCategory::Lm => "Letter, modifier",
            GeneralCategory::Lo => "Letter, other",
            GeneralCategory::Mn => "Mark, nonspacing",
            GeneralCategory::Mc => "Mark, spacing combining",
            GeneralCategory::Me => "Mark, enclosing",
            GeneralCategory::Nd => "Number, decimal digit",
            GeneralCategory::Nl => "Number, letter",
            GeneralCategory::No => "Number, other",
            GeneralCategory::Pc => "Punctuation, connector",
            GeneralCategory::Pd => "Punctuation, dash",
            GeneralCategory::Ps => "Punctuation, open",
            GeneralCategory::Pe => "Punctuation, close",
            GeneralCategory::Pi => "Punctuation, initial quote",
            GeneralCategory::Pf => "Punctuation, final quote",
            GeneralCategory::Po => "Punctuation, other",
            GeneralCategory::Sm => "Symbol, math",
            GeneralCategory::Sc => "Symbol, currency",
            GeneralCategory::Sk => "Symbol, modifier",
            GeneralCategory::So => "Symbol, other",
            GeneralCategory::Zs => "Separator, space",
            GeneralCategory::Zl => "Separator, line",
            GeneralCategory::Zp => "Separator, paragraph",
            GeneralCategory::Cc => "Other, control",
            GeneralCategory::Cf => "Other, format",
            GeneralCategory::Cs => "Other, surrogate",
            GeneralCategory::Co => "Other, private use",
            GeneralCategory::Cn => "Other, not assigned",
        }
    }

    pub fn codepoint_type(&self, codepoint: u32) -> CodepointType {
        match self {
            GeneralCategory::Cc => CodepointType::Control,
            GeneralCategory::Co => CodepointType::PrivateUse,
            GeneralCategory::Cs => CodepointType::Surrogate,
            GeneralCategory::Cf | GeneralCategory::Zl | GeneralCategory::Zp => {
                CodepointType::Format
            }
            GeneralCategory::Cn => match codepoint {
                0x00fdd0..=0x00fdef => CodepointType::Noncharacter,
                0x00fffe..=0x00ffff => CodepointType::Noncharacter,
                0x01fffe..=0x01ffff => CodepointType::Noncharacter,
                0x02fffe..=0x02ffff => CodepointType::Noncharacter,
                0x03fffe..=0x03ffff => CodepointType::Noncharacter,
                0x04fffe..=0x04ffff => CodepointType::Noncharacter,
                0x05fffe..=0x05ffff => CodepointType::Noncharacter,
                0x06fffe..=0x06ffff => CodepointType::Noncharacter,
                0x07fffe..=0x07ffff => CodepointType::Noncharacter,
                0x08fffe..=0x08ffff => CodepointType::Noncharacter,
                0x09fffe..=0x09ffff => CodepointType::Noncharacter,
                0x0afffe..=0x0affff => CodepointType::Noncharacter,
                0x0bfffe..=0x0bffff => CodepointType::Noncharacter,
                0x0cfffe..=0x0cffff => CodepointType::Noncharacter,
                0x0dfffe..=0x0dffff => CodepointType::Noncharacter,
                0x0efffe..=0x0effff => CodepointType::Noncharacter,
                0x0ffffe..=0x0fffff => CodepointType::Noncharacter,
                0x10fffe..=0x10ffff => CodepointType::Noncharacter,
                _ => CodepointType::Reserved,
            },
            _ => CodepointType::Graphic,
        }
    }
}

impl FromStr for GeneralCategory {
    type Err = ();

    fn from_str(s: &str) -> Result<GeneralCategory, ()> {
        match s {
            "Lu" => Ok(GeneralCategory::Lu),
            "Ll" => Ok(GeneralCategory::Ll),
            "Lt" => Ok(GeneralCategory::Lt),
            "Lm" => Ok(GeneralCategory::Lm),
            "Lo" => Ok(GeneralCategory::Lo),
            "Mn" => Ok(GeneralCategory::Mn),
            "Mc" => Ok(GeneralCategory::Mc),
            "Me" => Ok(GeneralCategory::Me),
            "Nd" => Ok(GeneralCategory::Nd),
            "Nl" => Ok(GeneralCategory::Nl),
            "No" => Ok(GeneralCategory::No),
            "Pc" => Ok(GeneralCategory::Pc),
            "Pd" => Ok(GeneralCategory::Pd),
            "Ps" => Ok(GeneralCategory::Ps),
            "Pe" => Ok(GeneralCategory::Pe),
            "Pi" => Ok(GeneralCategory::Pi),
            "Pf" => Ok(GeneralCategory::Pf),
            "Po" => Ok(GeneralCategory::Po),
            "Sm" => Ok(GeneralCategory::Sm),
            "Sc" => Ok(GeneralCategory::Sc),
            "Sk" => Ok(GeneralCategory::Sk),
            "So" => Ok(GeneralCategory::So),
            "Zs" => Ok(GeneralCategory::Zs),
            "Zl" => Ok(GeneralCategory::Zl),
            "Zp" => Ok(GeneralCategory::Zp),
            "Cc" => Ok(GeneralCategory::Cc),
            "Cf" => Ok(GeneralCategory::Cf),
            "Cs" => Ok(GeneralCategory::Cs),
            "Co" => Ok(GeneralCategory::Co),
            "Cn" => Ok(GeneralCategory::Cn),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::general_category::GeneralCategory;

    #[test]
    fn category_name() {
        assert_eq!(GeneralCategory::Lu.long_name(), "Letter, uppercase");
        assert_eq!(GeneralCategory::Sc.long_name(), "Symbol, currency");
    }
}
