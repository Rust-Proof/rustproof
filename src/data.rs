#[derive(Debug)]
struct attr {
    pre: Option<syntax::ast::LitKind>,
    post: Option<syntax::ast::LitKind>,
}
