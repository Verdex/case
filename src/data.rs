
use std::collections::HashMap;

#[derive(Debug)]
pub enum Lexeme {
    LParen { index : usize },
    RParen { index : usize },
    Comma { index : usize },
    SemiColon { index : usize },
    Float { value : f64, start : usize, end : usize },
    Symbol { value : String, start : usize, end : usize },
    ColonSymbol { value : String, start : usize, end : usize },
}

#[derive(Debug)]
pub enum DefOrExpr {
    Expr(Expr),
    FnDef(FnDef),
}

#[derive(Debug)]
pub struct FnDef {
    pub name : String,
    pub params : Vec<String>,
    pub body : Expr,
    pub l_start : usize,
    pub l_end : usize,
}

#[derive(Debug)] 
pub enum Expr {
    TupleCons { params: Vec<Expr>, l_start : usize, l_end : usize },
    Float { value : f64, l_start : usize, l_end : usize },
    Symbol { value : String, l_start : usize, l_end : usize },
    Var { value : String, l_start : usize, l_end : usize },
    Call { fn_expr : Box<Expr>, params : Vec<Expr> },
}

#[derive(Debug)]
pub enum Pat {
    Float { value: f64, l_start : usize, l_end : usize },
    Symbol { value : String, l_start : usize, l_end : usize },
    UnboundVariable { value : String, l_start : usize, l_end : usize },
    Tuple { params : Vec<Pat>, l_start : usize, l_end : usize },
}

#[derive(Debug, Clone)]
pub enum IlData {
    Float(f64),
    Symbol(String),
    String(String),
    Tuple(Vec<IlData>),
    List(Vec<IlData>),
    Pattern(IlPat),
    Code(Vec<Il>),
}

#[derive(Debug, Clone)]
pub enum IlPat {
    Float(f64),
    Symbol(String),
    UnboundVariable(String),
    Tuple(Vec<IlPat>),
}

#[derive(Debug, Clone)]
pub enum Il {
    Push(IlData),
    TupleCons(usize),
    Match,
    Def,
    Exit,
}

#[derive(Debug)] 
pub struct ExeEnv {
    pub data_stack : Vec<IlData>,
    pub def_stack : Vec<HashMap<String, IlData>>,
}

#[derive(Debug)]
pub enum CompileError { }
// TODO: Error impl