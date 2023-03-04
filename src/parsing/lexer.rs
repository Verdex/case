
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

pat!(comma<'a> : (usize, char) => Lexeme = (index, ',') => Lexeme::Comma { index } );
pat!(l_paren<'a> : (usize, char) => Lexeme = (index, '(') => Lexeme::LParen { index } );
pat!(r_paren<'a> : (usize, char) => Lexeme = (index, ')') => Lexeme::RParen { index } );

fn junk<'a>(input : &mut CharIndices<'a>) -> Result<(), ParseError> {

    fn space<'a>(input : &mut CharIndices<'a>) -> Result<(), ParseError> {
        pat!(any : (usize, char) => char = (_, x) => x);

        parser!( input => {
            x <= any;
            where x.is_whitespace();
            select ()
        })
    }

    parser!( input => {
        x <= * space;
        select ()
    })
}
