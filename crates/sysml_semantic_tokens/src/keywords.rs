//! SysML v2 reserved keywords for lexer fallback highlighting.

/// Note: "position" is a contextual keyword only, not reserved—valid as identifier.
pub const RESERVED_KEYWORDS: &[&str] = &[
    "about", "abstract", "accept", "action", "actor", "after", "alias", "all", "allocate",
    "allocation", "analysis", "and", "as", "assert", "assign", "assume", "at", "attribute",
    "bind", "binding", "by", "calc", "case", "comment", "concern", "connect", "connection",
    "constant", "constraint", "crosses", "decide", "def", "default", "defined", "dependency",
    "derived", "do", "doc", "else", "end", "entry", "enum", "event", "exhibit", "exit", "expose",
    "false", "filter", "first", "flow", "for", "fork", "frame", "from", "hastype", "if", "implies",
    "import", "in", "include", "individual", "inout", "interface", "istype", "item", "join",
    "language", "library", "locale", "loop", "merge", "message", "meta", "metadata", "nonunique",
    "not", "null", "objective", "occurrence", "of", "or", "ordered", "out", "package", "parallel",
    "part", "perform", "port", "private", "protected", "provides", "public", "redefines", "ref",
    "references", "render", "rendering", "rep", "require", "requirement", "requires", "return",
    "satisfy", "send", "snapshot", "specializes", "stakeholder", "standard", "state", "subject",
    "subsets", "succession", "terminate", "then", "timeslice", "to", "transition", "true", "until",
    "use", "value", "variant", "variation", "verification", "verify", "via", "view", "viewpoint",
    "when", "while", "xor",
];

pub fn is_reserved_keyword(word: &str) -> bool {
    RESERVED_KEYWORDS.contains(&word)
}
