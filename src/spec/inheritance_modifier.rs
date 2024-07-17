use crate::io::{RenderContext, RenderKotlin};

#[derive(Debug, Clone)]
pub enum MemberInheritanceModifier {
    Open, Final, Default, Abstract
}

impl RenderKotlin for MemberInheritanceModifier {
    fn render(&self, context: RenderContext) -> String {
        match self {
            MemberInheritanceModifier::Open => "open".to_string(),
            MemberInheritanceModifier::Final => "final".to_string(),
            MemberInheritanceModifier::Default => "".to_string(),
            MemberInheritanceModifier::Abstract => "abstract".to_string()
        }
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