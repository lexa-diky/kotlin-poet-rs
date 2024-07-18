use crate::io::{RenderContext, RenderKotlin};
use crate::spec::{CodeBlock, Type};

#[derive(PartialEq, Debug, Clone)]
pub struct LambdaType {
    receiver: Box<Option<Type>>,
    parameters: Box<Vec<Type>>,
    returns: Box<Type>,
}

impl LambdaType {

    pub fn new(returns: Type) -> Self {
        LambdaType {
            receiver: Box::new(None),
            parameters: Box::new(Vec::new()),
            returns: Box::new(returns),
        }
    }

    pub fn receiver(mut self, receiver: Type) -> Self {
        self.receiver = Box::new(Some(receiver));
        self
    }

    pub fn parameter(mut self, parameters: Type) -> Self {
        self.parameters.push(parameters);
        self
    }
}

impl RenderKotlin for LambdaType {
    fn render(&self, context: RenderContext) -> CodeBlock {
        let mut lambda = CodeBlock::empty();
        if let Some(receiver) = &*self.receiver {
            lambda.with_nested(receiver.render(context));
            lambda.with_atom(".")
        }
        lambda.with_atom("(");
        for (idx, parameter) in self.parameters.iter().enumerate() {
            lambda.with_nested(parameter.render(context));
            if idx != self.parameters.len() - 1 {
                lambda.with_atom(", ");
            }
        }
        lambda.with_atom(") -> ");
        lambda.with_nested(self.returns.render(context));
        lambda
    }
}

#[cfg(test)]
mod test {

    use crate::io::RenderKotlin;
    use crate::spec::{LambdaType, Type};

    #[test]
    fn render_lambda_type() {
        let lambda_type = LambdaType::new(Type::int())
            .parameter(Type::string())
            .parameter(Type::boolean());
        assert_eq!(lambda_type.render_without_context(), "(kotlin.String, kotlin.Boolean) -> kotlin.Int");
    }

    #[test]
    fn render_lambda_type_with_receiver() {
        let lambda_type = LambdaType::new(Type::int())
            .receiver(Type::string())
            .parameter(Type::boolean());
        assert_eq!(lambda_type.render_without_context(), "kotlin.String.(kotlin.Boolean) -> kotlin.Int");
    }
}