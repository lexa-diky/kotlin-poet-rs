use std::str::FromStr;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kotlin_poet_rs::io::RenderKotlin;
use kotlin_poet_rs::spec::{AccessModifier, CodeBlock, Function, FunctionParameter, KotlinFile, MemberInheritanceModifier, Name, Package, Property, PropertyGetter, PropertySetter, Type};

fn render_generic_file() -> String {
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
        .parameter(FunctionParameter::new(Name::from("text"), Type::string()))
        .returns(Type::array(Type::string()))
        .receiver(Type::int())
        .body(CodeBlock::statement("val a = 2"));


    let file = KotlinFile::new(
        Package::from_str("a.b.c").unwrap(),
        Name::from("Test"),
    )
        .property(property)
        .function(function);

    file.render().to_string()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generic file", |b| b.iter(||
    black_box(
        render_generic_file()
    )
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);