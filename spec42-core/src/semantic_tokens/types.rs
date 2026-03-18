//! Token type constants for semantic highlighting (indices into the legend).

pub const TYPE_KEYWORD: u32 = 0;
pub const TYPE_STRING: u32 = 1;
pub const TYPE_NUMBER: u32 = 2;
pub const TYPE_COMMENT: u32 = 3;
pub const TYPE_OPERATOR: u32 = 4;
pub const TYPE_VARIABLE: u32 = 5;
pub const TYPE_TYPE: u32 = 6;
pub const TYPE_NAMESPACE: u32 = 7;
pub const TYPE_CLASS: u32 = 8;
pub const TYPE_INTERFACE: u32 = 9;
pub const TYPE_PROPERTY: u32 = 10;
pub const TYPE_FUNCTION: u32 = 11;

pub const TYPE_NAMES: [&str; 12] = [
    "KEYWORD",
    "STRING",
    "NUMBER",
    "COMMENT",
    "OPERATOR",
    "VARIABLE",
    "TYPE",
    "NAMESPACE",
    "CLASS",
    "INTERFACE",
    "PROPERTY",
    "FUNCTION",
];
