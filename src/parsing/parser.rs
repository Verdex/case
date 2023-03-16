
use renounce::*;

use crate::data::*;

macro_rules! input {
    ($life:lifetime) => { &mut (impl Iterator<Item = (usize, &$life Lexeme)> + Clone) }
}


pub fn parse<'a>(input : input!('a)) -> Result<Vec<DefOrExpr>, ParseError> {
    Ok(vec![])
}


fn parse_expr<'a>(input : input!('a)) -> Result<Expr, ParseError> {
    alt!( input => parse_float; parse_symbol; parse_tuple_cons )
}

pat!(parse_float<'a> : (usize, &'a Lexeme) => Expr = 
    (i, Lexeme::Float { value, .. }) => Expr::Float { value: *value, l_start: i, l_end: i });
pat!(parse_symbol<'a> : (usize, &'a Lexeme) => Expr = 
    (i, Lexeme::ColonSymbol{ value, .. }) => Expr::Symbol { value: value.to_string(), l_start: i, l_end: i });

fn parse_tuple_cons<'a>(input : input!('a)) -> Result<Expr, ParseError> {
    fn parse_expr_comma<'a>(input : input!('a)) -> Result<Expr, ParseError> {
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

pat!(parse_comma<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::Comma { .. }) => x);
pat!(parse_l_paren<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::LParen { .. }) => x);
pat!(parse_r_paren<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::RParen { .. }) => x);

#[cfg(test)]
mod test { 
    use intra::*;

    use super::*;
    use super::super::lexer::lex;

    fn slice<'a, T>(input : &'a Vec<T>) -> &'a [T] { &input[..] }

    #[test]
    fn parse_tuple_cons_should_parse() {
        let input = "(1, 2, 3)";
        let mut input = input.char_indices();
        let ls = lex(&mut input).unwrap();
        let mut ls = ls.iter().enumerate();
        let output = parse_tuple_cons(&mut ls).unwrap();

        let mut matched = false;
        atom!( output => [Expr::TupleCons { ref params, .. }] params
                       ; slice $ [[ Expr::Float { value: 1.0, .. }
                                  , Expr::Float { value: 2.0, .. }
                                  , Expr::Float { value: 3.0, .. } 
                                 ]]  
                      => { matched = true; } );
        assert!(matched);
    }
}