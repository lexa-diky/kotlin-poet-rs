pub mod io;
pub mod spec;

#[cfg(test)]
mod tests {
    use crate::io::{RenderContext, RenderKotlin};
    use crate::spec::{CodeBlock, CodeBlockNode, Function, Name, Type};

    #[test]
    fn it_works() {
        let mut body = CodeBlock::empty();
        body.with_statement("println(\"Hello, World!\")");
        body.with_statement("hello world");

        let mut block = Function::new(Name::from("main"))
            .receiver(Type::short())
            .access_modifier(crate::spec::AccessModifier::Public)
            .parameter(Name::from("args"), Type::array(Type::string()))
            .parameter(Name::from("args2"), Type::array(Type::int()))
            .body(body)
            .suspended(true)
            .inline(true);

        for x in block.render(RenderContext::new()).nodes {
            if matches!(x, CodeBlockNode::Space) { continue }
            println!("{:?}", x);
        }

        println!("{}", block.render_without_context());
    }
}
