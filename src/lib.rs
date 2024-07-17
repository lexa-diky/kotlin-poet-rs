pub mod io;
pub mod spec;

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::{AccessModifier, CodeBlock, Function, MemberInheritanceModifier, Name, Property, PropertyGetter, PropertySetter, Type};

    #[test]
    fn it_works() {
        let function = Function::new(Name::from("main"))
            .access_modifier(AccessModifier::Private)
            .parameter(
                Name::from("args"),
                Type::array(Type::string())
            )
            .receiver(Type::unit())
            .inheritance_modifier(MemberInheritanceModifier::Open)
            .body(
                CodeBlock::empty()
                    .statement("val a = 5")
                    .statement("val b = 10")
                    .nest(
                        CodeBlock::empty()
                            .statement("println(\"Hello, world!\")")
                    )
                    .statement("val c = a + b")
            );

        println!("{}", function.render_without_context());

        let property = Property::new(Name::from("name"), Type::string())
            .access_modifier(AccessModifier::Public)
            .inheritance_modifier(MemberInheritanceModifier::Open)
            .initializer(
                CodeBlock::empty()
                    .statement("\"John Doe\"")
            )
            .getter(
                PropertyGetter::new(
                    CodeBlock::empty()
                        .statement("return field")
                )
            )
            .setter(
                PropertySetter::new(
                    CodeBlock::empty()
                        .statement("field = value")
                )
            )
            .mutable(true);

        println!("{}", property.render_without_context());
    }
}
