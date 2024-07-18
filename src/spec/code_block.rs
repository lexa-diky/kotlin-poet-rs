use crate::io::{CodeBuffer, tokens};

#[derive(Debug, PartialEq, Clone)]
pub enum  CodeBlockNode {
    Atom(CodeBuffer),
    Space,
    NewLine,
    Indent(usize),
    Unindent(usize)
}

#[derive(Debug, PartialEq, Clone)]
pub struct CodeBlock {
    nodes: Vec<CodeBlockNode>,
}

impl CodeBlock {

    pub fn empty() -> CodeBlock {
        return CodeBlock {
            nodes: vec![],
        }
    }

    pub fn atom(text: &str) -> CodeBlock {
        return CodeBlock {
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
        return cb
    }

    pub fn with_statement(&mut self, text: &str) {
        self.with_atom(text);
        self.with_new_line();
    }

    pub fn with_nested(&mut self, code_block: CodeBlock) {
        for node in code_block.nodes {
            if let CodeBlockNode::Atom(buffer) = node {
                self.with_atom(buffer.to_string().as_str());
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
            return
        }
        self.nodes.push(CodeBlockNode::Indent(value));
    }

    pub fn with_unindent(&mut self) {
        self.nodes.push(CodeBlockNode::Unindent(1));
    }

    pub fn with_new_line(&mut self) {
        self.nodes.push(CodeBlockNode::NewLine);
    }

    pub fn with_atom(&mut self, text: &str) {
        if let Some(last) = self.nodes.last_mut() {
            if let CodeBlockNode::Atom(inner_buffer) = last {
                inner_buffer.push(text);
                return;
            }
        }
        self.nodes.push(CodeBlockNode::Atom(CodeBuffer::from(text)));
    }

    pub fn with_space(&mut self) {
        self.nodes.push(CodeBlockNode::Space);
    }

    fn render(&self) -> String {
        let mut root_buffer = CodeBuffer::default();
        let mut indent = 0;
        let mut last_rendered: Option<&CodeBlockNode> = None;

        for node in &self.nodes {
            match node {
                CodeBlockNode::Atom(buffer) => {
                    if matches!(root_buffer.last_char(), Some(tokens::NEW_LINE_CH)) {
                        root_buffer.push(CodeBlock::mk_indent(indent).as_str());
                    }
                    root_buffer.push(buffer.to_string().as_str());
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
            last_rendered = Some(node)
        }
        return root_buffer.to_string()
    }

    fn mk_indent(value: usize) -> String {
        let mut buff = String::new();
        for _ in 0..value {
            buff.push_str(tokens::INDENT);
        }
        buff
    }
}

impl ToString for CodeBlock {

    fn to_string(&self) -> String {
        self.render()
    }
}
