use crate::io::{RenderContext, RenderKotlin};
use crate::io::tokens::INDENT;
use crate::spec::{AccessModifier, CodeBlock, MemberInheritanceModifier, Name, Type};

pub struct Property {
    name: Name,
    returns: Type,
    inheritance_modifier: MemberInheritanceModifier,
    access_modifier: AccessModifier,
    initializer: Option<CodeBlock>,
    getter: Option<PropertyGetter>,
    setter: Option<PropertySetter>,
    mutable: bool,
}

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
    fn render(&self, context: RenderContext) -> CodeBlock {
        let mut block = CodeBlock::from(context);
        block.with_statement("get() {");
        block.with_indent();
        block.with_nested(self.code.clone());
        block.with_unindent();
        block.with_statement("}");
        block
    }
}

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

    fn render(&self, context: RenderContext) -> CodeBlock {
        let mut code = CodeBlock::from(context);
        code.with_statement("set(value) {");
        code.with_indent();
        code.with_nested(self.code.clone());
        code.with_unindent();
        code.with_statement("}");
        code
    }
}

impl Property {
    pub fn new(name: Name, returns: Type) -> Property {
        Property {
            name,
            returns,
            inheritance_modifier: MemberInheritanceModifier::Default,
            access_modifier: AccessModifier::Public,
            initializer: None,
            getter: None,
            setter: None,
            mutable: false,
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
        self.mutable = true;
        self
    }

    pub fn mutable(mut self, flag: bool) -> Property {
        self.mutable = flag;
        self
    }
}

impl RenderKotlin for Property {
    fn render(&self, context: RenderContext) -> CodeBlock {
        let mut block = CodeBlock::empty();
        block.with_nested(self.access_modifier.render(context));
        block.with_space();
        block.with_nested(self.inheritance_modifier.render(context));
        block.with_space();

        if self.mutable {
            block.with_atom("var");
        } else {
            block.with_atom("val");
        }
        block.with_space();

        block.with_nested(self.name.render(context));
        block.with_atom(":");
        block.with_space();
        block.with_nested(self.returns.render(context));
        if let Some(initializer) = &self.initializer {
            block.with_space();
            block.with_atom("=");
            block.with_space();
            block.with_nested(initializer.clone())
        }
        if let Some(setter) = &self.setter {
            block.with_nested(setter.render(context.indent()));
        }
        if let Some(getter) = &self.getter {
            block.with_nested(getter.render(context.indent()));
        }
        block
    }
}
