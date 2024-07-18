use std::ops::Deref;
use std::str::FromStr;
use crate::io::{RenderContext, RenderKotlin};
use crate::io::tokens::NAME_PROHIBITED_TOKENS;
use crate::spec::CodeBlock;

#[derive(Debug, PartialEq, Clone)]
pub struct Name {
    value: String,
}

impl Name {
    pub fn from(str: &str) -> Name {
        Name::from_str(str).unwrap()
    }
}

impl FromStr for Name {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(
            Name {
                value: s.to_string()
            }
        );
    }
}

impl RenderKotlin for Name {
    fn render(&self, context: RenderContext) -> CodeBlock {
        let contains_prohibited_token = NAME_PROHIBITED_TOKENS.iter().any(
            |it| self.value.contains(it)
        );
        if contains_prohibited_token {
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
        assert_eq!(name.render_without_context(), "Foo");
    }

    #[test]
    fn test_name_with_space() {
        let name = Name::from_str("Foo Bar").unwrap();
        assert_eq!(name.render_without_context(), "`Foo Bar`");
    }
}