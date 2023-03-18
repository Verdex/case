
#[derive(Debug)]
pub enum Lexeme {
    LParen { index : usize },
    RParen { index : usize },
    Comma { index : usize },
    Float { value : f64, start : usize, end : usize },
    Symbol { value : String, start : usize, end : usize },
    ColonSymbol { value : String, start : usize, end : usize },
}

#[derive(Debug)]
pub enum DefOrExpr {
    Expr(Expr),
}

#[derive(Debug)] 
pub enum Expr {
    TupleCons { params: Vec<Expr>, l_start : usize, l_end : usize },
    Float { value : f64, l_start : usize, l_end : usize },
    Symbol { value : String, l_start : usize, l_end : usize },
}

#[derive(Debug)]
pub enum Pat {
    UnboundVariable(&'static str),
    Tuple(Vec<Pat>),
}