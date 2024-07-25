use std::path::Path;
use std::str::FromStr;

use kotlin_poet_rs::io::RenderKotlin;
use kotlin_poet_rs::spec::{VisibilityModifier, Class, ClassInheritanceModifier, CodeBlock, CompanionObject, Function, Parameter, KotlinFile, MemberInheritanceModifier, Name, Package, PrimaryConstructor, Property, PropertyGetter, PropertySetter, Type};

#[test]
fn generic_file() {
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

    let function = Function::new(Name::from("main"))
        .operator(true)
        .inline(true)
        .visibility_modifier(VisibilityModifier::Private)
        .inheritance_modifier(MemberInheritanceModifier::Abstract)
        .parameter(Parameter::new(Name::from("args"), Type::array(Type::string())))
        .returns(Type::array(Type::string()))
        .receiver(Type::int())
        .body(CodeBlock::statement("val a = 2"));

    let class = Class::new(Name::from("Person"))
        .visibility_modifier(VisibilityModifier::Private)
        .inheritance_modifier(ClassInheritanceModifier::Abstract)
        .property(property.clone())
        .function(function.clone())
        .subclass(Class::new(Name::from("Person")));

    let file = KotlinFile::new(
        Package::from_str("a.b.c").unwrap(),
    ).property(property)
        .function(function)
        .class(class);

    assert_rendered(
        "tests/samples/generic_file.kt",
        &file.render(),
    )
}

#[test]
fn class_with_companion_object() {
    let class = Class::new(Name::from("Person"))
        .primary_constructor(
            PrimaryConstructor::new()
                .property(
                    Property::new(
                        Name::from("name"),
                        Type::string(),
                    )
                )
        )
        .companion_object(
            CompanionObject::new()
                .property(
                    Property::new(
                        Name::from("nameCmp"),
                        Type::string(),
                    ).initializer(
                        CodeBlock::atom("\"Alex\"")
                    )
                )
                .function(
                    Function::new(Name::from("printName"))
                        .parameter(
                            Parameter::new(
                                Name::from("name"),
                                Type::string(),
                            )
                        )
                        .body(
                            CodeBlock::statement("println(nameCmp)")
                        )
                )
        );

    assert_rendered(
        "tests/samples/class_with_companion_object.kt",
        &&class.render(),
    )
}

pub fn assert_rendered(
    expected_path: &str,
    code: &CodeBlock,
) {
    let expected_path = Path::new(expected_path);
    if !expected_path.exists() {
        std::fs::write(
            expected_path,
            code.to_string(),
        ).unwrap();
    } else {
        let expected_code = std::fs::read_to_string(expected_path);
        assert_eq!(
            expected_code.unwrap(),
            code.to_string(),
        );
    }
}