pub mod io;
pub mod spec;

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::{AccessModifier, CodeBlock, Function, MemberInheritanceModifier, Name, Type};

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

        println!("{}", function.render())
    }
}
