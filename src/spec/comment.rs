use crate::io::RenderKotlin;
use crate::spec::CodeBlock;
use crate::tokens;

/// Represents a 'normal', non documentation comment in Kotlin.
///
/// Entities that support comments should usually store them as [Vec<Comment>] and aggregate multiple comments.
///
/// # Example
/// ```rust
/// use kotlin_poet_rs::spec::Comment;
/// use kotlin_poet_rs::io::RenderKotlin;
///
/// let comment = Comment::new()
///     .append("Hello, World!");
///
/// assert_eq!(
///    "// Hello, World!",
///    comment.render_string()
/// )
/// ```
pub struct Comment {
    content: String,
    is_block_render: bool,
}

impl Comment {
    /// Creates new, empty comment
    pub fn new() -> Comment {
        Comment {
            content: String::new(),
            is_block_render: false,
        }
    }

    /// Appends content to the comment.
    /// If content contains new line will be automatically converted to block comment
    pub fn append(mut self, content: &str) -> Comment {
        self.content.push_str(content);
        if self.is_block_render { return self; }
        self.is_block_render = self.content.contains(tokens::NEW_LINE);
        self
    }
}

impl RenderKotlin for Comment {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
        if self.is_block_render {
            block.with_atom(tokens::BLOCK_COMMENT_START);
            block.with_new_line();
            let split = self.content.split(tokens::NEW_LINE)
                .enumerate().collect::<Vec<_>>();
            let split_len = split.len();
            for (idx, line) in split {
                if idx == split_len - 1 && line.is_empty() {
                    break;
                }

                block.with_atom(tokens::BLOCK_COMMENT_MIDDLE);
                block.with_space();
                block.with_atom(line);
                block.with_new_line();
            }
            block.with_atom(tokens::BLOCK_COMMENT_END)
        } else {
            block.with_atom(tokens::INLINE_COMMENT_START);
            block.with_space();
            block.with_atom(self.content.as_str());
        }

        block
    }
}

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;

    #[test]
    fn test_comment_normal() {
        let mut comment = super::Comment::new()
            .append("Hello, ")
            .append("World!");
        assert_eq!(
            comment.render_string(),
            "// Hello, World!"
        );
    }

    #[test]
    fn test_comment_block() {
        let mut comment = super::Comment::new()
            .append("Hello\n")
            .append("World\n");

        assert_eq!(
            comment.render_string(),
            "/*\n * Hello\n * World\n */"
        )
    }
}