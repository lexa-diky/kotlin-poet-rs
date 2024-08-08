use crate::io::RenderKotlin;
use crate::spec::CodeBlock;
use crate::tokens;

/// Represents a Kotlin documentation comment in KDoc format.
///
/// Entities that support KDoc should usually store it as singular instance and merge multiple KDocs into one.
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
    fn render_into(&self, block: &mut CodeBlock) {
        block.push_atom(tokens::KDOC_COMMENT_START);
        block.push_new_line();
        let split = self.content.split(tokens::NEW_LINE)
            .enumerate().collect::<Vec<_>>();
        let split_len = split.len();
        for (idx, line) in split {
            if idx == split_len - 1 && line.is_empty() {
                break;
            }

            block.push_atom(tokens::KDOC_COMMENT_MIDDLE);
            block.push_space();
            block.push_atom(line);
            block.push_new_line();
        }
        block.push_atom(tokens::KDOC_COMMENT_END);
    }
}

/// Utility wrapper around [Option<KDoc>] that allows to store it as an option and merge multiple KDocs into one.
#[derive(Default, Clone, Debug)]
pub(crate) struct KdocSlot(Option<KDoc>);
impl RenderKotlin for KdocSlot {
    fn render_into(&self, block: &mut CodeBlock) {
        if let Some(kdoc) = &self.0 {
            block.push_renderable(kdoc);
            block.push_new_line()
        }
    }
}

impl KdocSlot {
    /// Merges [other] into this KDoc slot.
    /// If [None] sets [other] as current value.
    pub(crate) fn merge(&mut self, other: KDoc) {
        match self.0 {
            None => { self.0 = Some(other) }
            Some(ref mut old) => {
                old.content.push_str(tokens::NEW_LINE);
                old.content.push_str(other.content.as_str());
            }
        };
    }
}

macro_rules! mixin_kdoc_mutators {
    () => {
        /// Adds [KDoc] to this entity.
        /// In case of multiple calls, KDocs will be merged, see [KDoc::merge].
        pub fn kdoc(mut self, kdoc: crate::spec::KDoc) -> Self {
            self.kdoc.merge(kdoc);
            self
        }

        /// Adds [KDoc] made from string to this entity.
        /// In case of multiple calls, KDocs will be merged, see [KDoc::merge].
        pub fn kdoc_str(mut self, text: &str) -> Self {
            self.kdoc.merge(crate::spec::KDoc::from(text));
            self
        }
    };
}

pub(crate) use mixin_kdoc_mutators;

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;

    #[test]
    fn test_kdoc_render() {
        let comment = super::KDoc::new()
            .append("Hello\n")
            .append("World\n");

        assert_eq!(
            comment.render_string(),
            "/**\n * Hello\n * World\n */"
        )
    }

    #[test]
    fn test_comment_block() {
        let comment = super::KDoc::new()
            .append("Hello\n")
            .merge(super::KDoc::new().append("World\n"));

        assert_eq!(
            comment.render_string(),
            "/**\n * Hello\n * \n * World\n */"
        )
    }
}
