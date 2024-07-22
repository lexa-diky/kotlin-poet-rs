use crate::io::{RenderKotlin, tokens};
use crate::spec::{AccessModifier, Argument, ClassInheritanceModifier, CodeBlock, Function, Name, Property};

#[derive(Debug, Clone)]
enum ClassMemberNode {
    Property(Property),
    Function(Function),
    Subclass(Class),
}

#[derive(Debug, Clone)]
struct EnumInstance {
    name: Name,
    arguments: Vec<Argument>
}

#[derive(Debug, Clone)]
pub struct Class {
    name: Name,
    access_modifier: AccessModifier,
    inheritance_modifier: ClassInheritanceModifier,
    member_nodes: Vec<ClassMemberNode>,
    enum_instances: Vec<EnumInstance>
}

impl Class {

    pub fn new(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Final,
            member_nodes: Vec::new(),
            enum_instances: Vec::new()
        }
    }

    pub fn new_interface(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Interface,
            member_nodes: Vec::new(),
            enum_instances: Vec::new()
        }
    }

    pub fn new_abstract(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Abstract,
            member_nodes: Vec::new(),
            enum_instances: Vec::new()
        }
    }

    pub fn new_object(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Object,
            member_nodes: Vec::new(),
            enum_instances: Vec::new()
        }
    }

    pub fn new_sealed(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Sealed,
            member_nodes: Vec::new(),
            enum_instances: Vec::new()
        }
    }

    pub fn new_enum(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Enum,
            member_nodes: Vec::new(),
            enum_instances: Vec::new()
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
            arguments
        });
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
        code.with_space();

        code.with_scope(|class_body_code| {
            class_body_code.with_new_line();

            if !self.enum_instances.is_empty() {
                for (inst_idx, instance) in self.enum_instances.iter().enumerate() {
                    class_body_code.with_nested(instance.name.render());
                    class_body_code.with_round_brackets(|arg_code| {
                        for (index, argument) in instance.arguments.iter().enumerate() {
                            arg_code.with_nested(argument.render());
                            if index != instance.arguments.len() - 1 {
                                arg_code.with_atom(tokens::COMMA);
                                arg_code.with_space();
                            }
                        }
                    });

                    if inst_idx != self.enum_instances.len() - 1 {
                        class_body_code.with_atom(tokens::COMMA);
                        class_body_code.with_new_line();
                    }
                }

                class_body_code.with_atom(tokens::SEMICOLON);
            }

            for node in &self.member_nodes {
                match node {
                    ClassMemberNode::Property(property) => {
                        class_body_code.with_nested(property.render());
                        class_body_code.with_new_line();
                    },
                    ClassMemberNode::Function(function) => {
                        class_body_code.with_nested(function.render());
                        class_body_code.with_new_line();
                    },
                    ClassMemberNode::Subclass(subclass) => {
                        class_body_code.with_nested(subclass.render());
                        class_body_code.with_new_line();
                    }
                }
            }
        });

        code
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::{PropertyGetter, PropertySetter, Type};
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
}