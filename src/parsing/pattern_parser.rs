
use renounce::*;

use crate::data::*;

/*
    variable, cons, list, object, object-list, call pattern, next pattern, or pattern, if pattern, and pattern, xor pattern
    at pattern, tuple pattern

    * x
    * (a, b)
    * [], [x], [x, y, ...]
    * {[x; y; z]}
    * {[* x; ? y;]}  // ?
    * x(y, z, w) 
    * {[x]}
    * { x; y; z; }
    * { !; x(y, !, w); }
    * { w(a, (x @ !).and(a:b).and(c:d) ) }  // b and d are some patterns with some known list of anon-struct return value, which is dropped into a and c
    * $x(a) // call pattern 'x' and match against the returned value


*/

type ExprParser<T> = fn(&mut T) -> Result<Expr, ParseError>;

pub fn parse_pattern<'a, T>(input : &mut T, expr_parser : ExprParser<T>) -> Result<Pat, ParseError> 
    where T : Iterator<Item = (usize, &'a Lexeme)> + Clone {

    alt!( input => parse_unbound_variable )
}

pat!(parse_float<'a> : (usize, &'a Lexeme) => Pat = 
    (i, Lexeme::Float { value, .. }) => Pat::Float { value: *value, l_start: i, l_end: i });
pat!(parse_symbol<'a> : (usize, &'a Lexeme) => Pat = 
    (i, Lexeme::ColonSymbol{ value, .. }) => Pat::Symbol { value: value.to_string(), l_start: i, l_end: i });

pat!(parse_comma<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::Comma { .. }) => x);
pat!(parse_l_paren<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::LParen { .. }) => x);
pat!(parse_r_paren<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::RParen { .. }) => x);

pat!(parse_unbound_variable<'a> : (usize, &'a Lexeme) => Pat = 
    (i, Lexeme::Symbol{ value, .. }) => Pat::UnboundVariable { value: value.to_string(), l_start: i, l_end: i });