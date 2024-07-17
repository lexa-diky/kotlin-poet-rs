use crate::io::{CodeBuffer, RenderContext};
use crate::io::tokens::{CURLY_BRACE_LEFT, CURLY_BRACE_RIGHT, INDENT, NEW_LINE};

#[derive(Debug, PartialEq, Clone)]
pub enum  CodeBlockNode {
    Statement(CodeBuffer),
    Atom(CodeBuffer),
    Space,
    Indent(usize),
    Unindent(usize)
}

#[derive(Debug, PartialEq, Clone)]
pub struct CodeBlock {
    pub nodes: Vec<CodeBlockNode>,
    pub default_indent: usize
}

impl CodeBlock {

    pub fn empty() -> CodeBlock {
        return CodeBlock {
            nodes: vec![],
            default_indent: 0
        }
    }

    pub fn from(context: RenderContext) -> CodeBlock {
        return CodeBlock {
            nodes: vec![],
            default_indent: context.level()
        }
    }

    pub fn atom(text: &str) -> CodeBlock {
        return CodeBlock {
            nodes: vec![
                CodeBlockNode::Atom(
                    CodeBuffer::from(text)
                )
            ],
            default_indent: 0
        }
    }

    pub fn statement(text: &str) -> CodeBlock {
        return CodeBlock {
            nodes: vec![
                CodeBlockNode::Statement(
                    CodeBuffer::from(text)
                )
            ],
            default_indent: 0
        }
    }

    pub fn with_statement(&mut self, text: &str) {
        self.nodes.push(
            CodeBlockNode::Statement(
                CodeBuffer::from(text)
            )
        );
    }

    pub fn with_nested(&mut self, code_block: CodeBlock) {
        self.nodes.push(CodeBlockNode::Indent(code_block.default_indent));
        for node in code_block.nodes {
            self.nodes.push(node);
        }
        self.nodes.push(CodeBlockNode::Unindent(code_block.default_indent));
    }

    pub fn with_indent(&mut self) {
        self.nodes.push(CodeBlockNode::Indent(1));
    }

    pub fn with_unindent(&mut self) {
        self.nodes.push(CodeBlockNode::Unindent(1));
    }

    pub fn with_atom(&mut self, text: &str) {
        self.nodes.push(CodeBlockNode::Atom(CodeBuffer::from(text)));
    }

    pub fn with_space(&mut self) {
        self.nodes.push(CodeBlockNode::Space);
    }

    pub fn wrap_in_scope(mut self) -> CodeBlock {
        self.nodes.insert(0, CodeBlockNode::Statement(CodeBuffer::from(CURLY_BRACE_LEFT)));
        self.nodes.insert(1, CodeBlockNode::Indent(1));
        self.nodes.push(CodeBlockNode::Unindent(1));
        self.nodes.push(CodeBlockNode::Statement(CodeBuffer::from(CURLY_BRACE_RIGHT)));
        self
    }

    pub fn render(&self) -> String {
        let mut root_buffer = CodeBuffer::default();
        let mut indent = 0;
        for node in &self.nodes {
            match node {
                CodeBlockNode::Statement(buf) => {
                    root_buffer.push(Self::mk_indent(indent).as_str());
                    root_buffer.push(buf.to_string().as_str());
                    root_buffer.push(NEW_LINE)
                }
                CodeBlockNode::Atom(buffer) => {
                    root_buffer.push(buffer.to_string().as_str());
                }
                CodeBlockNode::Indent(size) => {
                    indent += size;
                }
                CodeBlockNode::Unindent(size) => {
                    indent -= size;
                }
                CodeBlockNode::Space => {
                    root_buffer.push(" ")
                }
            }
        }
        return root_buffer.to_string()
    }

    fn mk_indent(value: usize) -> String {
        let mut buff = String::new();
        for _ in 0..value {
            buff.push_str(INDENT);
        }
        buff
    }
}
