use crate::io::{CodeBuffer, tokens};

#[derive(Debug, PartialEq, Clone)]
pub enum CodeBlockNode {
    Atom(CodeBuffer),
    Space,
    NewLine,
    Indent(usize),
    Unindent(usize),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CodeBlock {
    nodes: Vec<CodeBlockNode>,
}

impl CodeBlock {
    pub fn empty() -> CodeBlock {
        CodeBlock {
            nodes: vec![],
        }
    }

    pub fn atom(text: &str) -> CodeBlock {
        CodeBlock {
            nodes: vec![
                CodeBlockNode::Atom(
                    CodeBuffer::from(text)
                )
            ],
        }
    }

    pub fn statement(text: &str) -> CodeBlock {
        let mut cb = CodeBlock::empty();
        cb.with_statement(text);
        cb
    }

    pub fn with_statement(&mut self, text: &str) {
        self.with_atom(text);
        self.with_new_line();
    }

    pub fn with_nested(&mut self, code_block: CodeBlock) {
        for node in code_block.nodes {
            if let CodeBlockNode::Atom(buffer) = node {
                self.with_atom(buffer.as_string().as_str());
                continue;
            };
            self.nodes.push(node);
        }
    }

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

    pub fn with_new_line(&mut self) {
        self.nodes.push(CodeBlockNode::NewLine);
    }

    pub fn with_atom(&mut self, text: &str) {
        if let Some(CodeBlockNode::Atom(inner_buffer)) = self.nodes.last_mut() {
            inner_buffer.push(text);
            return;
        }
        self.nodes.push(CodeBlockNode::Atom(CodeBuffer::from(text)));
    }

    pub fn with_space(&mut self) {
        if matches!(self.nodes.last(), Some(CodeBlockNode::Space)) {
            return; // no double spaces
        }
        self.nodes.push(CodeBlockNode::Space);
    }

    pub fn with_scope<F>(&mut self, block: F)
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

    fn render(&self) -> String {
        let mut root_buffer = CodeBuffer::default();
        let mut indent = 0;

        for node in &self.nodes {
            match node {
                CodeBlockNode::Atom(buffer) => {
                    if matches!(root_buffer.last_char(), Some(tokens::NEW_LINE_CH)) {
                        root_buffer.push(CodeBlock::mk_indent(indent).as_str());
                    }
                    root_buffer.push(buffer.as_string().as_str());
                }
                CodeBlockNode::Indent(size) => {
                    indent += size;
                }
                CodeBlockNode::Unindent(size) => {
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

    fn mk_indent(value: usize) -> String {
        tokens::INDENT.repeat(value)
    }
}

//noinspection RsImplToString
#[allow(clippy::to_string_trait_impl)]
impl ToString for CodeBlock {
    fn to_string(&self) -> String {
        self.render()
    }
}
