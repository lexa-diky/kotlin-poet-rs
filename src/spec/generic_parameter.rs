use crate::io::RenderKotlin;
use crate::spec::{CodeBlock, GenericInvariance, Name, Type};
use crate::tokens;

/// Describes a generic parameter of a class or function.
///
/// Note, because [RenderKotlin] implementation for this struct would be ambiguous, it is not implemented.
#[derive(Debug, Clone)]
pub struct GenericParameter {
    name: Name,
    /// Invariance of the generic parameter, only available for function's generic parameters
    invariance: Option<GenericInvariance>,
    type_boundaries: Vec<Type>,
}

impl GenericParameter {
    /// Creates new [GenericParameter] with a given name, no invariance modifier or type boundaries.
    pub fn new(name: Name) -> Self {
        GenericParameter {
            name,
            invariance: None,
            type_boundaries: Vec::new(),
        }
    }

    /// Sets [GenericInvariance]. Should not be used with function generic parameters.
    pub fn invariance(mut self, invariance: GenericInvariance) -> Self {
        self.invariance = Some(invariance);
        self
    }

    /// Adds new type boundary to the generic parameter.
    /// This method could be called multiple times to add multiple type boundaries.
    pub fn type_boundary(mut self, boundary: Type) -> Self {
        self.type_boundaries.push(boundary);
        self
    }

    pub(crate) fn render_definition(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();
        if let Some(invariance) = &self.invariance {
            code.push_renderable(invariance);
            code.push_space();
        }
        code.push_renderable(&self.name);
        code
    }

    pub(crate) fn render_type_boundaries(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();
        code.push_comma_separated(
            &self.type_boundaries.iter().map(|boundary| {
                let mut inner = CodeBlock::empty();
                inner.push_renderable(&self.name);
                inner.push_atom(tokens::COLON);
                inner.push_space();
                inner.push_renderable(boundary);
                inner
            }).collect::<Vec<CodeBlock>>()
        );
        code
    }

    pub(crate) fn render_type_boundaries_vec_if_required(vec: &[GenericParameter]) -> CodeBlock {
        let boundary_code_blocks = vec.iter().filter(|parameter| !parameter.type_boundaries.is_empty())
            .map(|parameter| {
                parameter.render_type_boundaries()
            })
            .collect::<Vec<CodeBlock>>();

        if boundary_code_blocks.is_empty() {
            return CodeBlock::empty();
        }

        let mut code = CodeBlock::empty();
        code.push_atom(tokens::keyword::WHERE);
        code.push_space();
        code.push_comma_separated(&boundary_code_blocks);
        code.push_space();

        code
    }
}
