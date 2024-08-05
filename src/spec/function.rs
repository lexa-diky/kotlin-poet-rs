use crate::io::RenderKotlin;
use crate::spec::{VisibilityModifier, CodeBlock, GenericParameter, MemberInheritanceModifier, Name, Type, Parameter, Annotation, KDoc};
use crate::tokens;

#[derive(Debug, Clone)]
pub struct Function {
    name: Name,
    visibility_modifier: VisibilityModifier,
    parameters: Vec<Parameter>,
    body: Option<CodeBlock>,
    returns: Type,
    receiver: Option<Type>,
    inheritance_modifier: MemberInheritanceModifier,
    is_suspended: bool,
    is_inline: bool,
    is_operator: bool,
    is_override: bool,
    generic_parameters: Vec<GenericParameter>,
    annotations: Vec<Annotation>,
    kdoc: Option<KDoc>
}

impl Function {
    pub fn new(name: Name) -> Function {
        Function {
            name,
            visibility_modifier: VisibilityModifier::Public,
            parameters: Vec::new(),
            body: None,
            returns: Type::unit(),
            receiver: None,
            inheritance_modifier: MemberInheritanceModifier::Final,
            is_suspended: false,
            is_inline: false,
            is_operator: false,
            generic_parameters: Vec::new(),
            is_override: false,
            annotations: Vec::new(),
            kdoc: None
        }
    }

    pub fn visibility_modifier(mut self, visibility_modifier: VisibilityModifier) -> Function {
        self.visibility_modifier = visibility_modifier;
        self
    }

    pub fn operator(mut self, flag: bool) -> Function {
        self.is_operator = flag;
        self
    }

    pub fn parameter(mut self, parameter: Parameter) -> Function {
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

    pub fn generic_parameter(mut self, parameter: GenericParameter) -> Function {
        self.generic_parameters.push(parameter);
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

    pub fn overrides(mut self, flag: bool) -> Function {
        self.is_override = flag;
        self
    }

    pub fn annotation(mut self, annotation: Annotation) -> Function {
        self.annotations.push(annotation);
        self
    }

    /// Adds [KDoc] to this class.
    /// In case of multiple calls, KDocs will be merged, see [KDoc::merge].
    pub fn kdoc(mut self, kdoc: KDoc) -> Self {
        self.kdoc = match self.kdoc {
            None => { Some(kdoc) }
            Some(old) => { Some(old.merge(kdoc)) }
        };
        self
    }
}

impl RenderKotlin for Function {
    fn render(&self) -> CodeBlock {
        let mut block = CodeBlock::empty();

        if let Some(kdoc) = &self.kdoc {
            block.with_nested(kdoc.render());
            block.with_new_line();
        }

        for annotation in &self.annotations {
            block.with_nested(annotation.render());
            block.with_new_line()
        }

        block.with_nested(self.visibility_modifier.render());
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

        if self.is_override {
            block.with_atom(tokens::keyword::OVERRIDE);
            block.with_space();
        }

        block.with_atom(tokens::keyword::FUN);
        block.with_space();

        if !self.generic_parameters.is_empty() {
            block.with_angle_brackets(|code| {
                code.with_comma_separated(
                    &self.generic_parameters.iter().map(|it| it.render_definition())
                        .collect::<Vec<CodeBlock>>()
                );
            });
            block.with_space();
        }

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

        block.with_space();
        block.with_nested(
            GenericParameter::render_type_boundaries_vec_if_required(
                &self.generic_parameters
            )
        );

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
    use crate::spec::{Annotation, ClassLikeTypeName, CodeBlock, Function, GenericParameter, KDoc, Name, Package, Type};
    use crate::spec::function::Parameter;

    #[test]
    fn test_function_with_multiple_parameters() {
        let block = Function::new(Name::from("main"))
            .receiver(Type::short())
            .visibility_modifier(crate::spec::VisibilityModifier::Public)
            .parameter(Parameter::new(Name::from("args"), Type::array(Type::string())))
            .parameter(Parameter::new(Name::from("args2"), Type::array(Type::int())))
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
            .visibility_modifier(crate::spec::VisibilityModifier::Public)
            .parameter(
                Parameter::new(Name::from("args"), Type::array(Type::string()))
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

    #[test]
    fn test_function_with_generic_arguments() {
        let block = Function::new(Name::from("box"))
            .generic_parameter(
                GenericParameter::new(Name::from("A"))
                    .type_boundary(Type::string())
                    .type_boundary(Type::int()),
            )
            .generic_parameter(
                GenericParameter::new(Name::from("B"))
            );


        assert_eq!(
            "public fun <A, B> box(): kotlin.Unit where A: kotlin.String, A: kotlin.Int",
            block.render_string()
        )
    }

    #[test]
    fn test_override() {
        let block = Function::new(Name::from("box"))
            .overrides(true)
            .returns(Type::int())
            .body(CodeBlock::statement("return 23"));

        assert_eq!(
            "public override fun box(): kotlin.Int {\n    return 23\n}",
            block.render_string()
        )
    }

    #[test]
    fn test_kdoc() {
        let block = Function::new(Name::from("box"))
            .kdoc(KDoc::from("Hello\nWorld"));

        assert_eq!(
            "/**\n * Hello\n * World\n */\npublic fun box(): kotlin.Unit",
            block.render_string()
        )
    }

    #[test]
    fn test_with_annotation() {
        let block = Function::new(Name::from("box"))
            .annotation(
                Annotation::new(
                    ClassLikeTypeName::top_level(
                        Package::root(),
                        Name::from("Test"),
                    )
                )
            )
            .annotation(
                Annotation::new(
                    ClassLikeTypeName::top_level(
                        Package::root(),
                        Name::from("Test2"),
                    )
                )
            );

        assert_eq!(
            "@Test()\n@Test2()\npublic fun box(): kotlin.Unit",
            block.render_string()
        )
    }
}