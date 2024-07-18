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
    fn render(&self) -> CodeBlock {
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
        assert_eq!(MemberInheritanceModifier::Open.render_string_in_root(), "open");
        assert_eq!(MemberInheritanceModifier::Final.render_string_in_root(), "final");
        assert_eq!(MemberInheritanceModifier::Default.render_string_in_root(), "");
    }
}