use crate::io::RenderKotlin;
use crate::spec::CodeBlock;
use crate::tokens;

/// Inheritance modifiers applicable to class-like entities.
/// Enables converting class to interface, enum e.t.c.
/// Defaults to [ClassInheritanceModifier::Final], lack of inheritance modifier is represented as default.
///
/// Class / File members use [crate::spec::MemberInheritanceModifier] instead.
#[derive(Debug, Clone, Default)]
pub enum ClassInheritanceModifier {
    /// Corresponds open classes a.k.a. classes that can be inherited.
    Open,
    /// Default, no inheritance allowed.
    #[default]
    Final,
    /// Denotes that class-like entity is interface.
    Interface,
    /// Denotes abstract classes
    Abstract,
    /// Denotes sealed class, for simplicity abstract keyword is omitted
    Sealed,
    /// Denotes that class-like entry is standalone object, for companion objects see [crate::spec::CompanionObject]
    Object,
    /// Denotes that class-like entity is enum
    Enum,
    /// Denotes that class-like entity is data, for simplicity final keyword is omitted
    Data,
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
            ClassInheritanceModifier::Enum => tokens::keyword::ENUM,
            ClassInheritanceModifier::Data => tokens::keyword::DATA
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