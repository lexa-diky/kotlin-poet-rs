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

    pub fn new<TypeLike: Into<Type>>(returns: TypeLike) -> Self {
        FunctionType {
            receiver: Box::new(None),
            parameters: Vec::new(),
            returns: Box::new(returns.into()),
            is_suspended: false
        }
    }

    pub fn receiver<TypeLike: Into<Type>>(mut self, receiver: TypeLike) -> Self {
        self.receiver = Box::new(Some(receiver.into()));
        self
    }

    pub fn parameter<TypeLike: Into<Type>>(mut self, parameters: TypeLike) -> Self {
        self.parameters.push(parameters.into());
        self
    }

    pub fn returns<TypeLike: Into<Type>>(mut self, returns: TypeLike) -> Self {
        *self.returns = returns.into();
        self
    }

    pub fn suspended(mut self, flag: bool) -> Self {
        self.is_suspended = flag;
        self
    }
}

impl RenderKotlin for FunctionType {
    fn render_into(&self, block: &mut CodeBlock) {
        if let Some(receiver) = &*self.receiver {
            block.push_renderable(receiver);
            block.push_atom(tokens::DOT)
        }

        if self.is_suspended {
            block.push_atom(tokens::keyword::SUSPEND);
            block.push_space()
        }

        block.push_round_brackets(|parameters_code| {
            parameters_code.push_comma_separated(
                &self.parameters
            );
        });

        block.push_space();
        block.push_atom(tokens::ARROW);
        block.push_space();
        block.push_renderable(self.returns.as_ref());
    }
}

#[cfg(test)]
mod tests {

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