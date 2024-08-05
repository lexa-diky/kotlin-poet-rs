use crate::io::RenderKotlin;
use crate::spec::{VisibilityModifier, CodeBlock, MemberInheritanceModifier, Name, Type, Annotation, KDoc};
use crate::tokens;

#[derive(Debug, Clone)]
enum PropertyInitializer {
    Value(CodeBlock),
    Delegate(CodeBlock),
}

impl RenderKotlin for PropertyInitializer {

    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
        match self {
            PropertyInitializer::Value(initializer) => {
                block.with_space();
                block.with_atom(tokens::ASSIGN);
                block.with_space();
                block.with_nested(initializer.clone())
            }
            PropertyInitializer::Delegate(delegate) => {
                block.with_space();
                block.with_atom(tokens::keyword::BY);
                block.with_space();
                block.with_nested(delegate.clone())
            }
        }

        block
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
    kdoc: Option<KDoc>
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

    pub fn annotation(mut self, annotation: Annotation) -> PropertyGetter {
        self.annotations.push(annotation);
        self
    }
}

impl RenderKotlin for PropertyGetter {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
        for annotation in &self.annotations {
            block.with_nested(annotation.render());
            block.with_new_line();
        }
        block.with_atom(tokens::keyword::GET);
        block.with_round_brackets(|_| {});
        block.with_space();
        block.with_atom(tokens::CURLY_BRACKET_LEFT);
        block.with_new_line();
        block.with_indent();
        block.with_nested(self.code.clone());
        block.with_unindent();
        block.with_statement(tokens::CURLY_BRACKET_RIGHT);
        block
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
            visibility_modifier: VisibilityModifier::Public,
            annotations: Vec::new()
        }
    }

    pub fn visibility_modifier(mut self, visibility_modifier: VisibilityModifier) -> PropertySetter {
        self.visibility_modifier = visibility_modifier;
        self
    }

    pub fn annotation(mut self, annotation: Annotation) -> PropertySetter {
        self.annotations.push(annotation);
        self
    }
}

impl RenderKotlin for PropertySetter {

    fn render(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();
        for annotation in &self.annotations {
            code.with_nested(annotation.render());
            code.with_new_line();
        }
        code.with_atom(tokens::keyword::SET);
        code.with_round_brackets(|parameters_code| {
            parameters_code.with_atom(tokens::CONV_VAR_VALUE);
        });
        code.with_space();
        code.with_curly_brackets(|set_body| {
            set_body.with_nested(self.code.clone());
        });
        code.with_new_line();
        code
    }
}

impl Property {
    pub fn new(name: Name, returns: Type) -> Property {
        Property {
            name,
            returns,
            inheritance_modifier: MemberInheritanceModifier::Final,
            visibility_modifier: VisibilityModifier::Public,
            initializer: None,
            getter: None,
            setter: None,
            is_mutable: false,
            is_const: false,
            is_override: false,
            annotations: Vec::new(),
            kdoc: None
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

    /// Adds an annotation
    pub fn annotation(mut self, annotation: Annotation) -> Property {
        self.annotations.push(annotation);
        self
    }

    /// Adds [KDoc] to this class.
    /// In case of multiple calls, KDocs will be merged, see [KDoc::merge].
    pub fn kdoc(mut self, kdoc: KDoc) -> Self {
        self.kdoc = match self.kdoc {
            None => { Some(kdoc) }
            Some(old) => { Some(old.merge(kdoc)) }
        };
        self
    }
}

impl RenderKotlin for Property {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();

        if let Some(kdoc) = &self.kdoc {
            block.with_nested(kdoc.render());
            block.with_new_line();
        }

        for annotation in &self.annotations {
            block.with_nested(annotation.render());
            block.with_new_line();
        }

        block.with_nested(self.visibility_modifier.render());
        block.with_space();
        block.with_nested(self.inheritance_modifier.render());
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

        block.with_nested(self.name.render());
        block.with_atom(tokens::COLON);
        block.with_space();
        block.with_nested(self.returns.render());
        block.with_indent();
        if let Some(initializer) = &self.initializer {
            block.with_nested(initializer.render());
        }
        if let Some(setter) = &self.setter {
            block.with_nested(setter.render());
        }
        if let Some(getter) = &self.getter {
            block.with_nested(getter.render());
        }
        block.with_unindent();
        block
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

        let rendered = property.render().to_string();
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
            property.render().to_string()
        )
    }

    #[test]
    fn test_delegate() {
        let property = Property::new(Name::from("name"), Type::string())
            .constant(true)
            .delegate(CodeBlock::atom("lazy { \"Alex\" }"));

        assert_eq!(
            "public final const val name: kotlin.String by lazy { \"Alex\" }",
            property.render().to_string()
        )
    }

    #[test]
    fn test_override() {
        let property = Property::new(Name::from("age"), Type::int())
            .overrides(true)
            .initializer(CodeBlock::atom("22"));

        assert_eq!(
            "public final override val age: kotlin.Int = 22",
            property.render().to_string()
        )
    }

    #[test]
    fn test_kdoc() {
        let property = Property::new(Name::from("age"), Type::int())
            .kdoc(KDoc::from("Hello\nWorld"));

        assert_eq!(
            "/**\n * Hello\n * World\n */\npublic final val age: kotlin.Int",
            property.render().to_string()
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
            property.render().to_string()
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