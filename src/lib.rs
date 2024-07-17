pub mod io;
pub mod spec;

#[cfg(test)]
mod tests {
    use crate::io::{RenderContext, RenderKotlin};
    use crate::spec::CodeBlock;

    #[test]
    fn it_works() {
        let mut block = CodeBlock::empty();
        block.with_statement("val a = 2");

        print!("{}", block.render(RenderContext::new().indent()));
    }
}
