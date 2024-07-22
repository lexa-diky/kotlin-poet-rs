use crate::io::{RenderKotlin, tokens};
use crate::spec::CodeBlock;

#[derive(Debug, Clone)]
pub enum ClassInheritanceModifier {
    Open,
    Final,
    Interface,
    Abstract,
    Sealed,
    Object
}

impl RenderKotlin for ClassInheritanceModifier {
    fn render(&self) -> CodeBlock {
        let text = match self {
            ClassInheritanceModifier::Open => tokens::KW_OPEN,
            ClassInheritanceModifier::Final => tokens::KW_FINAL,
            ClassInheritanceModifier::Abstract => tokens::KW_ABSTRACT,
            ClassInheritanceModifier::Sealed => tokens::KW_SEALED,
            ClassInheritanceModifier::Interface => tokens::KW_INTERFACE,
            ClassInheritanceModifier::Object => tokens::KW_OBJECT
        };

        CodeBlock::atom(text)
    }
}

#[cfg(test)]
mod test {
    use crate::io::RenderKotlin;
    use crate::spec::ClassInheritanceModifier;
    use crate::spec::member_inheritance_modifier::MemberInheritanceModifier;

    #[test]
    fn test_render() {
        assert_eq!(ClassInheritanceModifier::Open.render_string_in_root(), "open");
        assert_eq!(ClassInheritanceModifier::Final.render_string_in_root(), "final");
        assert_eq!(ClassInheritanceModifier::Abstract.render_string_in_root(), "abstract");
        assert_eq!(ClassInheritanceModifier::Sealed.render_string_in_root(), "sealed");
    }
}