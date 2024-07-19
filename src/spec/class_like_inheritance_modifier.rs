use crate::io::{RenderKotlin, tokens};
use crate::spec::CodeBlock;

#[derive(Debug, Clone)]
pub enum ClassLikeInheritanceModifier {
    Open,
    Final,
    Abstract,
    Sealed
}

impl RenderKotlin for ClassLikeInheritanceModifier {
    fn render(&self) -> CodeBlock {
        let text = match self {
            ClassLikeInheritanceModifier::Open => tokens::KW_OPEN,
            ClassLikeInheritanceModifier::Final => tokens::KW_FINAL,
            ClassLikeInheritanceModifier::Abstract => tokens::KW_ABSTRACT,
            ClassLikeInheritanceModifier::Sealed => tokens::KW_SEALED
        };

        CodeBlock::atom(text)
    }
}

#[cfg(test)]
mod test {
    use crate::io::RenderKotlin;
    use crate::spec::ClassLikeInheritanceModifier;
    use crate::spec::member_inheritance_modifier::MemberInheritanceModifier;

    #[test]
    fn test_render() {
        assert_eq!(ClassLikeInheritanceModifier::Open.render_string_in_root(), "open");
        assert_eq!(ClassLikeInheritanceModifier::Final.render_string_in_root(), "final");
        assert_eq!(ClassLikeInheritanceModifier::Abstract.render_string_in_root(), "abstract");
        assert_eq!(ClassLikeInheritanceModifier::Sealed.render_string_in_root(), "sealed");
    }
}