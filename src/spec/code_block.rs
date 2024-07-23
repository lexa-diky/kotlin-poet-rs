use crate::io::{CodeBuffer, RenderKotlin};
use crate::tokens;

/// Node of a code block that can be rendered to a Kotlin code.
/// You can treat this nodes as commands for rendering, like "add atom", "add new line", etc.
#[derive(Debug, PartialEq, Clone)]
pub enum CodeBlockNode {
    Atom(CodeBuffer),
    Space,
    NewLine,
    Indent(usize),
    Unindent(usize),
}

/// Plain list of nodes that can be rendered to a Kotlin code.
#[derive(Debug, PartialEq, Clone)]
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
    /// Please avoid using it in [RenderKotlin::render], prefer [CodeBlock::empty] and [CodeBlock::with_atom] instead.
    pub fn atom(text: &str) -> CodeBlock {
        CodeBlock {
            nodes: vec![
                CodeBlockNode::Atom(
                    CodeBuffer::from(text)
                )
            ],
        }
    }

    /// Creates code block with a single atom node and empty line.
    /// Please avoid using it in [RenderKotlin::render], prefer [CodeBlock::empty] and [CodeBlock::with_atom] instead.
    #[deprecated]
    pub fn statement(text: &str) -> CodeBlock {
        let mut cb = CodeBlock::empty();
        cb.with_statement(text);
        cb
    }

    /// Shortcut for [RenderKotlin::with_atom] + [RenderKotlin::with_new_line]
    pub fn with_statement(&mut self, text: &str) {
        self.with_atom(text);
        self.with_new_line();
    }

    /// Embeds all node from [code_block] into [self].
    pub fn with_nested(&mut self, code_block: CodeBlock) {
        for node in code_block.nodes {
            if let CodeBlockNode::Atom(buffer) = node {
                self.with_atom(buffer.as_string().as_str());
                continue;
            };
            self.nodes.push(node);
        }
    }

    /// Adds [CodeBlockNode::Indent] with value 1.
    pub fn with_indent(&mut self) {
        self.with_indent_value(1);
    }

    fn with_indent_value(&mut self, value: usize) {
        if value == 0 {
            return;
        }
        if let Some(CodeBlockNode::Indent(last_value)) = self.nodes.last_mut() {
            *last_value += value;
            return;
        }
        self.nodes.push(CodeBlockNode::Indent(value));
    }

    /// Adds [CodeBlockNode::Unindent] with value 1
    pub fn with_unindent(&mut self) {
        self.with_unindent_value(1);
    }

    fn with_unindent_value(&mut self, value: usize) {
        if value == 0 {
            return;
        }
        if let Some(CodeBlockNode::Unindent(last_value)) = self.nodes.last_mut() {
            *last_value += value;
            return;
        }
        self.nodes.push(CodeBlockNode::Unindent(value));
    }

    /// Adds [CodeBlockNode::NewLine]
    pub fn with_new_line(&mut self) {
        self.nodes.push(CodeBlockNode::NewLine);
    }

    /// Adds [CodeBlockNode::Atom]
    pub fn with_atom(&mut self, text: &str) {
        if let Some(CodeBlockNode::Atom(inner_buffer)) = self.nodes.last_mut() {
            inner_buffer.push(text);
            return;
        }
        self.nodes.push(CodeBlockNode::Atom(CodeBuffer::from(text)));
    }

    /// Adds [CodeBlockNode::Space]
    pub fn with_space(&mut self) {
        if matches!(self.nodes.last(), Some(CodeBlockNode::Space)) {
            return; // no double spaces
        }
        self.nodes.push(CodeBlockNode::Space);
    }

    /// Surrounds first parameter [block] with curly brackets + indent and adds it to [self].
    pub fn with_curly_brackets<F>(&mut self, block: F)
    where
        F: FnOnce(&mut CodeBlock),
    {
        let mut inner_code = CodeBlock::empty();

        self.with_atom(tokens::CURLY_BRACKET_LEFT);
        self.with_new_line();
        self.with_indent();
        block(&mut inner_code);
        self.with_nested(inner_code);
        self.with_unindent();
        self.with_atom(tokens::CURLY_BRACKET_RIGHT);
    }

    /// Surrounds first parameter [block] with round brackets and adds it to [self].
    pub fn with_round_brackets<F>(&mut self, block: F)
    where
        F: FnOnce(&mut CodeBlock),
    {
        let mut inner_code = CodeBlock::empty();

        self.with_atom(tokens::ROUND_BRACKET_LEFT);
        block(&mut inner_code);
        self.with_nested(inner_code);
        self.with_atom(tokens::ROUND_BRACKET_RIGHT);
    }

    /// Adds all elements from [elements] with comma separation, except for last one
    pub fn with_comma_separated<F>(&mut self, elements: &[F])
    where
        F: RenderKotlin,
    {
        let mut code = CodeBlock::empty();
        let len = elements.len();
        for (index, renderable) in elements.iter().enumerate() {
            code.with_nested(renderable.render());
            if index != len - 1 {
                code.with_atom(tokens::COMMA);
                code.with_space();
            }
        }

        self.with_nested(code);
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
                    root_buffer.push(buffer.as_string().as_str());
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
        root_buffer.as_string()
    }
}

//noinspection RsImplToString
#[allow(clippy::to_string_trait_impl)]
impl ToString for CodeBlock {
    fn to_string(&self) -> String {
        self.render()
    }
}
