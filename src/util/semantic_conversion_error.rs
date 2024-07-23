use std::fmt::Display;

/// Error type for semantic conversion errors.
/// For example when converting [crate::spec::ClassLikeTypeName] to [crate::spec::KotlinFile].
#[derive(Debug)]
pub struct SemanticConversionError {
    message: String
}

impl SemanticConversionError {

    pub(crate) fn new(message: &str) -> Self {
        SemanticConversionError {
            message: message.to_string()
        }
    }
}

impl Display for SemanticConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SemanticConversionError {}