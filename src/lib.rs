pub mod io;
pub mod spec;

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::{CodeBlock, Function, Name, Type};

    #[test]
    fn it_works() {
        let mut block = Function::new(Name::from("main"))
            .receiver(Type::short())
            .access_modifier(crate::spec::AccessModifier::Public)
            .parameter(Name::from("args"), Type::array(Type::string()))
            .parameter(Name::from("args2"), Type::array(Type::int()))
            .body(CodeBlock::statement("return 23"));

        println!("{}", block.render_without_context());
    }
}
