#![doc = include_str!("README.md")]

// SPACES
pub const EMPTY: &str = "";
pub const SPACE: &str = " ";
pub const NEW_LINE: &str = "\n";
pub const NEW_LINE_CH: char = '\n';
pub const INDENT: &str = "    ";

// SYMBOLS
pub const DOT: &str = ".";
pub const STAR: &str = "*";
pub const COLON: &str = ":";
pub const EQUALS: &str = "=";
pub const COMMA: &str = ",";
pub const QUESTION_MARK: &str = "?";
pub const TICK: &str = "`";
pub const ANGLE_BRACKET_LEFT: &str = "<";
pub const ANGLE_BRACKET_RIGHT: &str = ">";
pub const ARROW: &str = "->";
pub const CURLY_BRACKET_LEFT: &str = "{";
pub const CURLY_BRACKET_RIGHT: &str = "}";
pub const ROUND_BRACKET_LEFT: &str = "(";
pub const ROUND_BRACKET_RIGHT: &str = ")";
pub const CONV_VAR_VALUE: &str = "value";
pub const CONV_VAR_FIELD: &str = "field";
pub const SEMICOLON: &str = ";";

// CATEGORY
pub const NAME_PROHIBITED_TOKENS: [&str; 1] = [SPACE];

pub mod keyword {
    pub const CLASS: &str = "class";
    pub const AS: &str = "as";
    pub const OPERATOR: &str = "operator";
    pub const INLINE: &str = "inline";
    pub const OVERRIDE: &str = "override";
    pub const SUSPEND: &str = "suspend";
    pub const SET: &str = "set";
    pub const GET: &str = "get";
    pub const PACKAGE: &str = "package";
    pub const TYPEALIAS: &str = "typealias";
    pub const FUN: &str = "fun";
    pub const VAL: &str = "val";
    pub const VAR: &str = "var";
    pub const PUBLIC: &str = "public";
    pub const INTERNAL: &str = "internal";
    pub const PRIVATE: &str = "private";
    pub const PROTECTED: &str = "protected";
    pub const OPEN: &str = "open";
    pub const SEALED: &str = "sealed";
    pub const OBJECT: &str = "object";
    pub const ENUM: &str = "enum";
    pub const INTERFACE: &str = "interface";
    pub const FINAL: &str = "final";
    pub const ABSTRACT: &str = "abstract";
    pub const IMPORT: &str = "import";
    pub const CONST: &str = "const";
}