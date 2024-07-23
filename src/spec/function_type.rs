use crate::io::RenderKotlin;
use crate::spec::{CodeBlock, Type};
use crate::tokens;

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionType {
    receiver: Box<Option<Type>>,
    parameters: Vec<Type>,
    returns: Box<Type>,
    is_suspended: bool
}

impl FunctionType {

    pub fn new(returns: Type) -> Self {
        FunctionType {
            receiver: Box::new(None),
            parameters: Vec::new(),
            returns: Box::new(returns),
            is_suspended: false
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

    pub fn returns(mut self, returns: Type) -> Self {
        *self.returns = returns;
        self
    }

    pub fn suspended(mut self, flag: bool) -> Self {
        self.is_suspended = flag;
        self
    }
}

impl RenderKotlin for FunctionType {
    fn render(&self) -> CodeBlock {
        let mut lambda = CodeBlock::empty();
        if let Some(receiver) = &*self.receiver {
            lambda.with_nested(receiver.render());
            lambda.with_atom(tokens::DOT)
        }

        if self.is_suspended {
            lambda.with_atom(tokens::keyword::SUSPEND);
            lambda.with_space()
        }

        lambda.with_round_brackets(|parameters_code| {
            parameters_code.with_comma_separated(
                &self.parameters
            );
        });

        lambda.with_space();
        lambda.with_atom(tokens::ARROW);
        lambda.with_space();
        lambda.with_nested(self.returns.render());
        lambda
    }
}

#[cfg(test)]
mod test {

    use crate::io::RenderKotlin;
    use crate::spec::{FunctionType, Type};

    #[test]
    fn render_lambda_type() {
        let lambda_type = FunctionType::new(Type::int())
            .parameter(Type::string())
            .parameter(Type::boolean());
        assert_eq!(lambda_type.render_string(), "(kotlin.String, kotlin.Boolean) -> kotlin.Int");
    }

    #[test]
    fn render_lambda_type_with_receiver() {
        let lambda_type = FunctionType::new(Type::int())
            .receiver(Type::string())
            .parameter(Type::boolean());
        assert_eq!(lambda_type.render_string(), "kotlin.String.(kotlin.Boolean) -> kotlin.Int");
    }

    #[test]
    fn render_lambda_type_with_suspended() {
        let lambda_type = FunctionType::new(Type::int())
            .parameter(Type::string())
            .parameter(Type::boolean())
            .suspended(true);
        assert_eq!(lambda_type.render_string(), "suspend (kotlin.String, kotlin.Boolean) -> kotlin.Int");
    }
}