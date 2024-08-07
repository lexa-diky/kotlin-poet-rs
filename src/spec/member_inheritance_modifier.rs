use crate::io::RenderKotlin;
use crate::spec::CodeBlock;
use crate::tokens;

/// Inheritance modifiers applicable to class / file members
#[derive(Debug, Clone)]
pub enum MemberInheritanceModifier {
    /// `open` modifier
    Open,
    /// `final` modifier
    Final,
    /// `abstract` modifier
    Abstract,
}

impl RenderKotlin for MemberInheritanceModifier {
    fn render_into(&self, block: &mut CodeBlock) {
        let text = match self {
            MemberInheritanceModifier::Open => tokens::keyword::OPEN,
            MemberInheritanceModifier::Final => tokens::keyword::FINAL,
            MemberInheritanceModifier::Abstract => tokens::keyword::ABSTRACT
        };

        block.with_atom(text);
    }
}

#[cfg(test)]
mod test {
    use crate::io::RenderKotlin;
    use crate::spec::MemberInheritanceModifier;

    #[test]
    fn test_render() {
        assert_eq!(MemberInheritanceModifier::Open.render_string(), "open");
        assert_eq!(MemberInheritanceModifier::Final.render_string(), "final");
        assert_eq!(MemberInheritanceModifier::Abstract.render_string(), "abstract");
    }
}