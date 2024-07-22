use crate::io::{RenderKotlin, tokens};
use crate::spec::{AccessModifier, ClassInheritanceModifier, CodeBlock, Function, Name, Property};

#[derive(Debug, Clone)]
enum ClassMemberNode {
    Property(Property),
    Function(Function),
    Subclass(Class)
}

#[derive(Debug, Clone)]
pub struct Class {
    name: Name,
    access_modifier: AccessModifier,
    inheritance_modifier: ClassInheritanceModifier,
    member_nodes: Vec<ClassMemberNode>
}

impl Class {

    pub fn new(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Final,
            member_nodes: Vec::new()
        }
    }

    pub fn new_interface(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Interface,
            member_nodes: Vec::new()
        }
    }

    pub fn new_abstract(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Abstract,
            member_nodes: Vec::new()
        }
    }

    pub fn new_object(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Object,
            member_nodes: Vec::new()
        }
    }

    pub fn new_sealed(name: Name) -> Self {
        Class {
            name,
            access_modifier: AccessModifier::Public,
            inheritance_modifier: ClassInheritanceModifier::Sealed,
            member_nodes: Vec::new()
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
            code.with_atom(tokens::KW_CLASS);
            code.with_space();
        }
        code.with_nested(self.name.render());
        code.with_space();
        code.with_atom(tokens::CURLY_BRACE_LEFT);
        code.with_new_line();
        code.with_indent();
        code.with_new_line();

        for node in &self.member_nodes {
            match node {
                ClassMemberNode::Property(property) => {
                    code.with_nested(property.render());
                    code.with_new_line();
                },
                ClassMemberNode::Function(function) => {
                    code.with_nested(function.render());
                    code.with_new_line();
                },
                ClassMemberNode::Subclass(subclass) => {
                    code.with_nested(subclass.render());
                    code.with_new_line();
                }
            }
        }

        code.with_unindent();
        code.with_atom(tokens::CURLY_BRACE_RIGHT);

        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class() {
        let class = Class::new(Name::from("Person"));
        let code = class.render();

        assert_eq!(code.to_string(), "public final class Person {\n\n}");
    }
}