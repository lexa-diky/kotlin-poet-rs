pub mod io;
pub mod spec;

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::CodeBlock;

    #[test]
    fn it_works() {
        let my_import = CodeBlock::empty()
            .statement("val a = 5")
            .indent()
            .statement("val b = 10")
            .nest(
                CodeBlock::empty()
                    .statement("println(\"Hello, world!\")")
            )
            .unindent()
            .statement("val c = a + b");

        println!("{}", my_import.render())
    }
}
