use std::sync::mpsc::Receiver;
use std::task::Context;
use crate::io::{RenderContext, RenderKotlin};
use crate::io::tokens::NOTHING;
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
    fn render(&self, context: RenderContext) -> String {
        format!("{}: {}", self.0.render(context), self.1.render(context))
    }
}

impl RenderKotlin for Function {
    fn render(&self, context: RenderContext) -> String {
        fn render_parameters(params: &Vec<(Name, Type)>, context: RenderContext) -> String {
            let mut buf = String::new();
            for parameter in params {
                buf.push_str(parameter.render(context).as_str())
            }

            buf
        }

        let access_modifier = self.access_modifier.render(context);
        let content = if let Some(body) = &self.body {
            body.clone().wrap_in_scope().render(context)
        } else {
            NOTHING.to_string()
        };
        let returns = self.returns.render(context);
        let receiver = if let Some(receiver) = &self.receiver {
            format!("{}.", receiver.render(context))
        } else {
            "".to_string()
        };
        let inheritance = self.inheritance_modifier.render(context);

        format!(
            "{inheritance} {access_modifier} fun {receiver}{}({}): {returns} {content}",
            self.name.render(context),
            render_parameters(&self.parameters, context),
        )
    }
}