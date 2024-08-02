mod semantic_conversion_error;
pub use semantic_conversion_error::SemanticConversionError;

/// A macro that implements `From<&str>` for a type that implements `FromStr`.
/// Panics if the conversion fails.
macro_rules! yolo_from_str {
    ($base:ident) => {
        impl From<&str> for $base {
            fn from(s: &str) -> Self {
                $base::from_str(s).unwrap()
            }
        }
    };
}

pub(crate) use yolo_from_str;