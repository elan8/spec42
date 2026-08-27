//! The reserved-keyword vocabulary of SysML v2, owned here because the grammar owns it.
//!
//! OMG SysML v2 Language Specification 2.0, 8.2.2.1.2 "Reserved Keywords", sorted. `position`
//! is a contextual keyword only and is deliberately absent. The pinned parser keeps its own table
//! crate-private, so this copy is pinned by count and spot-checked by test until upstream exports
//! it; consumers read it through the facade and never carry a table of their own.

/// Every reserved keyword, sorted.
pub const RESERVED_KEYWORDS: &[&str] = &[
    "about",
    "abstract",
    "accept",
    "action",
    "actor",
    "after",
    "alias",
    "all",
    "allocate",
    "allocation",
    "analysis",
    "and",
    "as",
    "assert",
    "assign",
    "assume",
    "at",
    "attribute",
    "bind",
    "binding",
    "by",
    "calc",
    "case",
    "comment",
    "concern",
    "connect",
    "connection",
    "constant",
    "constraint",
    "crosses",
    "decide",
    "def",
    "default",
    "defined",
    "dependency",
    "derived",
    "do",
    "doc",
    "else",
    "end",
    "entry",
    "enum",
    "event",
    "exhibit",
    "exit",
    "expose",
    "false",
    "filter",
    "first",
    "flow",
    "for",
    "fork",
    "frame",
    "from",
    "hastype",
    "if",
    "implies",
    "import",
    "in",
    "include",
    "individual",
    "inout",
    "interface",
    "istype",
    "item",
    "join",
    "language",
    "library",
    "locale",
    "loop",
    "merge",
    "message",
    "meta",
    "metadata",
    "nonunique",
    "not",
    "null",
    "objective",
    "occurrence",
    "of",
    "or",
    "ordered",
    "out",
    "package",
    "parallel",
    "part",
    "perform",
    "port",
    "private",
    "protected",
    "public",
    "redefines",
    "ref",
    "references",
    "render",
    "rendering",
    "rep",
    "require",
    "requirement",
    "return",
    "satisfy",
    "send",
    "snapshot",
    "specializes",
    "stakeholder",
    "standard",
    "state",
    "subject",
    "subsets",
    "succession",
    "terminate",
    "then",
    "timeslice",
    "to",
    "transition",
    "true",
    "until",
    "use",
    "variant",
    "variation",
    "verification",
    "verify",
    "via",
    "view",
    "viewpoint",
    "when",
    "while",
    "xor",
];

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
