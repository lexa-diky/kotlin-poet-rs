use std::str::FromStr;

use crate::io::RenderKotlin;
use crate::spec::CodeBlock;
use crate::tokens;
use crate::util::SemanticConversionError;

/// Kotlin identifier name, automatically escaped with backticks if it contains escapable tokens
#[derive(Debug, PartialEq, Clone)]
pub struct Name {
    value: String,
    should_be_escaped: bool
}

/// Creates new [Name] from [&str], may panic if creates invalid name
impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Name::from_str(value).unwrap()
    }
}

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
    fn render(&self) -> CodeBlock {
        if self.should_be_escaped {
            return CodeBlock::atom(format!("`{}`", self.value).as_str());
        }
        return CodeBlock::atom(self.value.as_str());
    }
}

#[cfg(test)]
mod test {
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
}