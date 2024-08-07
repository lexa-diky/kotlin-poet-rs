use crate::io::RenderKotlin;
use crate::spec::{VisibilityModifier, CodeBlock, MemberInheritanceModifier, Name, Type, Annotation, KDoc};
use crate::spec::annotation::mixin_annotation_mutators;
use crate::spec::kdoc::{KdocSlot, mixin_kdoc_mutators};
use crate::tokens;

#[derive(Debug, Clone)]
enum PropertyInitializer {
    Value(CodeBlock),
    Delegate(CodeBlock),
}

impl RenderKotlin for PropertyInitializer {
    fn render_into(&self, block: &mut CodeBlock) {
        match self {
            PropertyInitializer::Value(initializer) => {
                block.with_space();
                block.with_atom(tokens::ASSIGN);
                block.with_space();
                block.with_embedded(initializer)
            }
            PropertyInitializer::Delegate(delegate) => {
                block.with_space();
                block.with_atom(tokens::keyword::BY);
                block.with_space();
                block.with_embedded(delegate)
            }
        }
    }
}

/// Represents a [Kotlin property](https://kotlinlang.org/docs/properties.html)
#[derive(Debug, Clone)]
pub struct Property {
    name: Name,
    returns: Type,
    inheritance_modifier: MemberInheritanceModifier,
    visibility_modifier: VisibilityModifier,
    initializer: Option<PropertyInitializer>,
    getter: Option<PropertyGetter>,
    setter: Option<PropertySetter>,
    is_mutable: bool,
    is_const: bool,
    is_override: bool,
    annotations: Vec<Annotation>,
    kdoc: KdocSlot
}

#[derive(Debug, Clone)]
pub struct PropertyGetter {
    code: CodeBlock,
    annotations: Vec<Annotation>
}

impl PropertyGetter {
    pub fn new(code: CodeBlock) -> PropertyGetter {
        PropertyGetter {
            code,
            annotations: Vec::new()
        }
    }

    mixin_annotation_mutators!();
}

impl RenderKotlin for PropertyGetter {
    fn render_into(&self, block: &mut CodeBlock) {
        for annotation in &self.annotations {
            block.with_embedded(annotation);
            block.with_new_line();
        }
        block.with_atom(tokens::keyword::GET);
        block.with_round_brackets(|_| {});
        block.with_space();
        block.with_atom(tokens::CURLY_BRACKET_LEFT);
        block.with_new_line();
        block.with_indent();
        block.with_embedded(&self.code);
        block.with_unindent();
        block.with_atom(tokens::CURLY_BRACKET_RIGHT);
        block.with_new_line();
    }
}

#[derive(Debug, Clone)]
pub struct PropertySetter {
    code: CodeBlock,
    visibility_modifier: VisibilityModifier,
    annotations: Vec<Annotation>
}

impl PropertySetter {
    pub fn new(code: CodeBlock) -> PropertySetter {
        PropertySetter {
            code,
            visibility_modifier: VisibilityModifier::default(),
            annotations: Vec::new()
        }
    }

    pub fn visibility_modifier(mut self, visibility_modifier: VisibilityModifier) -> PropertySetter {
        self.visibility_modifier = visibility_modifier;
        self
    }

    mixin_annotation_mutators!();
}

impl RenderKotlin for PropertySetter {
    fn render_into(&self, block: &mut CodeBlock) {
        for annotation in &self.annotations {
            block.with_embedded(annotation);
            block.with_new_line();
        }
        block.with_atom(tokens::keyword::SET);
        block.with_round_brackets(|parameters_code| {
            parameters_code.with_atom(tokens::CONV_VAR_VALUE);
        });
        block.with_space();
        block.with_curly_brackets(|set_body| {
            set_body.with_embedded(&self.code);
        });
        block.with_new_line();
    }
}

impl Property {
    pub fn new(name: Name, returns: Type) -> Property {
        Property {
            name,
            returns,
            inheritance_modifier: MemberInheritanceModifier::Final,
            visibility_modifier: VisibilityModifier::default(),
            initializer: None,
            getter: None,
            setter: None,
            is_mutable: false,
            is_const: false,
            is_override: false,
            annotations: Vec::new(),
            kdoc: KdocSlot::default()
        }
    }

    /// Sets [VisibilityModifier]
    pub fn visibility_modifier(mut self, visibility_modifier: VisibilityModifier) -> Property {
        self.visibility_modifier = visibility_modifier;
        self
    }

    /// Marks function as `override`
    pub fn overrides(mut self, flag: bool) -> Property {
        self.is_override = flag;
        self
    }

    /// Sets [MemberInheritanceModifier]
    pub fn inheritance_modifier(mut self, inheritance_modifier: MemberInheritanceModifier) -> Property {
        self.inheritance_modifier = inheritance_modifier;
        self
    }

    /// Sets property initializer `val some = <initializer>`
    /// Exclusive with [Property::delegate]
    pub fn initializer(mut self, initializer: CodeBlock) -> Property {
        self.initializer = Some(PropertyInitializer::Value(initializer));
        self
    }

    /// Sets property delegate `val some by <delegate>`
    /// Exclusive with [Property::initializer]
    pub fn delegate(mut self, delegate: CodeBlock) -> Property {
        self.initializer = Some(PropertyInitializer::Delegate(delegate));
        self
    }

    /// Sets [PropertyGetter]
    pub fn getter(mut self, getter: PropertyGetter) -> Property {
        self.getter = Some(getter);
        self
    }

    /// Sets [PropertySetter]
    pub fn setter(mut self, setter: PropertySetter) -> Property {
        self.setter = Some(setter);
        self.is_mutable = true;
        self
    }

    /// Sets property mutability, a.k.a `val` or `var`
    pub fn mutable(mut self, flag: bool) -> Property {
        self.is_mutable = flag;
        self
    }

    /// Adds `const` keyword to property
    pub fn constant(mut self, flag: bool) -> Property {
        self.is_const = flag;
        self
    }

    mixin_annotation_mutators!();
    mixin_kdoc_mutators!();
}

impl RenderKotlin for Property {
    fn render_into(&self, block: &mut CodeBlock) {
        block.with_embedded(&self.kdoc);

        for annotation in &self.annotations {
            block.with_embedded(annotation);
            block.with_new_line();
        }

        block.with_embedded(&self.visibility_modifier);
        block.with_space();
        block.with_embedded(&self.inheritance_modifier);
        block.with_space();

        if self.is_const {
            block.with_atom(tokens::keyword::CONST);
            block.with_space()
        }

        if self.is_override {
            block.with_atom(tokens::keyword::OVERRIDE);
            block.with_space();
        }

        if self.is_mutable {
            block.with_atom(tokens::keyword::VAR);
        } else {
            block.with_atom(tokens::keyword::VAL);
        }
        block.with_space();

        block.with_embedded(&self.name);
        block.with_atom(tokens::COLON);
        block.with_space();
        block.with_embedded(&self.returns);
        block.with_indent();
        if let Some(initializer) = &self.initializer {
            block.with_embedded(initializer);
        }
        if let Some(setter) = &self.setter {
            block.with_embedded(setter);
        }
        if let Some(getter) = &self.getter {
            block.with_embedded(getter);
        }
        block.with_unindent();
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::spec::ClassLikeTypeName;
    use super::*;

    #[test]
    fn property_render() {
        let property = Property::new(
            Name::from("name"),
            Type::string()
        ).initializer(
            CodeBlock::statement("\"\"")
        ).getter(
            PropertyGetter::new(
                CodeBlock::statement("return field")
            )
        ).setter(
            PropertySetter::new(
                CodeBlock::statement("field = value")
            )
        );

        let rendered = property.render_string();
        let expected = "public final var name: kotlin.String = \"\"\n    set(value) {\n        field = value\n    }\n    get() {\n        return field\n    }";
        assert_eq!(rendered, expected);
    }

    #[test]
    fn test_constant() {
        let property = Property::new(Name::from("name"), Type::string())
            .constant(true)
            .initializer(CodeBlock::atom("\"Alex\""));

        assert_eq!(
            "public final const val name: kotlin.String = \"Alex\"",
            property.render_string()
        )
    }

    #[test]
    fn test_delegate() {
        let property = Property::new(Name::from("name"), Type::string())
            .constant(true)
            .delegate(CodeBlock::atom("lazy { \"Alex\" }"));

        assert_eq!(
            "public final const val name: kotlin.String by lazy { \"Alex\" }",
            property.render_string()
        )
    }

    #[test]
    fn test_override() {
        let property = Property::new(Name::from("age"), Type::int())
            .overrides(true)
            .initializer(CodeBlock::atom("22"));

        assert_eq!(
            "public final override val age: kotlin.Int = 22",
            property.render_string()
        )
    }

    #[test]
    fn test_kdoc() {
        let property = Property::new(Name::from("age"), Type::int())
            .kdoc(KDoc::from("Hello\nWorld"));

        assert_eq!(
            "/**\n * Hello\n * World\n */\npublic final val age: kotlin.Int",
            property.render_string()
        )
    }

    #[test]
    fn test_annotation() {
        let property = Property::new(Name::from("age"), Type::int())
            .overrides(true)
            .initializer(CodeBlock::atom("22"))
            .annotation(Annotation::new(
                ClassLikeTypeName::from_str("io.github.lexadiky.MyAnnotation")
                    .unwrap()
            ))
            .annotation(Annotation::new(
                ClassLikeTypeName::from_str("io.github.lexadiky.OtherAnnotation")
                    .unwrap()
            ));

        assert_eq!(
            "@io.github.lexadiky.MyAnnotation()\n@io.github.lexadiky.OtherAnnotation()\npublic final override val age: kotlin.Int = 22",
            property.render_string()
        )
    }

    #[test]
    fn test_setter_with_annotation() {
        let setter = PropertySetter::new(CodeBlock::statement("println(47)"))
            .annotation(Annotation::new(
                ClassLikeTypeName::from_str("a.A").unwrap()
            ));

        assert_eq!(
            "@a.A()\nset(value) {\n    println(47)\n}",
            setter.render_string()
        )
    }

    #[test]
    fn test_getter_with_annotation() {
        let setter = PropertyGetter::new(CodeBlock::statement("println(47)"))
            .annotation(Annotation::new(
                ClassLikeTypeName::from_str("a.A").unwrap()
            ));

        assert_eq!(
            "@a.A()\nget() {\n    println(47)\n}",
            setter.render_string()
        )
    }
}