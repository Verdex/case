
use std::str::CharIndices;

use renounce::*;

use crate::data::*;

pub fn lex(input : &str) -> Result<Vec<Lexeme>, ParseError> {
    let mut input = input.char_indices();
    parser!(input => {
        ls <= * clean_lexeme;
        select ls
    })
}

fn clean_lexeme<'a>(input : &mut CharIndices<'a>) -> Result<Lexeme, ParseError> {
    parser!(input => {
        _1 <= junk;
        l <= lexeme;
        _2 <= junk;
        select l
    })
}

fn lexeme<'a>(input : &mut CharIndices<'a>) -> Result<Lexeme, ParseError> {
    alt!(input => comma; l_paren; r_paren)
}

pat!(any<'a> : (usize, char) => (usize, char) = x => x);
pat!(comma<'a> : (usize, char) => Lexeme = (index, ',') => Lexeme::Comma { index } );
pat!(l_paren<'a> : (usize, char) => Lexeme = (index, '(') => Lexeme::LParen { index } );
pat!(r_paren<'a> : (usize, char) => Lexeme = (index, ')') => Lexeme::RParen { index } );

fn digit<'a>( input : &mut CharIndices<'a> ) -> Result<(usize, char), ParseError> {
    parser!( input => {
        x <= any;
        where x.1.is_digit(10);
        select x
    })
}

fn float<'a>(input : &mut CharIndices<'a>) -> Result<Lexeme, ParseError> {
    fn sign<'a>(input : &mut CharIndices<'a>) -> Result<(usize, char), ParseError> {
        parser!(input => {
            x <= any;
            where x.1 == '+' || x.1 == '-';
            select x
        })
    }
    fn one_or_more_digits<'a>(input : &mut CharIndices<'a>) -> Result<(usize, Vec<char>), ParseError> {
        parser!(input => {
            d <= ! digit;
            ds <= * digit;
            select {
                let last = ds.last().map_or(d.0, |l| l.0);
                let mut digits = ds.into_iter().map(|x| x.1).collect::<Vec<_>>();
                digits.insert(0, d.1);
                (last, digits)
            }
        })
    }
    fn decimal<'a>(input : &mut CharIndices<'a>) -> Result<(usize, Vec<char>), ParseError> {
        parser!(input => {
            dot <= any;
            where dot.1 == '.';
            ds <= one_or_more_digits;
            select ds
        })
    }
    fn scientific_notation<'a>(input : &mut CharIndices<'a>) -> Result<(usize, Vec<char>), ParseError> {
        parser!(input => {
            e <= any;
            where e.1 == 'e' || e.1 == 'E';
            s <= ? sign;
            let s : Option<(usize, char)> = s;
            ds <= one_or_more_digits;
            select {
                let last = ds.0;
                let mut digits = ds.1;
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
        s <= ? sign;
        ds <= one_or_more_digits;
        deci <= ? decimal;
        sci <= ? scientific_notation;
        select {

        }
    })
}

fn junk<'a>(input : &mut CharIndices<'a>) -> Result<(), ParseError> {

    fn space<'a>(input : &mut CharIndices<'a>) -> Result<(), ParseError> {
        parser!( input => {
            x <= any;
            where x.1.is_whitespace();
            select ()
        })
    }

    parser!( input => {
        x <= * space;
        select ()
    })
}
