use crate::ast;

#[allow(dead_code)]
pub fn walk<'a, F>(root: &'a ast::Expr, f : &F)
    where F: Fn(&'a ast::Expr){
    f(root);
    match root {
        ast::Expr::Cons(box lhs, box rhs) => {
            walk(&lhs, f);
            walk(&rhs, f);
        },
        _ => ()
    }
}