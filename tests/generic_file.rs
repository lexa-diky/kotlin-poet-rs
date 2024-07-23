use std::path::Path;
use std::str::FromStr;

use kotlin_poet_rs::io::RenderKotlin;
use kotlin_poet_rs::spec::{AccessModifier, Class, ClassInheritanceModifier, CodeBlock, Function, FunctionParameter, KotlinFile, MemberInheritanceModifier, Name, Package, Property, PropertyGetter, PropertySetter, Type};

#[test]
fn generic_test() {
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
        .access_modifier(AccessModifier::Private)
        .inheritance_modifier(MemberInheritanceModifier::Abstract)
        .parameter(FunctionParameter::new(Name::from("args"), Type::array(Type::string())))
        .returns(Type::array(Type::string()))
        .receiver(Type::int())
        .body(CodeBlock::statement("val a = 2"));

    let class = Class::new(Name::from("Person"))
        .access_modifier(AccessModifier::Private)
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