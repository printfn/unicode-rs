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
