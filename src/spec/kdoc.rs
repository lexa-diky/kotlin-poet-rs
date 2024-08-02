use crate::io::RenderKotlin;
use crate::spec::CodeBlock;
use crate::tokens;

/// Represents a Kotlin documentation comment in KDoc format.
///
/// Entities that support KDoc should usually store it as singular instance and merge multiple KDocs into one.
/// Missing KDoc is represented as [Option::None].
#[derive(Debug, Clone)]
pub struct KDoc {
    content: String,
}

impl KDoc {
    /// Creates new, empty KDoc
    pub fn new() -> KDoc {
        KDoc {
            content: String::new()
        }
    }

    /// Appends content to the KDoc.
    pub fn append(mut self, content: &str) -> KDoc {
        self.content.push_str(content);
        self
    }

    /// Merges contents of [other] into [self]. Adds new line between contents.
    pub fn merge(mut self, other: KDoc) -> KDoc {
        self.content.push_str(tokens::NEW_LINE);
        self.content.push_str(other.content.as_str());
        self
    }
}

impl From<&str> for KDoc {
    fn from(value: &str) -> Self {
        KDoc::new().append(value)
    }
}

impl RenderKotlin for KDoc {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
        block.with_atom(tokens::KDOC_COMMENT_START);
        block.with_new_line();
        let split = self.content.split(tokens::NEW_LINE)
            .enumerate().collect::<Vec<_>>();
        let split_len = split.len();
        for (idx, line) in split {
            if idx == split_len - 1 && line.is_empty() {
                break;
            }

            block.with_atom(tokens::KDOC_COMMENT_MIDDLE);
            block.with_space();
            block.with_atom(line);
            block.with_new_line();
        }
        block.with_atom(tokens::KDOC_COMMENT_END);

        block
    }
}

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;

    #[test]
    fn test_kdoc_render() {
        let mut comment = super::KDoc::new()
            .append("Hello\n")
            .append("World\n");

        assert_eq!(
            comment.render_string(),
            "/**\n * Hello\n * World\n */"
        )
    }

    #[test]
    fn test_comment_block() {
        let mut comment = super::KDoc::new()
            .append("Hello\n")
            .merge(super::KDoc::new().append("World\n"));

        assert_eq!(
            comment.render_string(),
            "/**\n * Hello\n * \n * World\n */"
        )
    }
}
