
#[derive(Debug)]
pub enum Lexeme {
    LParen { index : usize },
    RParen { index : usize },
    Comma { index : usize },
    Float { value : f64, start : usize, end : usize },
    Symbol { value : String, start : usize, end : usize },
    ColonSymbol { value : String, start : usize, end : usize },
}

impl Lexeme { 
    fn start(&self) -> usize {
        0
    }
    fn end(&self) -> usize {
        0
    }
}

#[derive(Debug)]
pub enum FuncDefOrExpr {
    Expr(Expr),
}

#[derive(Debug)] 
pub enum Expr {
    TupleCons
}