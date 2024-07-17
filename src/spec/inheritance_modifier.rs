use crate::io::RenderKotlin;

#[derive(Debug, Clone)]
pub enum MemberInheritanceModifier {
    Open, Final, Default, Abstract
}

impl RenderKotlin for MemberInheritanceModifier {
    fn render(&self) -> String {
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
        assert_eq!(MemberInheritanceModifier::Open.render(), "open");
        assert_eq!(MemberInheritanceModifier::Final.render(), "final");
        assert_eq!(MemberInheritanceModifier::Default.render(), "");
    }
}