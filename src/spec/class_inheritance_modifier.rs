use crate::io::RenderKotlin;
use crate::spec::CodeBlock;
use crate::tokens;

#[derive(Debug, Clone)]
pub enum ClassInheritanceModifier {
    Open,
    Final,
    Interface,
    Abstract,
    Sealed,
    Object,
    Enum,
}

impl RenderKotlin for ClassInheritanceModifier {
    fn render(&self) -> CodeBlock {
        let text = match self {
            ClassInheritanceModifier::Open => tokens::keyword::OPEN,
            ClassInheritanceModifier::Final => tokens::keyword::FINAL,
            ClassInheritanceModifier::Abstract => tokens::keyword::ABSTRACT,
            ClassInheritanceModifier::Sealed => tokens::keyword::SEALED,
            ClassInheritanceModifier::Interface => tokens::keyword::INTERFACE,
            ClassInheritanceModifier::Object => tokens::keyword::OBJECT,
            ClassInheritanceModifier::Enum => tokens::keyword::ENUM
        };

        CodeBlock::atom(text)
    }
}

#[cfg(test)]
mod test {
    use crate::io::RenderKotlin;
    use crate::spec::ClassInheritanceModifier;

    #[test]
    fn test_render() {
        assert_eq!(ClassInheritanceModifier::Open.render_string(), "open");
        assert_eq!(ClassInheritanceModifier::Final.render_string(), "final");
        assert_eq!(ClassInheritanceModifier::Abstract.render_string(), "abstract");
        assert_eq!(ClassInheritanceModifier::Sealed.render_string(), "sealed");
    }
}