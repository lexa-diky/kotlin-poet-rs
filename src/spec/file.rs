use crate::io::RenderKotlin;
use crate::spec::{Annotation, AnnotationTarget, Class, ClassLikeTypeName, CodeBlock, Comment, Function, Import, Package, Property, TypeAlias};
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
    annotations: Vec<Annotation>,
    header_comments: Vec<Comment>,
}

impl KotlinFile {
    /// Creates file in specified [package]
    pub fn new(package: Package) -> Self {
        KotlinFile {
            package: Some(package),
            imports: Vec::new(),
            nodes: Vec::new(),
            annotations: Vec::new(),
            header_comments: Vec::new(),
        }
    }

    /// Creates new file without package statement
    pub fn root() -> Self {
        KotlinFile {
            package: None,
            imports: Vec::new(),
            nodes: Vec::new(),
            annotations: Vec::new(),
            header_comments: Vec::new(),
        }
    }

    /// Adds new comment in first line.
    ///
    /// This method can be called multiple times to add multiple comments,
    /// they will appear in order on enw lines.
    pub fn header_comment(mut self, comment: Comment) -> Self {
        self.header_comments.push(comment);
        self
    }

    /// Adds new import to the file.
    pub fn import(mut self, import: Import) -> Self {
        self.imports.push(import);
        self
    }

    /// Adds new property to the file.
    pub fn property(mut self, property: Property) -> Self {
        self.nodes.push(KotlinFileNode::Property(property));
        self
    }

    /// Adds new function to the file.
    pub fn function(mut self, function: Function) -> Self {
        self.nodes.push(KotlinFileNode::Function(function));
        self
    }

    /// Adds new type alias to the file.
    pub fn type_alias(mut self, type_alias: TypeAlias) -> Self {
        self.nodes.push(KotlinFileNode::TypeAlias(type_alias));
        self
    }

    /// Adds new class to the file.
    pub fn class(mut self, class: Class) -> Self {
        self.nodes.push(KotlinFileNode::Class(class));
        self
    }

    /// Adds new annotation to the file.
    /// Added annotation will be forced to have [AnnotationTarget::File] target.
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
        if !self.header_comments.is_empty() {
            for comment in &self.header_comments {
                block.push_renderable(comment);
                block.push_new_line();
            }
            block.push_new_line()
        }

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

#[cfg(test)]
mod tests {
    use crate::spec::{Comment, Function, KotlinFile};
    use crate::io::RenderKotlin;

    #[test]
    fn test_root_file() {
        let file = KotlinFile::root()
            .function(Function::new("main".into()));

        assert_eq!(
            file.render_string(),
            "public fun main(): kotlin.Unit",
        )
    }

    #[test]
    fn test_file_with_header_comments() {
        let file = KotlinFile::new("com.example".into())
            .header_comment(Comment::from("This is a header comment"))
            .header_comment(Comment::from("This is another header comment"));

        assert_eq!(
            file.render_string(),
            "// This is a header comment\n// This is another header comment\n\npackage com.example"
        )
    }
}