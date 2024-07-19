use std::str::FromStr;
use kotlin_poet_rs::io::RenderKotlin;
use kotlin_poet_rs::spec::{AccessModifier, CodeBlock, Function, KotlinFile, MemberInheritanceModifier, Name, Package, Property, PropertyGetter, PropertySetter, Type};


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
        .parameter(Name::from("text"), Type::string())
        .returns(Type::array(Type::string()))
        .receiver(Type::int())
        .body(CodeBlock::statement("val a = 2"));


    let file = KotlinFile::new(
        Package::from_str("a.b.c").unwrap(),
        Name::from("Test"),
    )
        .property(property)
        .function(function);

    println!("{}", file.render().to_string())
}