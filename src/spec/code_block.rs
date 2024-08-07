use crate::io::{CodeBuffer, RenderKotlin};
use crate::tokens;

/// Node of a code block that can be rendered to a Kotlin code.
/// You can treat these nodes as commands for rendering, like "add atom", "add new line", etc.
#[derive(Debug, Clone)]
pub(crate) enum CodeBlockNode {
    Atom(String),
    Space,
    NewLine,
    Indent(usize),
    Unindent(usize),
}

/// Plain list of nodes that can be rendered to a Kotlin code.
#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub(crate) nodes: Vec<CodeBlockNode>,
}

impl CodeBlock {
    /// Create an empty code block.
    pub fn empty() -> CodeBlock {
        CodeBlock {
            nodes: vec![],
        }
    }

    /// Creates code block with a single atom node.
    /// Please avoid using it in [RenderKotlin::render], prefer [CodeBlock::empty] and [CodeBlock::push_atom] instead.
    pub fn atom(text: &str) -> CodeBlock {
        CodeBlock {
            nodes: vec![
                CodeBlockNode::Atom(
                    text.to_string()
                )
            ],
        }
    }

    /// Creates code block with a single atom node and empty line.
    pub fn statement(text: &str) -> CodeBlock {
        let mut cb = CodeBlock::atom(text);
        cb.push_new_line();
        cb
    }

    /// Embeds all node from [code_block] into [self].
    pub fn push_renderable<T: RenderKotlin>(&mut self, renderable: &T) {
        renderable.render_into(self);
    }

    /// Adds [CodeBlockNode::Indent] with value 1.
    /// In case there is already [CodeBlockNode::Indent] at the end of the list, increments its value.
    pub fn push_indent(&mut self) {
        if let Some(CodeBlockNode::Indent(last_value)) = self.nodes.last_mut() {
            *last_value += 1;
            return;
        }
        self.nodes.push(CodeBlockNode::Indent(1));
    }

    /// Adds [CodeBlockNode::Unindent] with value 1
    /// In case there is already [CodeBlockNode::Unindent] at the end of the list, increments its value.
    pub fn push_unindent(&mut self) {
        if let Some(CodeBlockNode::Unindent(last_value)) = self.nodes.last_mut() {
            *last_value += 1;
            return;
        }
        self.nodes.push(CodeBlockNode::Unindent(1));
    }

    /// Adds [CodeBlockNode::NewLine]
    pub fn push_new_line(&mut self) {
        self.nodes.push(CodeBlockNode::NewLine);
    }

    /// Adds [CodeBlockNode::Atom]
    pub fn push_atom(&mut self, text: &str) {
        if let Some(CodeBlockNode::Atom(inner_buffer)) = self.nodes.last_mut() {
            inner_buffer.push_str(text);
            return;
        }
        self.nodes.push(CodeBlockNode::Atom(text.to_string()));
    }

    /// Adds [CodeBlockNode::Space]
    pub fn push_space(&mut self) {
        if matches!(self.nodes.last(), Some(CodeBlockNode::Space)) {
            return; // no double spaces
        }
        self.nodes.push(CodeBlockNode::Space);
    }

    /// Removes last [CodeBlockNode::Space] if exists
    pub fn pop_space(&mut self) {
        if matches!(self.nodes.last(), Some(CodeBlockNode::Space)) {
            self.nodes.remove(self.nodes.len() - 1);
        }
    }

    /// Surrounds first parameter [block] with curly brackets + indent and adds it to [self].
    pub fn push_curly_brackets<F>(&mut self, block: F)
    where
        F: FnOnce(&mut CodeBlock),
    {
        let mut inner_code = CodeBlock::empty();

        self.push_atom(tokens::CURLY_BRACKET_LEFT);
        self.push_new_line();
        self.push_indent();
        block(&mut inner_code);
        self.push_renderable(&inner_code);
        self.push_unindent();
        self.push_atom(tokens::CURLY_BRACKET_RIGHT);
    }

    /// Surrounds first parameter [block] with round brackets and adds it to [self].
    pub fn push_round_brackets<F>(&mut self, block: F)
    where
        F: FnOnce(&mut CodeBlock),
    {
        let mut inner_code = CodeBlock::empty();

        self.push_atom(tokens::ROUND_BRACKET_LEFT);
        block(&mut inner_code);
        self.push_renderable(&inner_code);
        self.push_atom(tokens::ROUND_BRACKET_RIGHT);
    }

    /// Surrounds first parameter [block] with angle brackets and adds it to [self].
    pub fn push_angle_brackets<F>(&mut self, block: F)
    where
        F: FnOnce(&mut CodeBlock),
    {
        let mut inner_code = CodeBlock::empty();

        self.push_atom(tokens::ANGLE_BRACKET_LEFT);
        block(&mut inner_code);
        self.push_renderable(&inner_code);
        self.push_atom(tokens::ANGLE_BRACKET_RIGHT);
    }

    /// Adds all elements from [elements] with comma separation, except for last one
    pub fn push_comma_separated<F>(&mut self, elements: &[F])
    where
        F: RenderKotlin,
    {
        let mut code = CodeBlock::empty();
        let len = elements.len();
        for (index, renderable) in elements.iter().enumerate() {
            code.push_renderable(renderable);
            if index != len - 1 {
                code.push_atom(tokens::COMMA);
                code.push_space();
            }
        }

        self.push_renderable(&code);
    }

    fn render(&self) -> String {
        let mut root_buffer = CodeBuffer::default();
        let mut indent = 0;

        for node in &self.nodes {
            match node {
                CodeBlockNode::Atom(buffer) => {
                    if matches!(root_buffer.last_char(), Some(tokens::NEW_LINE_CH)) {
                        for _ in 0..indent {
                            root_buffer.push(tokens::INDENT)
                        }
                    }
                    root_buffer.push(buffer.as_str());
                }
                CodeBlockNode::Indent(size) => {
                    indent += size;
                }
                CodeBlockNode::Unindent(size) => {
                    if *size > indent {
                        indent = 0;
                        continue;
                    }
                    indent -= size;
                }
                CodeBlockNode::Space => {
                    root_buffer.push(tokens::SPACE)
                }
                CodeBlockNode::NewLine => {
                    root_buffer.push(tokens::NEW_LINE);
                }
            }
        }

        root_buffer.trim();
        root_buffer.into_string()
    }
}

//noinspection RsImplToString
#[allow(clippy::to_string_trait_impl)]
impl ToString for CodeBlock {
    fn to_string(&self) -> String {
        self.render()
    }
}

impl RenderKotlin for CodeBlock {
    fn render_into(&self, block: &mut CodeBlock) {
        block.nodes.extend(self.nodes.iter().cloned());
    }

    fn render_string(&self) -> String {
        self.to_string()
    }
}