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
/// ';' denotes statement end
pub const SEMICOLON: &str = ";";
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
/// '{' opens new scopes and lambda body
pub const CURLY_BRACKET_LEFT: &str = "{";
/// '}' closes scopes and lambda body
pub const CURLY_BRACKET_RIGHT: &str = "}";
/// '(' opens parameter / argument lists
pub const ROUND_BRACKET_LEFT: &str = "(";
/// ')' opens parameter / argument lists
pub const ROUND_BRACKET_RIGHT: &str = ")";

// Special variables

/// 'value' special argument inside property's `set(value) {...}` and `get() {...}`
pub const CONV_VAR_VALUE: &str = "value";
/// 'filed' special variable inside property's `set(value) {...}` and `get() {...}`
pub const CONV_VAR_FIELD: &str = "field";


// CATEGORY
pub const NAME_ESCAPED_TOKENS: &str = " -!\"#$%^&()*+,-=?@^_{|}~";
pub const NAME_DISALLOWED_TOKENS: &str = ".:/\\[]<>";

pub mod keyword {
    pub const CLASS: &str = "class";
    /// 'this' keyword, refers to current context parameter or parent constructor
    pub const THIS: &str = "this";
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
    /// 'data' class keyword
    pub const DATA: &str = "data";
    pub const INTERFACE: &str = "interface";
    pub const FINAL: &str = "final";
    pub const ABSTRACT: &str = "abstract";
    pub const IMPORT: &str = "import";
    pub const CONST: &str = "const";
    /// 'by' keyword
    pub const BY: &str = "by";
    /// 'constructor' keyword
    pub const CONSTRUCTOR: &str = "constructor";
    /// 'init' keyword, used for initializing class after constructor call
    pub const INIT: &str = "init";
    /// `companion`
    pub const COMPANION: &str = "companion";

    //
    pub const WHERE: &str = "where";
    pub const IN: &str = "in";
    pub const OUT: &str = "out";

}