#![doc = include_str!("README.md")]

// SPACES

/// Empty string
pub const EMPTY: &str = "";
/// Single space
pub const SPACE: &str = " ";
/// New line string
pub const NEW_LINE: &str = "\n";
/// New line character
pub const NEW_LINE_CH: char = '\n';
/// Default indentation value
pub const INDENT: &str = "    ";

// SYMBOLS
/// '.' used to separate qualifiers
pub const DOT: &str = ".";
/// '*' symbol used for star projections
pub const STAR: &str = "*";
/// ':' separates parameter / property name and type
pub const COLON: &str = ":";
/// `=` assign operator
pub const ASSIGN: &str = "=";
/// ',' separates list of values
pub const COMMA: &str = ",";
/// '?' denotes nullability
pub const QUESTION_MARK: &str = "?";
/// '`' used to escape non JVM compatible identifiers
pub const TICK: &str = "`";
/// `<` start of generic parameters
pub const ANGLE_BRACKET_LEFT: &str = "<";
/// `>` end of generic parameters
pub const ANGLE_BRACKET_RIGHT: &str = ">";
/// '->' separates lambda arguments from body
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