
use std::str::CharIndices;

use renounce::*;

use crate::data::*;

pub fn lex(input : &str) -> Result<Vec<Lexeme>, ParseError> {
    let mut input = input.char_indices();
    parser!(input => {
        clean_lexemes <= * lex_clean_lexeme;
        ! end;
        select clean_lexemes 
    })
}

fn lex_clean_lexeme<'a>(input : &mut CharIndices<'a>) -> Result<Lexeme, ParseError> {
    parser!(input => {
        _1_junk <= junk;
        lexeme <= lex_lexeme;
        _2_junk <= junk;
        select lexeme
    })
}

fn lex_lexeme<'a>(input : &mut CharIndices<'a>) -> Result<Lexeme, ParseError> {
    alt!(input => lex_float; lex_comma; lex_l_paren; lex_r_paren; lex_colon_symbol; lex_symbol)
}

pat!(lex_any<'a> : (usize, char) => (usize, char) = x => x);
pat!(lex_colon<'a> : (usize, char) => (usize, char) = x => x);
pat!(lex_comma<'a> : (usize, char) => Lexeme = (index, ',') => Lexeme::Comma { index } );
pat!(lex_l_paren<'a> : (usize, char) => Lexeme = (index, '(') => Lexeme::LParen { index } );
pat!(lex_r_paren<'a> : (usize, char) => Lexeme = (index, ')') => Lexeme::RParen { index } );

fn lex_digit<'a>( input : &mut CharIndices<'a> ) -> Result<(usize, char), ParseError> {
    parser!( input => {
        x <= lex_any;
        where x.1.is_digit(10);
        select x
    })
}

fn lex_colon_symbol<'a>(input : &mut CharIndices<'a>) -> Result<Lexeme, ParseError> {
    fn rest<'a>(input : &mut CharIndices<'a>) -> Result<(usize, char), ParseError> {
        parser!(input => {
            rest <= lex_any;
            let c = rest.1;
            where matches!( c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_');
            select rest
        })
    }
    parser!(input => {
        c <= lex_colon;
        first <= lex_any;
        let f = first.1;
        where matches!( f, 'a'..='z' | 'A'..='Z' | '_' );
        r <= * rest;
        select {
            let mut rest = r.iter().map(|x| x.1).collect::<Vec<char>>();
            rest.insert(0, f);
            let value = rest.iter().collect::<String>(); 
            let start = c.0;
            let end = c.0 + r.len();
            Lexeme::Symbol { value, start, end }
        }
    })
}

fn lex_symbol<'a>(input : &mut CharIndices<'a>) -> Result<Lexeme, ParseError> {
    fn rest<'a>(input : &mut CharIndices<'a>) -> Result<(usize, char), ParseError> {
        parser!(input => {
            rest <= lex_any;
            let c = rest.1;
            where matches!( c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_');
            select rest
        })
    }
    parser!(input => {
        first <= lex_any;
        let f = first.1;
        where matches!( f, 'a'..='z' | 'A'..='Z' | '_' );
        r <= * rest;
        select {
            let mut rest = r.iter().map(|x| x.1).collect::<Vec<char>>();
            rest.insert(0, f);
            let value = rest.iter().collect::<String>(); 
            let start = first.0;
            let end = first.0 + r.len();
            Lexeme::Symbol { value, start, end }
        }
    })
}

fn lex_float<'a>(input : &mut CharIndices<'a>) -> Result<Lexeme, ParseError> {
    struct Digits {
        start : usize,
        end : usize,
        digits : Vec<char>,
    }
    struct Last(usize);
    fn lex_sign<'a>(input : &mut CharIndices<'a>) -> Result<(usize, char), ParseError> {
        parser!(input => {
            x <= lex_any;
            where x.1 == '+' || x.1 == '-';
            select x
        })
    }
    fn lex_one_or_more_digits<'a>(input : &mut CharIndices<'a>) -> Result<Digits, ParseError> {
        parser!(input => {
            d <= ! lex_digit;
            ds <= * lex_digit;
            select {
                let last = ds.last().map_or(d.0, |l| l.0);
                let mut digits = ds.into_iter().map(|x| x.1).collect::<Vec<_>>();
                digits.insert(0, d.1);
                Digits { start : d.0, end : last, digits }
            }
        })
    }
    fn lex_decimal<'a>(input : &mut CharIndices<'a>) -> Result<(Last, Vec<char>), ParseError> {
        parser!(input => {
            dot <= lex_any;
            where dot.1 == '.';
            ds <= lex_one_or_more_digits;
            let cs = ds.digits; 
            let last = Last(ds.end);
            select {
                let mut cs = cs;
                cs.insert(0, '.');
                (last, cs) 
            }
        })
    }
    fn lex_scientific_notation<'a>(input : &mut CharIndices<'a>) -> Result<(Last, Vec<char>), ParseError> {
        parser!(input => {
            e <= lex_any;
            where e.1 == 'e' || e.1 == 'E';
            s <= ? lex_sign;
            let s : Option<(usize, char)> = s;
            ds <= lex_one_or_more_digits;
            select {
                let last = Last(ds.end);
                let mut digits = ds.digits;
                match s {
                    Some(s) => {
                        digits.insert(0, e.1);
                        digits.insert(1, s.1);
                        (last, digits)   
                    },
                    None => {
                        digits.insert(0, e.1);
                        (last, digits)
                    },
                }
            }
        })
    }
    parser!(input => {
        s <= ? lex_sign;
        ds <= lex_one_or_more_digits;
        deci <= ? lex_decimal;
        sci <= ? lex_scientific_notation;
        let s : Option<(usize, char)> = s;
        let deci : Option<(Last, Vec<char>)> = deci;
        let sci : Option<(Last, Vec<char>)> = sci;
        select {
            let start = if s.is_some() {
                s.as_ref().unwrap().0
            }
            else {
                ds.start
            };
            let end = if sci.is_some() {
                sci.as_ref().unwrap().0.0
            }
            else if deci.is_some() {
                deci.as_ref().unwrap().0.0
            }
            else {
                ds.end
            };
        
            let chars = vec![ s.map_or(vec![], |x| vec![x.1])
                            , ds.digits
                            , deci.map_or(vec![], |x| x.1)
                            , sci.map_or(vec![], |x| x.1)
                            ];

            let value = chars.into_iter().flatten().collect::<String>().parse::<f64>().expect("pre-parsed float failed to parse");
            Lexeme::Float { value, start, end }
        }
    })
}

fn junk<'a>(input : &mut CharIndices<'a>) -> Result<(), ParseError> {

    fn space<'a>(input : &mut CharIndices<'a>) -> Result<(), ParseError> {
        parser!( input => {
            x <= lex_any;
            where x.1.is_whitespace();
            select ()
        })
    }

    parser!( input => {
        _x <= * space;
        select ()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn float_parser_should_parse_everything() {
        let input = "+1234.1234E+12";
        let mut input = input.char_indices();
        let output = float(&mut input).unwrap();
        if let Lexeme::Float { value: output, .. } = output {
            assert_eq!( 1234.1234E+12, output );
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn float_parser_should_parse_negatives() {
        let input = "-1234.1234E-12";
        let mut input = input.char_indices();
        let output = float(&mut input).unwrap();
        if let Lexeme::Float { value: output, .. } = output {
            assert_eq!( -1234.1234E-12, output );
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn float_parser_should_parse_little_e_scientific_notation() {
        let input = "-1234.1234e-12";
        let mut input = input.char_indices();
        let output = float(&mut input).unwrap();
        if let Lexeme::Float { value: output, .. } = output {
            assert_eq!( -1234.1234E-12, output );
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn float_parser_should_parse_no_scientific_notation() {
        let input = "-1234.1234";
        let mut input = input.char_indices();
        let output = float(&mut input).unwrap();
        if let Lexeme::Float { value: output, .. } = output {
            assert_eq!( -1234.1234, output );
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn float_parser_should_parse_no_sign() {
        let input = "1234.1234";
        let mut input = input.char_indices();
        let output = float(&mut input).unwrap();
        if let Lexeme::Float { value: output, .. } = output {
            assert_eq!( 1234.1234, output );
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn float_parser_should_parse_no_decimal() {
        let input = "1234";
        let mut input = input.char_indices();
        let output = float(&mut input).unwrap();
        if let Lexeme::Float { value: output, .. } = output {
            assert_eq!( 1234.0, output );
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn float_parser_should_parse_no_decimal_with_scientific_notation() {
        let input = "1234E12";
        let mut input = input.char_indices();
        let output = float(&mut input).unwrap();
        if let Lexeme::Float { value: output, .. } = output {
            assert_eq!( 1234.0E12, output );
        }
        else {
            assert!(false);
        }
    }
}