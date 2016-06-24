#[derive(Debug)]
pub struct attr {
    // FIXME: super?
    pub pre: Option<super::syntax::ast::LitKind>,
    pub post: Option<super::syntax::ast::LitKind>,
}
