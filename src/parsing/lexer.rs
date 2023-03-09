
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
pat!(lex_colon<'a> : (usize, char) => (usize, char) = (i, ':') => (i, ':'));
pat!(lex_comma<'a> : (usize, char) => Lexeme = (index, ',') => Lexeme::Comma { index } );
pat!(lex_l_paren<'a> : (usize, char) => Lexeme = (index, '(') => Lexeme::LParen { index } );
pat!(lex_r_paren<'a> : (usize, char) => Lexeme = (index, ')') => Lexeme::RParen { index } );

fn lex_digit<'a>( input : &mut CharIndices<'a> ) -> Result<(usize, char), ParseError> {
    parser!( input => {
        digit_candidate <= lex_any;
        where digit_candidate.1.is_digit(10);
        select digit_candidate 
    })
}

fn lex_colon_symbol<'a>(input : &mut CharIndices<'a>) -> Result<Lexeme, ParseError> {
    fn lex_rest<'a>(input : &mut CharIndices<'a>) -> Result<(usize, char), ParseError> {
        parser!(input => {
            secondary_colon_symbol_char_candidate <= lex_any;
            let c = secondary_colon_symbol_char_candidate.1;
            where matches!( c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_');
            select secondary_colon_symbol_char_candidate 
        })
    }
    parser!(input => {
        colon <= lex_colon;
        init_colon_symbol_char_candidate <= lex_any;
        let init = init_colon_symbol_char_candidate.1;
        where matches!( init, 'a'..='z' | 'A'..='Z' | '_' );
        secondary_colon_symbol_chars <= * lex_rest;
        select {
            let mut rest = secondary_colon_symbol_chars.iter().map(|x| x.1).collect::<Vec<char>>();
            rest.insert(0, init);
            let value = rest.iter().collect::<String>(); 
            let start = colon.0;
            let end = colon.0 + secondary_colon_symbol_chars.len();
            Lexeme::ColonSymbol { value, start, end }
        }
    })
}

fn lex_symbol<'a>(input : &mut CharIndices<'a>) -> Result<Lexeme, ParseError> {
    fn lex_rest<'a>(input : &mut CharIndices<'a>) -> Result<(usize, char), ParseError> {
        parser!(input => {
            secondary_symbol_char_candidate <= lex_any;
            let c = secondary_symbol_char_candidate.1;
            where matches!( c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_');
            select secondary_symbol_char_candidate 
        })
    }
    parser!(input => {
        init_symbol_char_candidate <= lex_any;
        let f = init_symbol_char_candidate.1;
        where matches!( f, 'a'..='z' | 'A'..='Z' | '_' );
        secondary_symbol_chars <= * lex_rest;
        select {
            let mut rest = secondary_symbol_chars.iter().map(|x| x.1).collect::<Vec<char>>();
            rest.insert(0, f);
            let value = rest.iter().collect::<String>(); 
            let start = init_symbol_char_candidate.0;
            let end = init_symbol_char_candidate.0 + secondary_symbol_chars.len();
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
            sign_candidate <= lex_any;
            where sign_candidate.1 == '+' || sign_candidate.1 == '-';
            select sign_candidate 
        })
    }
    fn lex_one_or_more_digits<'a>(input : &mut CharIndices<'a>) -> Result<Digits, ParseError> {
        parser!(input => {
            digit <= lex_digit;
            digits <= * lex_digit;
            select {
                let last = digits.last().map_or(digit.0, |l| l.0);
                let mut digits = digits.into_iter().map(|x| x.1).collect::<Vec<_>>();
                digits.insert(0, digit.1);
                Digits { start : digit.0, end : last, digits }
            }
        })
    }
    fn lex_decimal<'a>(input : &mut CharIndices<'a>) -> Result<(Last, Vec<char>), ParseError> {
        parser!(input => {
            decimal_dot_candidate <= lex_any;
            where decimal_dot_candidate.1 == '.';
            decimal_digits <= lex_one_or_more_digits;
            let cs = decimal_digits.digits; 
            let last = Last(decimal_digits.end);
            select {
                let mut cs = cs;
                cs.insert(0, '.');
                (last, cs) 
            }
        })
    }
    fn lex_scientific_notation<'a>(input : &mut CharIndices<'a>) -> Result<(Last, Vec<char>), ParseError> {
        parser!(input => {
            e_candidate <= lex_any;
            where e_candidate.1 == 'e' || e_candidate.1 == 'E';
            scientific_notation_sign <= ? lex_sign;
            let scientific_notation_sign : Option<(usize, char)> = scientific_notation_sign;
            scientific_notation_digits <= lex_one_or_more_digits;
            select {
                let last = Last(scientific_notation_digits.end);
                let mut digits = scientific_notation_digits.digits;
                match scientific_notation_sign {
                    Some(s) => {
                        digits.insert(0, e_candidate.1);
                        digits.insert(1, s.1);
                        (last, digits)   
                    },
                    None => {
                        digits.insert(0, e_candidate.1);
                        (last, digits)
                    },
                }
            }
        })
    }
    parser!(input => {
        float_sign <= ? lex_sign;
        float_digits <= lex_one_or_more_digits;
        decimal <= ? lex_decimal;
        scientific_notation <= ? lex_scientific_notation;
        let float_sign : Option<(usize, char)> = float_sign;
        let decimal : Option<(Last, Vec<char>)> = decimal;
        let scientific_notation : Option<(Last, Vec<char>)> = scientific_notation;
        select {
            let start = if float_sign.is_some() {
                float_sign.as_ref().unwrap().0
            }
            else {
                float_digits.start
            };
            let end = if scientific_notation.is_some() {
                scientific_notation.as_ref().unwrap().0.0
            }
            else if decimal.is_some() {
                decimal.as_ref().unwrap().0.0
            }
            else {
                float_digits.end
            };
        
            let chars = vec![ float_sign.map_or(vec![], |x| vec![x.1])
                            , float_digits.digits
                            , decimal.map_or(vec![], |x| x.1)
                            , scientific_notation.map_or(vec![], |x| x.1)
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
        let output = lex_float(&mut input).unwrap();
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
        let output = lex_float(&mut input).unwrap();
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
        let output = lex_float(&mut input).unwrap();
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
        let output = lex_float(&mut input).unwrap();
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
        let output = lex_float(&mut input).unwrap();
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
        let output = lex_float(&mut input).unwrap();
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
        let output = lex_float(&mut input).unwrap();
        if let Lexeme::Float { value: output, .. } = output {
            assert_eq!( 1234.0E12, output );
        }
        else {
            assert!(false);
        }
    }
}