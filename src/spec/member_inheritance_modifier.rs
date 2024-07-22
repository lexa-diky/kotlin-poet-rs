use crate::io::{RenderKotlin, tokens};
use crate::spec::CodeBlock;

#[derive(Debug, Clone)]
pub enum MemberInheritanceModifier {
    Open,
    Final,
    Abstract,
}

impl RenderKotlin for MemberInheritanceModifier {
    fn render(&self) -> CodeBlock {
        let text = match self {
            MemberInheritanceModifier::Open => tokens::keyword::OPEN,
            MemberInheritanceModifier::Final => tokens::keyword::FINAL,
            MemberInheritanceModifier::Abstract => tokens::keyword::ABSTRACT
        };

        CodeBlock::atom(text)
    }
}

#[cfg(test)]
mod test {
    use crate::io::RenderKotlin;
    use crate::spec::member_inheritance_modifier::MemberInheritanceModifier;

    #[test]
    fn test_render() {
        assert_eq!(MemberInheritanceModifier::Open.render_string_in_root(), "open");
        assert_eq!(MemberInheritanceModifier::Final.render_string_in_root(), "final");
        assert_eq!(MemberInheritanceModifier::Abstract.render_string_in_root(), "abstract");
    }
}