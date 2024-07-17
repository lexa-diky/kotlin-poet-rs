use crate::io::{CodeBuffer, RenderContext, RenderKotlin};
use crate::io::tokens::{CURLY_BRACE_LEFT, CURLY_BRACE_RIGHT, INDENT, NEW_LINE, NOTHING};

#[derive(Debug, PartialEq, Clone)]
pub enum  CodeBlockNode {
    Statement(CodeBuffer),
    Indent(usize),
    Unindent(usize)
}

impl RenderKotlin for CodeBlockNode {

    fn render(&self, context: RenderContext) -> String {
        match self {
            CodeBlockNode::Statement(statement) => {
                statement.to_string()
            }
            CodeBlockNode::Indent(size) => {
                INDENT.to_string()
            }
            CodeBlockNode::Unindent(size) => {
                NOTHING.to_string()
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CodeBlock {
    pub nodes: Vec<CodeBlockNode>,
}

impl CodeBlock {

    pub fn empty() -> CodeBlock {
        return CodeBlock {
            nodes: vec![]
        }
    }

    pub fn statement(mut self, text: &str) -> CodeBlock {
        self.with_statement(text);
        return self
    }

    pub fn with_statement(&mut self, text: &str) {
        self.nodes.push(
            CodeBlockNode::Statement(
                CodeBuffer::from(text)
            )
        );
    }

    pub fn nest(mut self, code_block: CodeBlock) -> CodeBlock {
        for node in code_block.nodes {
            self.nodes.push(node);
        }
        return self
    }

    pub fn indent(mut self) -> CodeBlock {
        self.nodes.push(CodeBlockNode::Indent(1));
        return self
    }

    pub fn unindent(mut self) -> CodeBlock {
        self.nodes.push(CodeBlockNode::Unindent(1));
        return self
    }

    pub fn wrap_in_scope(mut self) -> CodeBlock {
        self.nodes.insert(0, CodeBlockNode::Statement(CodeBuffer::from(CURLY_BRACE_LEFT)));
        self.nodes.insert(1, CodeBlockNode::Indent(1));
        self.nodes.push(CodeBlockNode::Unindent(1));
        self.nodes.push(CodeBlockNode::Statement(CodeBuffer::from(CURLY_BRACE_RIGHT)));
        self
    }

    fn mk_indent(value: usize) -> String {
        let mut buff = String::new();
        for _ in 0..value {
            buff.push_str(INDENT);
        }
        buff
    }
}

impl RenderKotlin for CodeBlock {

    fn render(&self, context: RenderContext) -> String {
        let mut buffer = CodeBuffer::default();
        let mut indent = context.level();
        for node in &self.nodes {
            match node {
                CodeBlockNode::Statement(_) => {
                    buffer.push(Self::mk_indent(indent).as_str());
                    buffer.push(node.render(context).as_str());
                    buffer.push(NEW_LINE)
                }
                CodeBlockNode::Indent(size) => {
                    indent += size;
                }
                CodeBlockNode::Unindent(size) => {
                    indent -= size;
                }
            }
        }
        return buffer.to_string()
    }
}
