use crate::io::RenderKotlin;
use crate::spec::{AccessModifier, CodeBlock, MemberInheritanceModifier, Name, Type};
use crate::tokens;

#[derive(Debug, Clone)]
pub struct FunctionParameter {
    name: Name,
    parameter_type: Type,
    default_value: Option<CodeBlock>,
}

impl RenderKotlin for FunctionParameter {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
        block.with_nested(self.name.render());
        block.with_atom(tokens::COLON);
        block.with_space();
        block.with_nested(self.parameter_type.render());
        if let Some(default_value) = &self.default_value {
            block.with_space();
            block.with_atom(tokens::ASSIGN);
            block.with_space();
            block.with_nested(default_value.clone());
        }

        block
    }
}

impl FunctionParameter {
    pub fn new(name: Name, parameter_type: Type) -> FunctionParameter {
        FunctionParameter {
            name,
            parameter_type,
            default_value: None,
        }
    }

    pub fn default_value(mut self, default_value: CodeBlock) -> FunctionParameter {
        self.default_value = Some(default_value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    name: Name,
    access_modifier: AccessModifier,
    parameters: Vec<FunctionParameter>,
    body: Option<CodeBlock>,
    returns: Type,
    receiver: Option<Type>,
    inheritance_modifier: MemberInheritanceModifier,
    is_suspended: bool,
    is_inline: bool,
    is_operator: bool,
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
            inheritance_modifier: MemberInheritanceModifier::Final,
            is_suspended: false,
            is_inline: false,
            is_operator: false,
        }
    }

    pub fn access_modifier(mut self, access_modifier: AccessModifier) -> Function {
        self.access_modifier = access_modifier;
        self
    }

    pub fn operator(mut self, flag: bool) -> Function {
        self.is_operator = flag;
        self
    }

    pub fn parameter(mut self, parameter: FunctionParameter) -> Function {
        self.parameters.push(parameter);
        self
    }

    pub fn body(mut self, body: CodeBlock) -> Function {
        self.body = Some(body);
        self
    }

    pub fn returns(mut self, returns: Type) -> Function {
        self.returns = returns;
        self
    }

    pub fn receiver(mut self, receiver: Type) -> Function {
        self.receiver = Some(receiver);
        self
    }

    pub fn inheritance_modifier(mut self, inheritance_modifier: MemberInheritanceModifier) -> Function {
        self.inheritance_modifier = inheritance_modifier;
        self
    }

    pub fn suspended(mut self, flag: bool) -> Function {
        self.is_suspended = flag;
        self
    }

    pub fn inline(mut self, flag: bool) -> Function {
        self.is_inline = flag;
        self
    }
}

impl RenderKotlin for Function {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();
        block.with_nested(self.access_modifier.render());
        block.with_space();

        if self.is_suspended {
            block.with_atom(tokens::keyword::SUSPEND);
            block.with_space();
        }

        if self.is_inline {
            block.with_atom(tokens::keyword::INLINE);
            block.with_space();
        }

        if self.is_operator {
            block.with_atom(tokens::keyword::OPERATOR);
            block.with_space();
        }

        block.with_atom(tokens::keyword::FUN);
        block.with_space();

        if let Some(receiver) = &self.receiver {
            block.with_nested(receiver.render());
            block.with_atom(tokens::DOT);
        }
        block.with_nested(self.name.render());

        block.with_round_brackets(|parameters_code| {
            let total_parameters = self.parameters.len();
            for (index, parameter) in self.parameters.iter().enumerate() {
                parameters_code.with_nested(parameter.render());
                if index != total_parameters - 1 {
                    parameters_code.with_atom(tokens::COMMA);
                    parameters_code.with_space()
                }
            }
        });

        block.with_atom(tokens::COLON);
        block.with_space();
        block.with_nested(self.returns.render());

        if let Some(body) = &self.body {
            block.with_space();
            block.with_curly_brackets(|inner| {
                inner.with_nested(body.clone());
            });
        }

        block
    }
}

#[cfg(test)]
mod test {
    use crate::io::RenderKotlin;
    use crate::spec::{CodeBlock, Function, Name, Type};
    use crate::spec::function::FunctionParameter;

    #[test]
    fn test_function_with_multiple_parameters() {
        let block = Function::new(Name::from("main"))
            .receiver(Type::short())
            .access_modifier(crate::spec::AccessModifier::Public)
            .parameter(FunctionParameter::new(Name::from("args"), Type::array(Type::string())))
            .parameter(FunctionParameter::new(Name::from("args2"), Type::array(Type::int())))
            .body(CodeBlock::statement("return 23"))
            .operator(true)
            .suspended(true)
            .inline(true);


        assert_eq!(
            "public suspend inline operator fun kotlin.Short.main(args: kotlin.Array<kotlin.String>, args2: kotlin.Array<kotlin.Int>): kotlin.Unit {\n    return 23\n}",
            block.render_string()
        )
    }

    #[test]
    fn test_function_with_parameter_default_value() {
        let block = Function::new(Name::from("main"))
            .receiver(Type::short())
            .access_modifier(crate::spec::AccessModifier::Public)
            .parameter(
                FunctionParameter::new(Name::from("args"), Type::array(Type::string()))
                    .default_value(CodeBlock::atom("\"hello world\""))
            )
            .body(CodeBlock::statement("return 23"))
            .operator(true)
            .suspended(true)
            .inline(true);


        assert_eq!(
            "public suspend inline operator fun kotlin.Short.main(args: kotlin.Array<kotlin.String> = \"hello world\"): kotlin.Unit {\n    return 23\n}",
            block.render_string()
        )
    }
}