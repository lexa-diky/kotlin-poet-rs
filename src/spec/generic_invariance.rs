use crate::io::RenderKotlin;
use crate::spec::CodeBlock;
use crate::tokens;

/// Type of generic [parameter invariance](https://kotlinlang.org/docs/generics.html#variance-and-wildcards-in-java).
///
/// Conventionally possibly non-invariant generics are expressed via `Option<GenericInvariance>`
#[derive(Debug, Clone)]
pub enum GenericInvariance {
    /// Corresponds to [tokens::keyword::IN]
    In,
    /// Corresponds to [tokens::keyword::OUT]
    Out
}

impl RenderKotlin for GenericInvariance {
    fn render(&self) -> CodeBlock {
        match self {
            GenericInvariance::In => CodeBlock::atom(tokens::keyword::IN),
            GenericInvariance::Out => CodeBlock::atom(tokens::keyword::OUT),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::io::RenderKotlin;
    use super::*;

    #[test]
    fn test_generic_invariance() {
        let invariance = GenericInvariance::In;
        assert_eq!(invariance.render().to_string(), "in");

        let invariance = GenericInvariance::Out;
        assert_eq!(invariance.render().to_string(), "out");
    }
}