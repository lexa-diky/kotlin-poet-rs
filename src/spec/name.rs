use std::str::FromStr;

use crate::io::RenderKotlin;
use crate::spec::CodeBlock;
use crate::tokens;
use crate::util::{SemanticConversionError, yolo_from_str};

/// Kotlin identifier name, automatically escaped with backticks if it contains escapable tokens
///
/// # Examples
/// ```rust
/// use std::str::FromStr;
/// use kotlin_poet_rs::io::RenderKotlin;
/// use kotlin_poet_rs::spec::Name;
///
/// let name = Name::from("Foo");
/// assert_eq!(name.render_string(), "Foo");
///
/// let escaped_name = Name::from("Foo Bar");
/// assert_eq!(escaped_name.render_string(), "`Foo Bar`")
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Name {
    value: String,
    should_be_escaped: bool
}

yolo_from_str!(Name);
/// Creates new [Name] from [&str]
impl FromStr for Name {
    type Err = SemanticConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(
                SemanticConversionError::new(
                    "Name cannot be empty"
                )
            )
        }

        if s.chars().any(|ch| tokens::NAME_DISALLOWED_TOKENS.contains(ch)) {
            return Err(
                SemanticConversionError::new(
                    format!("`{} contains tokens not allowed in kotlin identifier names`", s)
                        .as_str()
                )
            )
        }

        let should_be_escaped = s.chars().any(
            |ch| tokens::NAME_ESCAPED_TOKENS.contains(ch)
        );
        
        Ok(
            Name {
                value: s.to_string(),
                should_be_escaped
            }
        )
    }
}

impl RenderKotlin for Name {
    fn render_into(&self, block: &mut CodeBlock) {
        if self.should_be_escaped {
            block.push_atom(format!("`{}`", self.value).as_str());
            return;
        }

        block.push_atom(self.value.as_str());
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_name() {
        let name = Name::from_str("Foo").unwrap();
        assert_eq!(name.render_string(), "Foo");
    }

    #[test]
    fn test_name_with_space() {
        let name = Name::from_str("Foo Bar").unwrap();
        assert_eq!(name.render_string(), "`Foo Bar`");
    }

    #[test]
    fn test_name_with_parentheses() {
        let name = Name::from_str("Foo()Bar").unwrap();
        assert_eq!(name.render_string(), "`Foo()Bar`");
    }

    #[test]
    fn test_name_with_disallowed_characters() {
        let name = Name::from_str("Foo/Bar");
        assert!(matches!(name, Err(_)));
    }

    #[test]
    fn test_empty_name() {
        let name = Name::from_str("");
        assert!(matches!(name, Err(_)));
    }
}