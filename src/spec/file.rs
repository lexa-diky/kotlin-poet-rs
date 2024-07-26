use crate::io::RenderKotlin;
use crate::spec::{Annotation, AnnotationTarget, Class, ClassLikeTypeName, CodeBlock, Function, Import, Package, Property, TypeAlias};
use crate::tokens;

#[derive(Debug, Clone)]
enum KotlinFileNode {
    Property(Property),
    Function(Function),
    TypeAlias(TypeAlias),
    Class(Class),
}

/// Represents a Kotlin file.
#[derive(Debug, Clone)]
pub struct KotlinFile {
    package: Option<Package>,
    imports: Vec<Import>,
    nodes: Vec<KotlinFileNode>,
    annotations: Vec<Annotation>
}

impl KotlinFile {
    pub fn new(package: Package) -> Self {
        KotlinFile {
            package: Some(package),
            imports: Vec::new(),
            nodes: Vec::new(),
            annotations: Vec::new()
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

    pub fn annotation(mut self, annotation: Annotation) -> Self {
        self.annotations.push(
            annotation
                .target(AnnotationTarget::File)
        );
        self
    }
}

impl From<ClassLikeTypeName> for KotlinFile {
    fn from(value: ClassLikeTypeName) -> Self {
        let package = value.package;
        KotlinFile::new(package)
    }
}

impl RenderKotlin for KotlinFile {
    fn render(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();
        for annotation in &self.annotations {
            code.with_nested(annotation.render());
            code.with_new_line();
        }
        if !self.annotations.is_empty() {
            code.with_new_line();
        }

        if let Some(package) = &self.package {
            code.with_atom(tokens::keyword::PACKAGE);
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