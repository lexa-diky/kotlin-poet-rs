use crate::io::{RenderKotlin};
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
            AccessModifier::Public => "public",
            AccessModifier::Internal => "internal",
            AccessModifier::Private => "private",
            AccessModifier::Protected => "protected"
        };

        CodeBlock::atom(keyword)
    }
}
