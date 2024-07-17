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
    fn render(&self, context: RenderContext) -> String {
        format!("get() {{\n{}{}}}\n", INDENT, self.code.render(context))
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
    fn render(&self, context: RenderContext) -> String {
        format!("set(value) {{\n{}{}}}\n", INDENT, self.code.render(context))
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
    fn render(&self, context: RenderContext) -> String {
        let mut result = String::new();
        result.push_str(self.access_modifier.render(context).as_str());
        result.push_str(" ");

        result.push_str(self.inheritance_modifier.render(context).as_str());
        if !matches!(self.inheritance_modifier, MemberInheritanceModifier::Default) {
            result.push_str(" ")
        }

        match self.mutable {
            true => result.push_str("var "),
            false => result.push_str("val ")
        }

        result.push_str(self.name.render(context).as_str());

        result.push_str(": ");
        result.push_str(self.returns.render(context).as_str());

        if let Some(initializer) = &self.initializer {
            result.push_str(" = ");
            result.push_str(initializer.render(context).as_str());
        }

        if let Some(getter) = &self.getter {
            result.push_str(getter.render(context.indent()).as_str());
        }

        if let Some(setter) = &self.setter {
            result.push_str(setter.render(context.indent()).as_str());
        }

        result
    }
}
