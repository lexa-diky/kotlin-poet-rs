use crate::io::RenderKotlin;

#[derive(PartialEq, Debug, Clone)]
pub enum AccessModifier {
    Public,
    Internal,
    Private,
    Protected
}

impl RenderKotlin for AccessModifier {
    fn render(&self) -> String {
        match self {
            AccessModifier::Public => "public".to_string(),
            AccessModifier::Internal => "internal".to_string(),
            AccessModifier::Private => "private".to_string(),
            AccessModifier::Protected => "protected".to_string()
        }
    }
}