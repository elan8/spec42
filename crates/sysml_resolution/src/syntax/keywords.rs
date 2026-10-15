//! The reserved-keyword vocabulary of SysML v2, owned here because the grammar owns it.
//!
//! The pinned parser owns the vocabulary; this boundary borrows that generated inventory.
//! Consumers read it through the facade and never maintain a table of their own.

/// Every reserved keyword, sorted.
pub const RESERVED_KEYWORDS: &[&str] =
    sysml_v2_parser::ast::NameLanguage::Sysml.reserved_keywords();

/// The reserved keywords, for consumers that classify tokens.
pub fn reserved_keywords() -> &'static [&'static str] {
    RESERVED_KEYWORDS
}

/// Whether `word` is a reserved keyword.
pub fn is_reserved_keyword(word: &str) -> bool {
    RESERVED_KEYWORDS.binary_search(&word).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_is_the_normative_set_sorted_for_binary_search() {
        assert_eq!(RESERVED_KEYWORDS.len(), 128);
        assert!(RESERVED_KEYWORDS.windows(2).all(|pair| pair[0] < pair[1]));
        for keyword in ["about", "def", "import", "package", "xor"] {
            assert!(is_reserved_keyword(keyword), "{keyword}");
        }
        for identifier in ["position", "provides", "requires", "value"] {
            assert!(
                !is_reserved_keyword(identifier),
                "{identifier} is not reserved"
            );
        }
    }
}
