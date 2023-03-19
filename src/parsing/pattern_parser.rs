
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

macro_rules! p {
    ($vis:vis $name:ident $input:ident $expr_parser:ident $b:block) => { 
        $vis fn $name<'a, T>($input : &mut T, $expr_parser : ExprParser<T>) -> Result<Pat, ParseError> 
            where T : Iterator<Item = (usize, &'a Lexeme)> + Clone {
            
            $b
        }
    }
}

p!( pub parse_pattern input expr_parser {

    alt!( input => parse_unbound_variable; parse_float; parse_symbol; |x| parse_tuple(x, expr_parser) )
});


pat!(parse_float<'a> : (usize, &'a Lexeme) => Pat = 
    (i, Lexeme::Float { value, .. }) => Pat::Float { value: *value, l_start: i, l_end: i });
pat!(parse_symbol<'a> : (usize, &'a Lexeme) => Pat = 
    (i, Lexeme::ColonSymbol{ value, .. }) => Pat::Symbol { value: value.to_string(), l_start: i, l_end: i });

pat!(parse_comma<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::Comma { .. }) => x);
pat!(parse_l_paren<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::LParen { .. }) => x);
pat!(parse_r_paren<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::RParen { .. }) => x);

pat!(parse_unbound_variable<'a> : (usize, &'a Lexeme) => Pat = 
    (i, Lexeme::Symbol{ value, .. }) => Pat::UnboundVariable { value: value.to_string(), l_start: i, l_end: i });

p!( parse_tuple input expr_parser {
    p!( parse_pat_comma input expr_parser {
        // TODO put start and stop data in return type?
        parser!(input => {
            pat <= |x| parse_pattern(x, expr_parser);
            comma <= parse_comma;
            select pat 
        })
    });

    parser!(input => {
        l_paren <= parse_l_paren;
        pats <= * |x| parse_pat_comma(x, expr_parser);
        maybe_last_pat <= ? |x| parse_pattern(x, expr_parser);
        r_paren <= parse_r_paren;
        select {
            let mut pats = pats;
            match maybe_last_pat {
                Some(last_pat) => { pats.push(last_pat); },
                _ => { },
            }

            Pat::Tuple { params: pats, l_start: l_paren.0, l_end: r_paren.0 }
        }
    })
});

#[cfg(test)]
mod test {
    use intra::*;

    use super::*;
    use super::super::lexer::lex;

    fn slice<'a, T>(input : &'a Vec<T>) -> &'a [T] { &input[..] }

    fn stub_parse_expr<'a>(input : &mut (impl Iterator<Item = (usize, &'a Lexeme)> + Clone)) -> Result<Expr, ParseError> {
        Ok(Expr::Symbol{ value: "symbol".to_string(), l_start: 0, l_end: 0 })
    }

    #[test]
    fn parse_pattern_should_parse_tuple() {
        let input = "(1, 2, 3)";
        let mut input = input.char_indices();
        let ls = lex(&mut input).unwrap();
        let mut ls = ls.iter().enumerate();
        let output = parse_pattern(&mut ls, stub_parse_expr).unwrap();

        let mut matched = false;
        atom!( output => [Pat::Tuple { ref params, .. }] params
                       ; slice $ [[ Pat::Float { value: 1.0, .. }
                                  , Pat::Float { value: 2.0, .. }
                                  , Pat::Float { value: 3.0, .. } 
                                 ]]  
                      => { matched = true; } );
        assert!(matched);
    }

}