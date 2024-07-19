use crate::io::{RenderKotlin, tokens};
use crate::spec::{Class, CodeBlock, Function, Import, Name, Package, Property, TypeAlias};

#[derive(Debug, Clone)]
enum KotlinFileNode {
    Property(Property),
    Function(Function),
    TypeAlias(TypeAlias),
    Class(Class)
}

#[derive(Debug, Clone)]
pub struct KotlinFile {
    name: Name,
    package: Option<Package>,
    imports: Vec<Import>,
    nodes: Vec<KotlinFileNode>
}

impl KotlinFile {

    pub fn new(package: Package, name: Name) -> Self {
        KotlinFile {
            name,
            package: Some(package),
            imports: Vec::new(),
            nodes: Vec::new()
        }
    }

    pub fn import(mut self, import: Import) -> Self {
        self.imports.push(import);
        self
    }

    pub fn property(mut self, property: Property) -> Self {
        self.nodes.push(KotlinFileNode::Property(property));
        self
    }

    pub fn function(mut self, function: Function) -> Self {
        self.nodes.push(KotlinFileNode::Function(function));
        self
    }

    pub fn type_alias(mut self, type_alias: TypeAlias) -> Self {
        self.nodes.push(KotlinFileNode::TypeAlias(type_alias));
        self
    }

    pub fn class(mut self, class: Class) -> Self {
        self.nodes.push(KotlinFileNode::Class(class));
        self
    }
}

impl RenderKotlin for KotlinFile {
    fn render(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();

        if let Some(package) = &self.package {
            code.with_atom(tokens::KW_PACKAGE);
            code.with_space();
            code.with_nested(package.render());
            code.with_new_line();
        }

        for import in &self.imports {
            code.with_nested(import.render());
            code.with_new_line();
        }

        for node in &self.nodes {
            match node {
                KotlinFileNode::Property(property) => {
                    code.with_new_line();
                    code.with_nested(property.render());
                    code.with_new_line();
                }
                KotlinFileNode::Function(function) => {
                    code.with_new_line();
                    code.with_nested(function.render());
                    code.with_new_line();
                }
                KotlinFileNode::TypeAlias(type_alias) => {
                    code.with_new_line();
                    code.with_nested(type_alias.render());
                    code.with_new_line();
                }
                KotlinFileNode::Class(class) => {
                    code.with_new_line();
                    code.with_nested(class.render());
                    code.with_new_line();
                }
            }
        }

        code
    }
}