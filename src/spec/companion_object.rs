use crate::io::RenderKotlin;
use crate::spec::{VisibilityModifier, Class, CodeBlock, Function, Property};
use crate::tokens;

#[derive(Debug, Clone)]
pub struct CompanionObject {
    visibility_modifier: VisibilityModifier,
    member_nodes: Vec<crate::spec::class::ClassMemberNode>,
}

impl CompanionObject {
    pub fn new() -> Self {
        CompanionObject {
            member_nodes: Vec::new(),
            visibility_modifier: VisibilityModifier::default()
        }
    }

    pub fn property(mut self, property: Property) -> Self {
        self.member_nodes.push(crate::spec::class::ClassMemberNode::Property(property));
        self
    }

    pub fn function(mut self, function: Function) -> Self {
        self.member_nodes.push(crate::spec::class::ClassMemberNode::Function(function));
        self
    }

    pub fn subclass(mut self, subclass: Class) -> Self {
        self.member_nodes.push(crate::spec::class::ClassMemberNode::Subclass(subclass));
        self
    }

    pub fn init(mut self, block: CodeBlock) -> Self {
        self.member_nodes.push(crate::spec::class::ClassMemberNode::InitBlock(block));
        self
    }

    pub fn visibility_modifier(mut self, visibility_modifier: VisibilityModifier) -> Self {
        self.visibility_modifier = visibility_modifier;
        self
    }
}

impl RenderKotlin for CompanionObject {
    fn render(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();
        code.with_nested(self.visibility_modifier.render());
        code.with_space();
        code.with_atom(tokens::keyword::COMPANION);
        code.with_space();
        code.with_atom(tokens::keyword::OBJECT);
        code.with_space();
        code.with_curly_brackets(|code| {
            for node in &self.member_nodes {
                code.with_nested(node.render());
                code.with_new_line();
            }
        });
        code
    }
}

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::{Class, CodeBlock, CompanionObject, Function, Parameter, Name, Property, Type};

    #[test]
    fn companion_object() {
        let companion = CompanionObject::new()
            .property(
                Property::new(Name::from("name"), Type::string())
                    .initializer(CodeBlock::atom("\"John Doe\""))
            )
            .function(
                Function::new(Name::from("printName"))
                    .parameter(Parameter::new(Name::from("name"), Type::string()))
                    .body(CodeBlock::statement("println(name)"))
            );

        let code = companion.render().to_string();
        assert_eq!(
            code,
            "public companion object {\n    public final val name: kotlin.String = \"John Doe\"\n    public fun printName(name: kotlin.String): kotlin.Unit {\n        println(name)\n    }\n}"
        );
    }

    #[test]
    fn companion_object_with_subclass() {
        let subclass = Class::new("Subclass".into());
        let companion = CompanionObject::new()
            .subclass(subclass);

        let code = companion.render().to_string();
        assert_eq!(code, "public companion object {\n    public final class Subclass {\n\n    }\n}");
    }

    #[test]
    fn companion_object_with_init_block() {
        let companion = CompanionObject::new()
            .init(CodeBlock::statement("println(\"init block\")"));

        let code = companion.render().to_string();
        assert_eq!(code, "public companion object {\n    init{\n        println(\"init block\")\n    }\n}");
    }
}