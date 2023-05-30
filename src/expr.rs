use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

#[macro_export]
macro_rules! generate_expr {
        ($($x:ident => ($($p:ident),+)),*) => {
            pub(crate) enum Expr{

                    $(
                        $x(
                        $(
                            Box<$p>
                        ),*
                )

                    ),*
            }
        }
}

generate_expr![
        Binary => (Expr, Token, Expr),
        Grouping => (Expr, Expr),
        Unary => (Token, Expr)
];
