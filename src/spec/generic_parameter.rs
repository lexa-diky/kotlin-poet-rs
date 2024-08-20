use crate::spec::{CodeBlock, GenericInvariance, Name, Type};
use crate::tokens;

/// Describes a generic parameter of a class or function.
///
/// Note, because [crate::io::RenderKotlin] implementation for this struct would be ambiguous, it is not implemented.
#[derive(Debug, Clone)]
pub struct GenericParameter {
    name: Name,
    /// Invariance of the generic parameter, only available for function's generic parameters
    invariance: Option<GenericInvariance>,
    type_boundaries: Vec<Type>,
    is_reified: bool
}

impl GenericParameter {
    /// Creates new [GenericParameter] with a given name, no invariance modifier or type boundaries.
    pub fn new<NameLike: Into<Name>>(name: NameLike) -> Self {
        GenericParameter {
            name: name.into(),
            invariance: None,
            type_boundaries: Vec::new(),
            is_reified: false
        }
    }

    /// Sets [GenericInvariance]. Should not be used with function generic parameters.
    pub fn invariance(mut self, invariance: GenericInvariance) -> Self {
        self.invariance = Some(invariance);
        self
    }

    /// Adds new type boundary to the generic parameter.
    /// This method could be called multiple times to add multiple type boundaries.
    pub fn type_boundary<TypeLike: Into<Type>>(mut self, boundary: TypeLike) -> Self {
        self.type_boundaries.push(boundary.into());
        self
    }

    pub fn reified(mut self, flag: bool) -> Self {
        self.is_reified = flag;
        self
    }

    pub(crate) fn render_definition(&self) -> CodeBlock {
        let mut code = CodeBlock::empty();
        if let Some(invariance) = &self.invariance {
            code.push_renderable(invariance);
            code.push_space();
        }
        if self.is_reified {
            code.push_static_atom(tokens::keyword::REIFIED);
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
                inner.push_static_atom(tokens::COLON);
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
        code.push_static_atom(tokens::keyword::WHERE);
        code.push_space();
        code.push_comma_separated(&boundary_code_blocks);
        code.push_space();

        code
    }
}
