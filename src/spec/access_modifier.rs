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
            AccessModifier::Public => tokens::keyword::PUBLIC,
            AccessModifier::Internal => tokens::keyword::INTERNAL,
            AccessModifier::Private => tokens::keyword::PRIVATE,
            AccessModifier::Protected => tokens::keyword::PROTECTED
        };

        CodeBlock::atom(keyword)
    }
}
