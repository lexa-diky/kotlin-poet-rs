use crate::io::{RenderContext, RenderKotlin};
use crate::spec::CodeBlock;

#[derive(Debug, Clone)]
pub enum MemberInheritanceModifier {
    Open,
    Final,
    Default,
    Abstract,
}

impl RenderKotlin for MemberInheritanceModifier {
    fn render(&self, context: RenderContext) -> CodeBlock {
        let text = match self {
            MemberInheritanceModifier::Open => "open",
            MemberInheritanceModifier::Final => "final",
            MemberInheritanceModifier::Default => "",
            MemberInheritanceModifier::Abstract => "abstract"
        };

        CodeBlock::atom(text)
    }
}

#[cfg(test)]
mod test {
    use crate::io::RenderKotlin;
    use crate::spec::inheritance_modifier::MemberInheritanceModifier;

    #[test]
    fn test_render() {
        assert_eq!(MemberInheritanceModifier::Open.render_without_context(), "open");
        assert_eq!(MemberInheritanceModifier::Final.render_without_context(), "final");
        assert_eq!(MemberInheritanceModifier::Default.render_without_context(), "");
    }
}