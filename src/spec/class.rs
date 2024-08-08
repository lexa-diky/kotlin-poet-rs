use crate::io::RenderKotlin;
use crate::spec::{VisibilityModifier, Argument, ClassInheritanceModifier, CodeBlock, CompanionObject, Function, GenericParameter, Name, PrimaryConstructor, Property, SecondaryConstructor, Type, Annotation, KDoc};
use crate::spec::annotation::mixin_annotation_mutators;
use crate::spec::kdoc::{KdocSlot, mixin_kdoc_mutators};
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
    fn render_into(&self, block: &mut CodeBlock) {
        match self {
            ClassMemberNode::Property(property) => {
                block.push_renderable(property);
            }
            ClassMemberNode::Function(function) => {
                block.push_renderable(function);
            }
            ClassMemberNode::Subclass(subclass) => {
                block.push_renderable(subclass);
            }
            ClassMemberNode::SecondaryConstructor(secondary_constructor) => {
                block.push_renderable(secondary_constructor);
            }
            ClassMemberNode::InitBlock(code) => {
                block.push_atom(tokens::keyword::INIT);
                block.push_curly_brackets(|block| {
                    block.push_renderable(code);
                });
            }
        }
    }
}

#[derive(Debug, Clone)]
struct EnumInstance {
    name: Name,
    arguments: Vec<Argument>,
}

/// Defines Kotlin's class like entity. This could represent any 'flavour' of class: enum, interface, e.t.c.
/// To change type of class please use [Class::inheritance_modifier].
///
/// #Example
///
/// ## Simple class
/// ```
/// use kotlin_poet_rs::io::RenderKotlin;
/// use kotlin_poet_rs::spec::{Class, Name};
///
/// let class = Class::new(Name::from("Person"));
///
///  assert_eq!(class.render_string(), "public final class Person {\n\n}");
/// ```
///
/// ## Interface
/// ```
/// use kotlin_poet_rs::io::RenderKotlin;
/// use kotlin_poet_rs::spec::{Class, ClassInheritanceModifier, Name};
///
/// let class = Class::new(Name::from("Person"))
///     .inheritance_modifier(ClassInheritanceModifier::Interface);
///
///  assert_eq!(class.render_string(), "public interface Person {\n\n}");
/// ```
#[derive(Debug, Clone)]
pub struct Class {
    name: Name,
    visibility_modifier: VisibilityModifier,
    inheritance_modifier: ClassInheritanceModifier,
    member_nodes: Vec<ClassMemberNode>,
    enum_instances: Vec<EnumInstance>,
    primary_constructor: Option<PrimaryConstructor>,
    companion_object: Option<CompanionObject>,
    generic_parameters: Vec<GenericParameter>,
    parent_classes: Vec<Type>,
    is_inner: bool,
    annotations: Vec<Annotation>,
    kdoc: KdocSlot,
}

impl Class {
    /// Creates new plain final class.
    pub fn new<NameLike: Into<Name>>(name: NameLike) -> Self {
        Class {
            name: name.into(),
            visibility_modifier: VisibilityModifier::default(),
            inheritance_modifier: ClassInheritanceModifier::default(),
            member_nodes: Vec::default(),
            enum_instances: Vec::default(),
            primary_constructor: None,
            companion_object: None,
            generic_parameters: Vec::default(),
            parent_classes: Vec::default(),
            is_inner: false,
            annotations: Vec::default(),
            kdoc: KdocSlot::default(),
        }
    }

    /// Marks class as inner
    pub fn inner(mut self, flag: bool) -> Self {
        self.is_inner = flag;
        self
    }

    /// Set's class visibility modifier
    pub fn visibility_modifier(mut self, visibility_modifier: VisibilityModifier) -> Self {
        self.visibility_modifier = visibility_modifier;
        self
    }

    /// Changes class type
    pub fn inheritance_modifier(mut self, inheritance_modifier: ClassInheritanceModifier) -> Self {
        self.inheritance_modifier = inheritance_modifier;
        self
    }

    /// Adds property to this class. Properties in body will appear in order this method is called.
    pub fn property(mut self, property: Property) -> Self {
        self.member_nodes.push(ClassMemberNode::Property(property));
        self
    }

    /// Adds function to this class. Functions in body will appear in order this method is called.
    pub fn function(mut self, function: Function) -> Self {
        self.member_nodes.push(ClassMemberNode::Function(function));
        self
    }

    /// Adds subclass to this class. Subclasses in body will appear in order this method is called.
    pub fn subclass(mut self, subclass: Class) -> Self {
        self.member_nodes.push(ClassMemberNode::Subclass(subclass));
        self
    }

    /// Adds enum instance to this class. Enum instances in body will appear in order this method is called.
    /// This method is only valid for enum classes. To change class type to enum please use [Class::inheritance_modifier].
    pub fn enum_instance<NameLike: Into<Name>>(mut self, name: NameLike, arguments: Vec<Argument>) -> Self {
        self.enum_instances.push(EnumInstance {
            name: name.into(),
            arguments,
        });
        self
    }

    /// Adds primary constructor to this class.
    pub fn primary_constructor(mut self, primary_constructor: PrimaryConstructor) -> Self {
        self.primary_constructor = Some(primary_constructor);
        self
    }

    /// Adds secondary constructor to this class. Secondary constructors in body will appear in order this method is called.
    pub fn secondary_constructor(mut self, secondary_constructor: SecondaryConstructor) -> Self {
        self.member_nodes.push(ClassMemberNode::SecondaryConstructor(secondary_constructor));
        self
    }

    /// Adds init block to this class. Init blocks in body will appear in order this method is called.
    pub fn init(mut self, block: CodeBlock) -> Self {
        self.member_nodes.push(ClassMemberNode::InitBlock(block));
        self
    }

    /// Adds companion object to this class.
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

    /// Adds parent class / interface to this class.
    pub fn inherits<TypeLike: Into<Type>>(mut self, parent_type: TypeLike) -> Self {
        self.parent_classes.push(parent_type.into());
        self
    }

    mixin_annotation_mutators!();
    mixin_kdoc_mutators!();
}

impl RenderKotlin for Class {
    fn render_into(&self, block: &mut CodeBlock) {
        block.push_renderable(&self.kdoc);
        for annotation in &self.annotations {
            block.push_renderable(annotation);
            block.push_new_line();
        }

        block.push_renderable(&self.visibility_modifier);
        block.push_space();
        if self.is_inner {
            block.push_atom(tokens::keyword::INNER);
            block.push_space();
        }
        block.push_renderable(&self.inheritance_modifier);
        block.push_space();
        if !matches!(
            self.inheritance_modifier,
            ClassInheritanceModifier::Interface |
            ClassInheritanceModifier::Object
        ) {
            block.push_atom(tokens::keyword::CLASS);
            block.push_space();
        }
        block.push_renderable(&self.name);
        if !self.generic_parameters.is_empty() {
            block.push_angle_brackets(|code| {
                code.push_comma_separated(
                    &self.generic_parameters.iter().map(|it| it.render_definition())
                        .collect::<Vec<CodeBlock>>()
                );
            });
        }
        block.push_space();

        if let Some(primary_constructor) = &self.primary_constructor {
            block.push_renderable(primary_constructor);
            block.push_space();
        }

        if !self.parent_classes.is_empty() {
            block.pop_space();
            block.push_atom(tokens::COLON);
            block.push_space();
            block.push_comma_separated(
                &self.parent_classes
            );
            block.push_space();
        }

        block.push_renderable(
            &GenericParameter::render_type_boundaries_vec_if_required(
                &self.generic_parameters
            )
        );

        block.push_curly_brackets(|class_body_code| {
            class_body_code.push_new_line();

            if !self.enum_instances.is_empty() {
                for (inst_idx, instance) in self.enum_instances.iter().enumerate() {
                    class_body_code.push_renderable(&instance.name);
                    class_body_code.push_round_brackets(|arg_code| {
                        arg_code.push_comma_separated(&instance.arguments);
                    });

                    if inst_idx != self.enum_instances.len() - 1 {
                        class_body_code.push_atom(tokens::COMMA);
                        class_body_code.push_new_line();
                    }
                }

                class_body_code.push_atom(tokens::SEMICOLON);
            }

            for node in &self.member_nodes {
                class_body_code.push_renderable(node);
                class_body_code.push_new_line();
            }

            if let Some(companion_object) = &self.companion_object {
                class_body_code.push_renderable(companion_object);
                class_body_code.push_new_line();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::{Parameter, GenericInvariance, PropertyGetter, PropertySetter, Type, ClassLikeTypeName, Package};
    use super::*;

    #[test]
    fn test_class() {
        let class = Class::new(Name::from("Person"));
        let code = class.render_string();

        assert_eq!(code, "public final class Person {\n\n}");
    }

    #[test]
    fn test_class_with_kdoc() {
        let class = Class::new(Name::from("Person"))
            .kdoc(
                KDoc::from("hello world")
                    .merge(KDoc::from("at here"))
            );
        let code = class.render_string();

        assert_eq!(
            code,
            "/**\n * hello world\n * at here\n */\npublic final class Person {\n\n}"
        );
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

        let code = class.render_string();

        assert_eq!(
            code,
            "public final class Person {\n\n    public final var name: kotlin.String = \"\"\n        set(value) {\n            field = value\n        }\n        get() {\n            return field\n        }\n\n}"
        );
    }

    #[test]
    fn test_enum() {
        let class = Class::new(Name::from("Person"))
            .inheritance_modifier(ClassInheritanceModifier::Enum)
            .enum_instance(Name::from("Alex"), vec![
                Argument::new_positional(CodeBlock::atom("23"))
            ])
            .enum_instance(Name::from("Vova"), vec![
                Argument::new_positional(CodeBlock::atom("23"))
            ])
            ;
        let code = class.render_string();

        assert_eq!(
            code,
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
                        Parameter::new(
                            Name::from("age"),
                            Type::int(),
                        )
                    )
            );

        assert_eq!(
            class.render_string(),
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
            class.render_string(),
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
            class.render_string(),
            "public final class Person {\n\n    init{\n        println(42)\n    }\n}"
        );
    }

    #[test]
    fn test_data_class() {
        let class = Class::new(Name::from("Person"))
            .inheritance_modifier(ClassInheritanceModifier::Data)
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
            class.render_string(),
            "public data class Person public constructor(public final val name: kotlin.String = \"\") {\n\n}"
        );
    }

    #[test]
    fn test_data_class_with_secondary_constructor() {
        let class = Class::new(Name::from("Person"))
            .inheritance_modifier(ClassInheritanceModifier::Data)
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
                        Parameter::new(
                            Name::from("name"),
                            Type::string(),
                        )
                    )
                    .delegate_argument(
                        Argument::new_positional(
                            CodeBlock::atom("name")
                        )
                    )
                    .delegate_argument(
                        Argument::new_positional(
                            CodeBlock::atom("23")
                        )
                    )
                    .body(
                        CodeBlock::statement("println(42)")
                    )
            );

        assert_eq!(
            class.render_string(),
            "public data class Person public constructor(public final val name: kotlin.String, public final val age: kotlin.Int) {\n\n    public constructor(name: kotlin.String) : this(name, 23) {\n        println(42)\n    }\n}"
        );
    }

    #[test]
    fn test_interface() {
        let class = Class::new(Name::from("Person"))
            .inheritance_modifier(ClassInheritanceModifier::Interface);

        assert_eq!(class.render_string(), "public interface Person {\n\n}");
    }

    #[test]
    fn test_abstract() {
        let class = Class::new(Name::from("Person"))
            .inheritance_modifier(ClassInheritanceModifier::Abstract);

        assert_eq!(class.render_string(), "public abstract class Person {\n\n}");
    }

    #[test]
    fn test_object() {
        let class = Class::new(Name::from("Person"))
            .inheritance_modifier(ClassInheritanceModifier::Object);

        assert_eq!(class.render_string(), "public object Person {\n\n}");
    }

    #[test]
    fn test_class_with_inner() {
        let class = Class::new(Name::from("Person"))
            .subclass(
                Class::new("InnerPerson")
                    .inheritance_modifier(ClassInheritanceModifier::Abstract)
                    .inner(true)
            );

        assert_eq!(
            class.render_string(),
            "public final class Person {\n\n    public inner abstract class InnerPerson {\n\n    }\n}"
        );
    }

    #[test]
    fn test_sealed() {
        let class = Class::new(Name::from("Person"))
            .inheritance_modifier(ClassInheritanceModifier::Sealed);

        assert_eq!(class.render_string(), "public sealed class Person {\n\n}");
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

        assert_eq!(class.render_string(), "public final class Box<A, in B, out C> {\n\n}");
    }

    #[test]
    fn test_generic_with_parent() {
        let class = Class::new(Name::from("Box"))
            .generic_parameter(
                GenericParameter::new(Name::from("A"))
                    .invariance(GenericInvariance::In)
                    .type_boundary(Type::string())
            )
            .inherits(
                Type::int()
            );

        assert_eq!(
            class.render_string(),
            "public final class Box<in A>: kotlin.Int where A: kotlin.String {\n\n}"
        );
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

        assert_eq!(class.render_string(), "public final class Box<A, in B, out C> where B: kotlin.String, B: kotlin.Int {\n\n}");
    }

    #[test]
    fn test_with_annotation() {
        let class = Class::new(Name::from("Person"))
            .annotation(
                Annotation::new(ClassLikeTypeName::top_level(
                    Package::from(Vec::new()),
                    Name::from("Deprecated"),
                ))
            );

        assert_eq!(class.render_string(), "@Deprecated()\npublic final class Person {\n\n}");
    }
}