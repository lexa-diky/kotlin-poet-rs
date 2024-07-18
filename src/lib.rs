pub mod io;
pub mod spec;

#[cfg(test)]
mod tests {
    use crate::io::{RenderContext, RenderKotlin};
    use crate::spec::{CodeBlock, CodeBlockNode, Function, Name, Property, PropertyGetter, PropertySetter, Type};

    #[test]
    fn it_works() {
        let property = Property::new(
            Name::from("name"),
            Type::string()
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

        println!("{}", property.render().to_string())
    }
}
