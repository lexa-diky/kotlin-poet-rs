use crate::io::RenderKotlin;
use crate::spec::{AccessModifier, CodeBlock, MemberInheritanceModifier, Name, Type};
use crate::tokens;

#[derive(Debug, Clone)]
pub struct Property {
    name: Name,
    returns: Type,
    inheritance_modifier: MemberInheritanceModifier,
    access_modifier: AccessModifier,
    initializer: Option<CodeBlock>,
    getter: Option<PropertyGetter>,
    setter: Option<PropertySetter>,
    is_mutable: bool,
    is_const: bool
}

#[derive(Debug, Clone)]
pub struct PropertyGetter {
    code: CodeBlock,
}

impl PropertyGetter {
    pub fn new(code: CodeBlock) -> PropertyGetter {
        PropertyGetter {
            code
        }
    }
}

impl RenderKotlin for PropertyGetter {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
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
    access_modifier: AccessModifier,
}

impl PropertySetter {
    pub fn new(code: CodeBlock) -> PropertySetter {
        PropertySetter {
            code,
            access_modifier: AccessModifier::Public,
        }
    }

    pub fn access_modifier(mut self, access_modifier: AccessModifier) -> PropertySetter {
        self.access_modifier = access_modifier;
        self
    }
}

impl RenderKotlin for PropertySetter {

    fn render(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();
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
            access_modifier: AccessModifier::Public,
            initializer: None,
            getter: None,
            setter: None,
            is_mutable: false,
            is_const: false
        }
    }

    pub fn access_modifier(mut self, access_modifier: AccessModifier) -> Property {
        self.access_modifier = access_modifier;
        self
    }

    pub fn inheritance_modifier(mut self, inheritance_modifier: MemberInheritanceModifier) -> Property {
        self.inheritance_modifier = inheritance_modifier;
        self
    }

    pub fn initializer(mut self, initializer: CodeBlock) -> Property {
        self.initializer = Some(initializer);
        self
    }

    pub fn getter(mut self, getter: PropertyGetter) -> Property {
        self.getter = Some(getter);
        self
    }

    pub fn setter(mut self, setter: PropertySetter) -> Property {
        self.setter = Some(setter);
        self.is_mutable = true;
        self
    }

    pub fn mutable(mut self, flag: bool) -> Property {
        self.is_mutable = flag;
        self
    }

    pub fn constant(mut self, flag: bool) -> Property {
        self.is_const = flag;
        self
    }
}

impl RenderKotlin for Property {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
        block.with_nested(self.access_modifier.render());
        block.with_space();
        block.with_nested(self.inheritance_modifier.render());
        block.with_space();

        if self.is_const {
            block.with_atom(tokens::keyword::CONST);
            block.with_space()
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
            block.with_space();
            block.with_atom(tokens::EQUALS);
            block.with_space();
            block.with_nested(initializer.clone())
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
        let expected = "public final var name: kotlin.String = \"\"\n    set(value) {\n        field = value\n    }\n    get() {\n        return field\n    }\n";
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
}