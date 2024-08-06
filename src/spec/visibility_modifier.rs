use crate::io::RenderKotlin;
use crate::spec::CodeBlock;
use crate::tokens;

/// Represents [Kotlin's visibility modifier](https://kotlinlang.org/docs/visibility-modifiers.html)
///
/// Defaults to [VisibilityModifier::Public].
/// Lack of visibility modifier means that the member is public.
#[derive(PartialEq, Debug, Clone, Default)]
pub enum VisibilityModifier {
    /// `public` means that any client who sees the declaring class sees its `public` members.
    #[default]
    Public,
    /// `internal` means that any client inside this module who sees the declaring class sees its `internal` members.
    Internal,
    /// `private` means that the member is visible inside this class only (including all its members).
    Private,
    /// `protected` means that the member has the same visibility as one marked as private, but that it is also visible in subclasses.
    Protected,
}

impl RenderKotlin for VisibilityModifier {
    fn render(&self) -> CodeBlock {
        let keyword = match self {
            VisibilityModifier::Public => tokens::keyword::PUBLIC,
            VisibilityModifier::Internal => tokens::keyword::INTERNAL,
            VisibilityModifier::Private => tokens::keyword::PRIVATE,
            VisibilityModifier::Protected => tokens::keyword::PROTECTED
        };

        CodeBlock::atom(keyword)
    }
}
