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
    fn render_into(&self, block: &mut CodeBlock) {
        for annotation in &self.annotations {
            block.push_renderable(annotation);
            block.push_new_line();
        }
        if !self.annotations.is_empty() {
            block.push_new_line();
        }

        if let Some(package) = &self.package {
            block.push_atom(tokens::keyword::PACKAGE);
            block.push_space();
            block.push_renderable(package);
            block.push_new_line();
        }

        for import in &self.imports {
            block.push_renderable(import);
            block.push_new_line();
        }

        for node in &self.nodes {
            match node {
                KotlinFileNode::Property(property) => {
                    block.push_new_line();
                    block.push_renderable(property);
                    block.push_new_line();
                }
                KotlinFileNode::Function(function) => {
                    block.push_new_line();
                    block.push_renderable(function);
                    block.push_new_line();
                }
                KotlinFileNode::TypeAlias(type_alias) => {
                    block.push_new_line();
                    block.push_renderable(type_alias);
                    block.push_new_line();
                }
                KotlinFileNode::Class(class) => {
                    block.push_new_line();
                    block.push_renderable(class);
                    block.push_new_line();
                }
            }
        }
    }
}