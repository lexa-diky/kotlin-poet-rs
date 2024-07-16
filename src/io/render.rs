pub trait RenderKotlin {

    fn render_into(&self, buffer: &mut crate::io::CodeBuffer) {
        let content = RenderKotlin::render(self);
        buffer.push(content.as_str())
    }

    fn render(&self) -> String;
}