pub mod io;
pub mod spec;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::io::RenderKotlin;
    use crate::spec::{AccessModifier, ClassLikeParameter, ClassLikeTypeName, CodeBlock, Function, Name, Package, ParameterType, TypeName};

    #[test]
    fn it_works() {
        let function = Function::new(Name::from("main"))
            .access_modifier(AccessModifier::Private)
            .parameter(
                Name::from("args"),
                ParameterType::ClassLike(
                    ClassLikeParameter::new(
                        TypeName::ClassLike(
                            ClassLikeTypeName::simple(
                                Package::from_str("kotlin").unwrap(),
                                Name::from("Array"),
                            )
                        )
                    ).generic_argument(
                        ParameterType::Generic(Name::from("String"))
                    )
                ),
            )
            .body(
                CodeBlock::empty()
                    .statement("val a = 5")
                    .indent()
                    .statement("val b = 10")
                    .nest(
                        CodeBlock::empty()
                            .statement("println(\"Hello, world!\")")
                    )
                    .unindent()
                    .statement("val c = a + b")
            );

        println!("{}", function.render())
    }
}
