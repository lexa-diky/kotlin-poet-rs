use crate::io::{CodeBuffer, RenderKotlin};
use crate::io::tokens::{CURLY_BRACE_LEFT, CURLY_BRACE_RIGHT, INDENT, NEW_LINE, NOTHING};

#[derive(Debug, PartialEq, Clone)]
pub enum  CodeBlockNode {
    Statement(CodeBuffer),
    Indent(),
    Unindent()
}

impl RenderKotlin for CodeBlockNode {

    fn render(&self) -> String {
        match self {
            CodeBlockNode::Statement(statement) => {
                statement.to_string()
            }
            CodeBlockNode::Indent() => {
                INDENT.to_string()
            }
            CodeBlockNode::Unindent() => {
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
        self.nodes.push(
            CodeBlockNode::Statement(
                CodeBuffer::from(text)
            )
        );
        return self
    }

    pub fn nest(mut self, code_block: CodeBlock) -> CodeBlock {
        for node in code_block.nodes {
            self.nodes.push(node);
        }
        return self
    }

    pub fn indent(mut self) -> CodeBlock {
        self.nodes.push(CodeBlockNode::Indent());
        return self
    }

    pub fn unindent(mut self) -> CodeBlock {
        self.nodes.push(CodeBlockNode::Unindent());
        return self
    }

    pub fn wrap_in_scope(mut self) -> CodeBlock {
        self.nodes.insert(0, CodeBlockNode::Statement(CodeBuffer::from(CURLY_BRACE_LEFT)));
        self.nodes.insert(1, CodeBlockNode::Indent());
        self.nodes.push(CodeBlockNode::Unindent());
        self.nodes.push(CodeBlockNode::Statement(CodeBuffer::from(CURLY_BRACE_RIGHT)));
        self
    }
}

impl RenderKotlin for CodeBlock {

    fn render(&self) -> String {
        pub fn mk_indent(value: usize) -> String {
            let mut buff = String::new();
            for _ in 0..value {
                buff.push_str(INDENT);
            }
            buff
        }

        let mut buffer = CodeBuffer::default();
        let mut indent = 0;
        for node in &self.nodes {
            match node {
                CodeBlockNode::Statement(_) => {
                    buffer.push(mk_indent(indent).as_str());
                    buffer.push(node.render().as_str());
                    buffer.push(NEW_LINE)
                }
                CodeBlockNode::Indent() => {
                    indent += 1;
                }
                CodeBlockNode::Unindent() => {
                    indent -= 1;
                }
            }
        }
        return buffer.to_string()
    }
}
