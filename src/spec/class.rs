use crate::io::RenderKotlin;
use crate::spec::{AccessModifier, Argument, ClassInheritanceModifier, CodeBlock, CompanionObject, Function, GenericParameter, Name, PrimaryConstructor, Property, SecondaryConstructor};
use crate::tokens;

#[derive(Debug, Clone)]
pub(crate) enum ClassMemberNode {
    Property(Property),
    Function(Function),
    Subclass(Class),
    SecondaryConstructor(SecondaryConstructor),
    InitBlock(CodeBlock),
}

impl RenderKotlin for ClassMemberNode {
    fn render(&self) -> CodeBlock {
        let mut class_body_code = CodeBlock::empty();
        match self {
            ClassMemberNode::Property(property) => {
                class_body_code.with_nested(property.render());
            }
            ClassMemberNode::Function(function) => {
                class_body_code.with_nested(function.render());
            }
            ClassMemberNode::Subclass(subclass) => {
                class_body_code.with_nested(subclass.render());
            }
            ClassMemberNode::SecondaryConstructor(secondary_constructor) => {
                class_body_code.with_nested(secondary_constructor.render());
            }
            ClassMemberNode::InitBlock(code) => {
                class_body_code.with_atom(tokens::keyword::INIT);
                class_body_code.with_curly_brackets(|block| {
                    block.with_nested(code.clone());
                });
            }
        }
        class_body_code
    }
}

#[derive(Debug, Clone)]
struct EnumInstance {
    name: Name,
    arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Class {
    name: Name,
    access_modifier: AccessModifier,
    inheritance_modifier: ClassInheritanceModifier,
    member_nodes: Vec<ClassMemberNode>,
    enum_instances: Vec<EnumInstance>,
    primary_constructor: Option<PrimaryConstructor>,
    companion_object: Option<CompanionObject>,
    generic_parameters: Vec<GenericParameter>,
}

impl Class {
    pub fn new(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Final,
            member_nodes: Vec::new(),
            enum_instances: Vec::new(),
            primary_constructor: None,
            companion_object: None,
            generic_parameters: Vec::new(),
        }
    }

    pub fn new_interface(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Interface,
            member_nodes: Vec::new(),
            enum_instances: Vec::new(),
            primary_constructor: None,
            companion_object: None,
            generic_parameters: Vec::new(),
        }
    }

    pub fn new_abstract(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Abstract,
            member_nodes: Vec::new(),
            enum_instances: Vec::new(),
            primary_constructor: None,
            companion_object: None,
            generic_parameters: Vec::new(),
        }
    }

    pub fn new_object(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Object,
            member_nodes: Vec::new(),
            enum_instances: Vec::new(),
            primary_constructor: None,
            companion_object: None,
            generic_parameters: Vec::new(),
        }
    }

    pub fn new_sealed(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Sealed,
            member_nodes: Vec::new(),
            enum_instances: Vec::new(),
            primary_constructor: None,
            companion_object: None,
            generic_parameters: Vec::new(),
        }
    }

    pub fn new_enum(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Enum,
            member_nodes: Vec::new(),
            enum_instances: Vec::new(),
            primary_constructor: None,
            companion_object: None,
            generic_parameters: Vec::new(),
        }
    }

    pub fn new_data_class(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Data,
            member_nodes: Vec::new(),
            enum_instances: Vec::new(),
            primary_constructor: None,
            companion_object: None,
            generic_parameters: Vec::new(),
        }
    }

    pub fn access_modifier(mut self, access_modifier: AccessModifier) -> Self {
        self.access_modifier = access_modifier;
        self
    }

    pub fn inheritance_modifier(mut self, inheritance_modifier: ClassInheritanceModifier) -> Self {
        self.inheritance_modifier = inheritance_modifier;
        self
    }

    pub fn property(mut self, property: Property) -> Self {
        self.member_nodes.push(ClassMemberNode::Property(property));
        self
    }

    pub fn function(mut self, function: Function) -> Self {
        self.member_nodes.push(ClassMemberNode::Function(function));
        self
    }

    pub fn subclass(mut self, subclass: Class) -> Self {
        self.member_nodes.push(ClassMemberNode::Subclass(subclass));
        self
    }

    pub fn enum_instance(mut self, name: Name, arguments: Vec<Argument>) -> Self {
        self.enum_instances.push(EnumInstance {
            name,
            arguments,
        });
        self
    }

    pub fn primary_constructor(mut self, primary_constructor: PrimaryConstructor) -> Self {
        self.primary_constructor = Some(primary_constructor);
        self
    }

    pub fn secondary_constructor(mut self, secondary_constructor: SecondaryConstructor) -> Self {
        self.member_nodes.push(ClassMemberNode::SecondaryConstructor(secondary_constructor));
        self
    }

    pub fn init(mut self, block: CodeBlock) -> Self {
        self.member_nodes.push(ClassMemberNode::InitBlock(block));
        self
    }

    pub fn companion_object(mut self, companion_object: CompanionObject) -> Self {
        self.companion_object = Some(companion_object);
        self
    }

    /// Adds [GenericParameter] to this class.
    /// Could be called multiple times to have multiple generic parameters.
    pub fn generic_parameter(mut self, generic_parameter: GenericParameter) -> Self {
        self.generic_parameters.push(generic_parameter);
        self
    }
}

impl RenderKotlin for Class {
    fn render(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();

        code.with_nested(self.access_modifier.render());
        code.with_space();
        code.with_nested(self.inheritance_modifier.render());
        code.with_space();
        if !matches!(
            self.inheritance_modifier,
            ClassInheritanceModifier::Interface |
            ClassInheritanceModifier::Object
        ) {
            code.with_atom(tokens::keyword::CLASS);
            code.with_space();
        }
        code.with_nested(self.name.render());
        if !self.generic_parameters.is_empty() {
            code.with_angle_brackets(|code| {
                code.with_comma_separated(
                    &self.generic_parameters.iter().map(|it| it.render_definition())
                        .collect::<Vec<CodeBlock>>()
                );
            });
        }
        code.with_space();

        if let Some(primary_constructor) = &self.primary_constructor {
            code.with_nested(primary_constructor.render());
            code.with_space();
        }

        code.with_nested(
            GenericParameter::render_type_boundaries_vec_if_required(
                &self.generic_parameters
            )
        );

        code.with_curly_brackets(|class_body_code| {
            class_body_code.with_new_line();

            if !self.enum_instances.is_empty() {
                for (inst_idx, instance) in self.enum_instances.iter().enumerate() {
                    class_body_code.with_nested(instance.name.render());
                    class_body_code.with_round_brackets(|arg_code| {
                        arg_code.with_comma_separated(&instance.arguments);
                    });

                    if inst_idx != self.enum_instances.len() - 1 {
                        class_body_code.with_atom(tokens::COMMA);
                        class_body_code.with_new_line();
                    }
                }

                class_body_code.with_atom(tokens::SEMICOLON);
            }

            for node in &self.member_nodes {
                class_body_code.with_nested(node.render());
                class_body_code.with_new_line();
            }

            if let Some(companion_object) = &self.companion_object {
                class_body_code.with_nested(companion_object.render());
                class_body_code.with_new_line();
            }
        });

        code
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::{FunctionParameter, GenericInvariance, PropertyGetter, PropertySetter, Type};
    use super::*;

    #[test]
    fn test_class() {
        let class = Class::new(Name::from("Person"));
        let code = class.render();

        assert_eq!(code.to_string(), "public final class Person {\n\n}");
    }

    #[test]
    fn test_class_with_property() {
        let property = Property::new(
            Name::from("name"),
            Type::string(),
        ).initializer(
            CodeBlock::statement("\"\"")
        ).getter(
            PropertyGetter::new(
                CodeBlock::statement("return field")
            )
        ).setter(
            PropertySetter::new(
                CodeBlock::statement("field = value")
            )
        );

        let class = Class::new(Name::from("Person"))
            .property(property.clone());

        let code = class.render();

        assert_eq!(
            code.to_string(),
            "public final class Person {\n\n    public final var name: kotlin.String = \"\"\n        set(value) {\n            field = value\n        }\n        get() {\n            return field\n        }\n\n}"
        );
    }

    #[test]
    fn test_enum() {
        let class = Class::new_enum(Name::from("Person"))
            .enum_instance(Name::from("Alex"), vec![
                Argument::new(CodeBlock::atom("23"))
            ])
            .enum_instance(Name::from("Vova"), vec![
                Argument::new(CodeBlock::atom("23"))
            ])
            ;
        let code = class.render();

        assert_eq!(
            code.to_string(),
            "public enum class Person {\n\n    Alex(23),\n    Vova(23);}"
        );
    }

    #[test]
    fn test_with_constructor() {
        let class = Class::new(Name::from("Person"))
            .primary_constructor(
                PrimaryConstructor::new()
                    .property(
                        Property::new(
                            Name::from("name"),
                            Type::string(),
                        )
                    )
                    .parameter(
                        FunctionParameter::new(
                            Name::from("age"),
                            Type::int(),
                        )
                    )
            );

        assert_eq!(
            class.render().to_string(),
            "public final class Person public constructor(public final val name: kotlin.String, age: kotlin.Int) {\n\n}"
        );
    }

    #[test]
    fn test_with_empty_constructor() {
        let class = Class::new(Name::from("Person"))
            .primary_constructor(
                PrimaryConstructor::new()
            );

        assert_eq!(
            class.render().to_string(),
            "public final class Person public constructor() {\n\n}"
        );
    }

    #[test]
    fn test_with_init_block() {
        let class = Class::new(Name::from("Person"))
            .init(
                CodeBlock::statement("println(42)")
            );

        assert_eq!(
            class.render().to_string(),
            "public final class Person {\n\n    init{\n        println(42)\n    }\n}"
        );
    }

    #[test]
    fn test_data_class() {
        let class = Class::new_data_class(Name::from("Person"))
            .primary_constructor(
                PrimaryConstructor::new()
                    .property(
                        Property::new(
                            Name::from("name"),
                            Type::string(),
                        ).initializer(
                            CodeBlock::atom("\"\"")
                        )
                    )
            );

        assert_eq!(
            class.render().to_string(),
            "public data class Person public constructor(public final val name: kotlin.String = \"\") {\n\n}"
        );
    }

    #[test]
    fn test_data_class_with_secondary_constructor() {
        let class = Class::new_data_class(Name::from("Person"))
            .primary_constructor(
                PrimaryConstructor::new()
                    .property(
                        Property::new(
                            Name::from("name"),
                            Type::string(),
                        )
                    )
                    .property(
                        Property::new(
                            Name::from("age"),
                            Type::int(),
                        )
                    )
            )
            .secondary_constructor(
                SecondaryConstructor::new()
                    .parameter(
                        FunctionParameter::new(
                            Name::from("name"),
                            Type::string(),
                        )
                    )
                    .delegate_argument(
                        Argument::new(
                            CodeBlock::atom("name")
                        )
                    )
                    .delegate_argument(
                        Argument::new(
                            CodeBlock::atom("23")
                        )
                    )
                    .body(
                        CodeBlock::statement("println(42)")
                    )
            );

        assert_eq!(
            class.render().to_string(),
            "public data class Person public constructor(public final val name: kotlin.String, public final val age: kotlin.Int) {\n\n    public constructor(name: kotlin.String) : this(name, 23) {\n        println(42)\n    }\n}"
        );
    }

    #[test]
    fn test_interface() {
        let class = Class::new_interface(Name::from("Person"));
        let code = class.render();

        assert_eq!(code.to_string(), "public interface Person {\n\n}");
    }

    #[test]
    fn test_abstract() {
        let class = Class::new_abstract(Name::from("Person"));
        let code = class.render();

        assert_eq!(code.to_string(), "public abstract class Person {\n\n}");
    }

    #[test]
    fn test_object() {
        let class = Class::new_object(Name::from("Person"));
        let code = class.render();

        assert_eq!(code.to_string(), "public object Person {\n\n}");
    }

    #[test]
    fn test_sealed() {
        let class = Class::new_sealed(Name::from("Person"));
        let code = class.render();

        assert_eq!(code.to_string(), "public sealed class Person {\n\n}");
    }

    #[test]
    fn test_generic_class() {
        let class = Class::new(Name::from("Box"))
            .generic_parameter(
                GenericParameter::new(Name::from("A"))
            )
            .generic_parameter(
                GenericParameter::new(Name::from("B"))
                    .invariance(GenericInvariance::In)
            )
            .generic_parameter(
                GenericParameter::new(Name::from("C"))
                    .invariance(GenericInvariance::Out)
            );

        let code = class.render();

        assert_eq!(code.to_string(), "public final class Box<A, in B, out C> {\n\n}");
    }

    #[test]
    fn test_generic_class_with_boundaries() {
        let class = Class::new(Name::from("Box"))
            .generic_parameter(
                GenericParameter::new(Name::from("A"))
            )
            .generic_parameter(
                GenericParameter::new(Name::from("B"))
                    .invariance(GenericInvariance::In)
                    .type_boundary(Type::string())
                    .type_boundary(Type::int())
            )
            .generic_parameter(
                GenericParameter::new(Name::from("C"))
                    .invariance(GenericInvariance::Out)
            );

        let code = class.render();

        assert_eq!(code.to_string(), "public final class Box<A, in B, out C> where B: kotlin.String, B: kotlin.Int {\n\n}");
    }
}