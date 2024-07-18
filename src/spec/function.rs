use crate::io::{RenderContext, RenderKotlin};
use crate::spec::{AccessModifier, CodeBlock, MemberInheritanceModifier, Name, Type};

#[derive(Debug, Clone)]
pub struct Function {
    name: Name,
    access_modifier: AccessModifier,
    parameters: Vec<(Name, Type)>,
    body: Option<CodeBlock>,
    returns: Type,
    receiver: Option<Type>,
    inheritance_modifier: MemberInheritanceModifier,
}

impl Function {
    pub fn new(name: Name) -> Function {
        Function {
            name,
            access_modifier: AccessModifier::Public,
            parameters: Vec::new(),
            body: None,
            returns: Type::unit(),
            receiver: None,
            inheritance_modifier: MemberInheritanceModifier::Default,
        }
    }

    pub fn access_modifier(mut self, access_modifier: AccessModifier) -> Function {
        self.access_modifier = access_modifier;
        self
    }

    pub fn parameter(mut self, name: Name, parameter: Type) -> Function {
        self.parameters.push((name, parameter));
        return self;
    }

    pub fn body(mut self, body: CodeBlock) -> Function {
        self.body = Some(body);
        return self;
    }

    pub fn returns(mut self, returns: Type) -> Function {
        self.returns = returns;
        return self;
    }

    pub fn receiver(mut self, receiver: Type) -> Function {
        self.receiver = Some(receiver);
        return self;
    }

    pub fn inheritance_modifier(mut self, inheritance_modifier: MemberInheritanceModifier) -> Function {
        self.inheritance_modifier = inheritance_modifier;
        self
    }
}

impl RenderKotlin for (Name, Type) {
    fn render(&self, context: RenderContext) -> CodeBlock {
        return CodeBlock::atom(
            format!("{}: {}", self.0.render_string(context), self.1.render_string(context)).as_str()
        );
    }
}

impl RenderKotlin for Function {
    fn render(&self, context: RenderContext) -> CodeBlock {
        let mut block = CodeBlock::empty();

        if !matches!(self.inheritance_modifier, MemberInheritanceModifier::Default) {
            block.with_nested(self.inheritance_modifier.render(context));
            block.with_space();
        }
        block.with_nested(self.access_modifier.render(context));
        block.with_space();

        block.with_atom("fun");
        block.with_space();

        if let Some(receiver) = &self.receiver {
            block.with_nested(receiver.render(context));
            block.with_atom(".");
        }
        block.with_atom(self.name.render_string(context).as_str());
        block.with_atom("(");

        let total_parameters = self.parameters.len();
        for (index, parameter) in self.parameters.iter().enumerate() {
            block.with_nested(parameter.render(context));
            if index != total_parameters - 1 {
                block.with_atom(", ");
            }
        }
        block.with_atom(")");

        block.with_atom(": ");
        block.with_nested(self.returns.render(context));

        if let Some(body) = &self.body {
            block.with_space();
            block.with_statement("{");
            block.with_indent();
            block.with_nested(body.clone());
            block.with_unindent();
            block.with_atom("}");
        }

        block
    }
}