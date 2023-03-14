
use renounce::*;

use crate::data::*;

macro_rules! input {
    () => { &mut (impl Iterator<Item = (usize, Lexeme)> + Clone) }
}


pub fn parse(input : input!()) -> Result<Vec<DefOrExpr>, ParseError> {
    Ok(vec![])
}


fn parse_expr(input : input!()) -> Result<Expr, ParseError> {
    alt!( input => parse_float; parse_symbol )
}

pat!(parse_float<'a> : (usize, Lexeme) => (usize, Lexeme) = x @ (_, Lexeme::Float { .. }) => x);
pat!(parse_symbol<'a> : (usize, Lexeme) => (usize, Lexeme) = x @ (_, Lexeme::ColonSymbol{ .. }) => x);

fn parse_tuple_cons(input : input!()) -> Result<Expr, ParseError> {
    fn parse_expr_comma(input : input!()) -> Result<Expr, ParseError> {
        // TODO put start and stop data in return type?
        parser!(input => {
            expr <= parse_expr;
            comma <= parse_comma;
            select expr
        })
    }

    parser!(input => {
        l_paren <= parse_l_paren;
        exprs <= * parse_expr_comma;
        maybe_last_expr <= ? parse_expr;
        r_paren <= parse_r_paren;
        select {
            let mut exprs = exprs;
            match maybe_last_expr {
                Some(last_expr) => { exprs.push(last_expr); },
                _ => { },
            }

            Expr::TupleCons { params: exprs, l_start: 0, l_end: 0 }
        }
    })
}

pat!(parse_comma<'a> : (usize, Lexeme) => (usize, Lexeme) = x @ (_, Lexeme::Comma { .. }) => x);
pat!(parse_l_paren<'a> : (usize, Lexeme) => (usize, Lexeme) = x @ (_, Lexeme::LParen { .. }) => x);
pat!(parse_r_paren<'a> : (usize, Lexeme) => (usize, Lexeme) = x @ (_, Lexeme::RParen { .. }) => x);