use crate::io::{RenderContext, RenderKotlin};

#[derive(PartialEq, Debug, Clone)]
pub enum AccessModifier {
    Public,
    Internal,
    Private,
    Protected
}

impl RenderKotlin for AccessModifier {
    fn render(&self, context: RenderContext) -> String {
        match self {
            AccessModifier::Public => "public".to_string(),
            AccessModifier::Internal => "internal".to_string(),
            AccessModifier::Private => "private".to_string(),
            AccessModifier::Protected => "protected".to_string()
        }
    }
}
