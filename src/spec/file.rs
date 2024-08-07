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
            block.with_embedded(annotation);
            block.with_new_line();
        }
        if !self.annotations.is_empty() {
            block.with_new_line();
        }

        if let Some(package) = &self.package {
            block.with_atom(tokens::keyword::PACKAGE);
            block.with_space();
            block.with_embedded(package);
            block.with_new_line();
        }

        for import in &self.imports {
            block.with_embedded(import);
            block.with_new_line();
        }

        for node in &self.nodes {
            match node {
                KotlinFileNode::Property(property) => {
                    block.with_new_line();
                    block.with_embedded(property);
                    block.with_new_line();
                }
                KotlinFileNode::Function(function) => {
                    block.with_new_line();
                    block.with_embedded(function);
                    block.with_new_line();
                }
                KotlinFileNode::TypeAlias(type_alias) => {
                    block.with_new_line();
                    block.with_embedded(type_alias);
                    block.with_new_line();
                }
                KotlinFileNode::Class(class) => {
                    block.with_new_line();
                    block.with_embedded(class);
                    block.with_new_line();
                }
            }
        }
    }
}