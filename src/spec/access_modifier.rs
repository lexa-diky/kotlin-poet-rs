use crate::io::{RenderKotlin, tokens};
use crate::spec::CodeBlock;

#[derive(PartialEq, Debug, Clone)]
pub enum AccessModifier {
    Public,
    Internal,
    Private,
    Protected
}

impl RenderKotlin for AccessModifier {

    fn render(&self) -> CodeBlock {
        let keyword = match self {
            AccessModifier::Public => tokens::KW_PUBLIC,
            AccessModifier::Internal => tokens::KW_INTERNAL,
            AccessModifier::Private => tokens::KW_PRIVATE,
            AccessModifier::Protected => tokens::KW_PROTECTED
        };

        CodeBlock::atom(keyword)
    }
}
