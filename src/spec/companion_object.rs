use crate::io::RenderKotlin;
use crate::spec::{VisibilityModifier, Class, CodeBlock, Function, Property};
use crate::spec::kdoc::{KdocSlot, mixin_kdoc_mutators};
use crate::tokens;

/// Companion object for class
/// Can contain properties, functions, subclasses and init blocks like class itself.
#[derive(Debug, Clone)]
pub struct CompanionObject {
    visibility_modifier: VisibilityModifier,
    member_nodes: Vec<crate::spec::class::ClassMemberNode>,
    kdoc: KdocSlot
}

impl CompanionObject {
    pub fn new() -> Self {
        CompanionObject {
            member_nodes: Vec::new(),
            visibility_modifier: VisibilityModifier::default(),
            kdoc: KdocSlot::default()
        }
    }

    /// Adds a property to the companion object
    pub fn property(mut self, property: Property) -> Self {
        self.member_nodes.push(crate::spec::class::ClassMemberNode::Property(property));
        self
    }

    /// Adds a function to the companion object
    pub fn function(mut self, function: Function) -> Self {
        self.member_nodes.push(crate::spec::class::ClassMemberNode::Function(function));
        self
    }

    /// Adds a subclass to the companion object
    pub fn subclass(mut self, subclass: Class) -> Self {
        self.member_nodes.push(crate::spec::class::ClassMemberNode::Subclass(subclass));
        self
    }

    /// Adds an init block to the companion object
    pub fn init(mut self, block: CodeBlock) -> Self {
        self.member_nodes.push(crate::spec::class::ClassMemberNode::InitBlock(block));
        self
    }

    /// Sets the visibility modifier for the companion object
    pub fn visibility_modifier(mut self, visibility_modifier: VisibilityModifier) -> Self {
        self.visibility_modifier = visibility_modifier;
        self
    }

    mixin_kdoc_mutators!();
}

impl RenderKotlin for CompanionObject {
    fn render_into(&self, block: &mut CodeBlock) {
        block.push_renderable(&self.kdoc);
        block.push_renderable(&self.visibility_modifier);
        block.push_space();
        block.push_atom(tokens::keyword::COMPANION);
        block.push_space();
        block.push_atom(tokens::keyword::OBJECT);
        block.push_space();
        block.push_curly_brackets(|code| {
            for node in &self.member_nodes {
                code.push_renderable(node);
                code.push_new_line();
            }
        });
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

        let code = companion.render_string();
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

        let code = companion.render_string();
        assert_eq!(code, "public companion object {\n    public final class Subclass {\n\n    }\n}");
    }

    #[test]
    fn companion_object_with_kdoc() {
        let companion = CompanionObject::new()
            .kdoc_str("Hello\nWorld");

        let code = companion.render_string();
        assert_eq!(code, "/**\n * Hello\n * World\n */\npublic companion object {\n}");
    }

    #[test]
    fn companion_object_with_init_block() {
        let companion = CompanionObject::new()
            .init(CodeBlock::statement("println(\"init block\")"));

        let code = companion.render_string();
        assert_eq!(code, "public companion object {\n    init{\n        println(\"init block\")\n    }\n}");
    }
}