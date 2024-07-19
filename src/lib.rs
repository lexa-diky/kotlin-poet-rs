pub mod io;
pub mod spec;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::io::RenderKotlin;
    use crate::spec::{CodeBlock, KotlinFile, Name, Package, Property, PropertyGetter, PropertySetter, Type};

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

        let file = KotlinFile::new(
            Package::from_str("a.b.c").unwrap(),
            Name::from("Test")
        ).property(property);

        println!("{}", file.render().to_string())
    }
}
