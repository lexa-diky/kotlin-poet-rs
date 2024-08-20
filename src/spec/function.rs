use crate::io::RenderKotlin;
use crate::spec::{VisibilityModifier, CodeBlock, GenericParameter, MemberInheritanceModifier, Name, Type, Parameter, Annotation};
use crate::spec::annotation::{mixin_annotation_mutators, AnnotationSlot};
use crate::spec::kdoc::{KdocSlot, mixin_kdoc_mutators};
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
    annotation_slot: AnnotationSlot,
    kdoc: KdocSlot,
}

impl Function {
    pub fn new<NameLike: Into<Name>>(name: NameLike) -> Function {
        Function {
            name: name.into(),
            visibility_modifier: VisibilityModifier::default(),
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
            annotation_slot: AnnotationSlot::vertical(),
            kdoc: KdocSlot::default(),
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

    pub fn body<CodeBlockLike: Into<CodeBlock>>(mut self, body: CodeBlockLike) -> Function {
        self.body = Some(body.into());
        self
    }

    pub fn returns<TypeLike: Into<Type>>(mut self, returns: TypeLike) -> Function {
        self.returns = returns.into();
        self
    }

    pub fn receiver<TypeLike: Into<Type>>(mut self, receiver: TypeLike) -> Function {
        self.receiver = Some(receiver.into());
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

    mixin_annotation_mutators!();
    mixin_kdoc_mutators!();
}

impl RenderKotlin for Function {
    fn render_into(&self, block: &mut CodeBlock) {
        block.push_renderable(&self.kdoc);
        block.push_renderable(&self.annotation_slot);

        block.push_renderable(&self.visibility_modifier);
        block.push_space();

        if self.is_suspended {
            block.push_static_atom(tokens::keyword::SUSPEND);
            block.push_space();
        }

        if self.is_inline {
            block.push_static_atom(tokens::keyword::INLINE);
            block.push_space();
        }

        if self.is_operator {
            block.push_static_atom(tokens::keyword::OPERATOR);
            block.push_space();
        }

        if self.is_override {
            block.push_static_atom(tokens::keyword::OVERRIDE);
            block.push_space();
        }

        block.push_static_atom(tokens::keyword::FUN);
        block.push_space();

        if !self.generic_parameters.is_empty() {
            block.push_angle_brackets(|code| {
                code.push_comma_separated(
                    &self.generic_parameters.iter().map(|it| it.render_definition())
                        .collect::<Vec<CodeBlock>>()
                );
            });
            block.push_space();
        }

        if let Some(receiver) = &self.receiver {
            block.push_renderable(receiver);
            block.push_static_atom(tokens::DOT);
        }
        block.push_renderable(&self.name);

        block.push_round_brackets(|parameters_code| {
            let total_parameters = self.parameters.len();
            for (index, parameter) in self.parameters.iter().enumerate() {
                parameters_code.push_renderable(parameter);
                if index != total_parameters - 1 {
                    parameters_code.push_static_atom(tokens::COMMA);
                    parameters_code.push_space()
                }
            }
        });

        block.push_static_atom(tokens::COLON);
        block.push_space();
        block.push_renderable(&self.returns);

        block.push_space();
        block.push_renderable(
            &GenericParameter::render_type_boundaries_vec_if_required(
                &self.generic_parameters
            )
        );

        if let Some(body) = &self.body {
            block.push_space();
            block.push_curly_brackets(|inner| {
                inner.push_renderable(body);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::{Annotation, ClassLikeTypeName, CodeBlock, Function, GenericParameter, KDoc, Name, Package, Type, VisibilityModifier};
    use crate::spec::function::Parameter;

    #[test]
    fn test_function_with_multiple_parameters() {
        let block = Function::new(Name::from("main"))
            .receiver(Type::short())
            .visibility_modifier(VisibilityModifier::default())
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
            .visibility_modifier(VisibilityModifier::Public)
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