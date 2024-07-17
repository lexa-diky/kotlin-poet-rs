use std::sync::mpsc::Receiver;
use crate::io::RenderKotlin;
use crate::spec::{AccessModifier, CodeBlock, Name, Type};

#[derive(Debug, Clone)]
pub struct Function {
    name: Name,
    access_modifier: AccessModifier,
    parameters: Vec<(Name, Type)>,
    body: Option<CodeBlock>,
    returns: Type,
    receiver: Option<Type>
}

impl Function {
    pub fn new(name: Name) -> Function {
        Function {
            name,
            access_modifier: AccessModifier::Public,
            parameters: Vec::new(),
            body: None,
            returns: Type::unit(),
            receiver: None
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
}

impl RenderKotlin for (Name, Type) {
    fn render(&self) -> String {
        format!("{}: {}", self.0.render(), self.1.render())
    }
}

impl RenderKotlin for Function {
    fn render(&self) -> String {
        fn render_parameters(params: &Vec<(Name, Type)>) -> String {
            let mut buf = String::new();
            for parameter in params {
                buf.push_str(parameter.render().as_str())
            }

            buf
        }

        let access_modifier = self.access_modifier.render();
        let content = if let Some(body) = &self.body {
            body.clone().wrap_in_scope().render()
        } else {
            "{}".to_string()
        };
        let returns = self.returns.render();
        let receiver = if let Some(receiver) = &self.receiver {
            format!("{}.", receiver.render())
        } else {
            "".to_string()
        };

        format!(
            "{access_modifier} fun {receiver}{}({}): {returns} {content}",
            self.name.render(),
            render_parameters(&self.parameters),
        )
    }
}