
use renounce::*;
use crate::data::*;
use super::pattern_parser::*;

macro_rules! input {
    ($life:lifetime) => { &mut (impl Iterator<Item = (usize, &$life Lexeme)> + Clone) }
}


pub fn parse<'a>(input : input!('a)) -> Result<Vec<DefOrExpr>, ParseError> {
    fn w_parse_expr<'a>(input : input!('a)) -> Result<DefOrExpr, ParseError> {
        parser!(input => {
            expr <= parse_expr;
            select DefOrExpr::Expr(expr)
        })
    }
    fn w_parse_fn_def<'a>(input : input!('a)) -> Result<DefOrExpr, ParseError> {
        parser!(input => {
            fn_def <= parse_fn_def;
            select DefOrExpr::FnDef(fn_def)
        })
    }

    fn parse_def_or_expr<'a>(input : input!('a)) -> Result<DefOrExpr, ParseError> {
        alt!(input => w_parse_fn_def; w_parse_expr)
    }

    parser!(input => {
        defs_and_exprs <= * parse_def_or_expr;
        select defs_and_exprs
    })
}

fn parse_fn_def<'a>(input : input!('a)) -> Result<FnDef, ParseError> {
    fn parse_ident_comma<'a>(input : input!('a)) -> Result<(usize, String), ParseError> {
        // TODO put start and stop data in return type?
        parser!(input => {
            ident <= parse_ident;
            comma <= parse_comma;
            select ident 
        })
    }

    parser!(input => {
        fn_sym <= parse_ident;
        where fn_sym.1 == "fn";
        name <= parse_ident;
        _l_paren <= parse_l_paren;
        params <= * parse_ident_comma;
        maybe_last_param <= ? parse_ident;
        _r_parent <= parse_r_paren;
        expr <= parse_expr;
        semi <= parse_semi_colon;
        select {
            let mut params = params;
            match maybe_last_param {
                Some(last_param) => { params.push(last_param); },
                _ => { },
            }
            // TODO start and end
            FnDef { name: name.1.to_string()
                  , params: params.into_iter().map(|x| x.1).collect()
                  , body: expr
                  , l_start: 0
                  , l_end: 0
                  }
        }
    })
}

fn parse_expr<'a>(input : input!('a)) -> Result<Expr, ParseError> {
    fn parse_leading_expr<'a>(input : input!('a)) -> Result<Expr, ParseError> {
        alt!( input => parse_float
                    ; parse_symbol
                    ; parse_tuple_cons 
                    ; parse_var
                    )
    }
    enum Follower {
        ParamList(Vec<Expr>)
    }
    fn parse_params_list<'a>(input : input!('a)) -> Result<Follower, ParseError> {
        parser!(input => {
            _l_paren <= parse_l_paren;
            exprs <= parse_expr_list;
            _r_paren <= parse_r_paren;
            select Follower::ParamList(exprs)
        })
    }
    fn parse_follower<'a>(input : input!('a)) -> Result<Follower, ParseError> {
        alt!(input => parse_params_list)
    }
    parser!(input => {
        fn_expr <= parse_leading_expr;
        followers <= * parse_follower;
        select {
            followers.into_iter().fold(fn_expr, |prev, f| match f {
                // TODO start and end
                Follower::ParamList(params) => Expr::Call { fn_expr: Box::new(prev), params, l_start: 0, l_end: 0 },
            })
        }
    })
}

pat!(parse_float<'a> : (usize, &'a Lexeme) => Expr = 
    (i, Lexeme::Float { value, .. }) => Expr::Float { value: *value, l_start: i, l_end: i });
pat!(parse_symbol<'a> : (usize, &'a Lexeme) => Expr = 
    (i, Lexeme::ColonSymbol{ value, .. }) => Expr::Symbol { value: value.to_string(), l_start: i, l_end: i });
pat!(parse_ident<'a> : (usize, &'a Lexeme) => (usize, String) = 
    (i, Lexeme::Symbol{ value, .. }) => (i, value.to_string()));
pat!(parse_var<'a> : (usize, &'a Lexeme) => Expr = 
    (i, Lexeme::Symbol{ value, .. }) => Expr::Var { value: value.to_string(), l_start: i, l_end: i });
    
// TODO start and stop data
fn parse_expr_list<'a>(input : input!('a)) -> Result<Vec<Expr>, ParseError> {
    fn parse_expr_comma<'a>(input : input!('a)) -> Result<Expr, ParseError> {
        // TODO put start and stop data in return type?
        parser!(input => {
            expr <= parse_expr;
            comma <= parse_comma;
            select expr
        })
    }

    parser!(input => {
        exprs <= * parse_expr_comma;
        maybe_last_expr <= ? parse_expr;
        select {
            let mut exprs = exprs;
            match maybe_last_expr {
                Some(last_expr) => { exprs.push(last_expr); },
                _ => { },
            }
            exprs
        }
    })
}

fn parse_tuple_cons<'a>(input : input!('a)) -> Result<Expr, ParseError> {

    parser!(input => {
        l_paren <= parse_l_paren;
        exprs <= parse_expr_list;
        r_paren <= parse_r_paren;
        select {

            // TODO start and end
            Expr::TupleCons { params: exprs, l_start: l_paren.0, l_end: r_paren.0 }
        }
    })
}

pat!(parse_comma<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::Comma { .. }) => x);
pat!(parse_l_paren<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::LParen { .. }) => x);
pat!(parse_r_paren<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::RParen { .. }) => x);
pat!(parse_semi_colon<'a> : (usize, &'a Lexeme) => (usize, &'a Lexeme) = x @ (_, Lexeme::SemiColon { .. }) => x);


#[cfg(test)]
mod test { 
    use intra::*;

    use super::*;
    use super::super::lexer::lex;

    fn slice<'a, T>(input : &'a Vec<T>) -> &'a [T] { &input[..] }
    fn unbox<'a, T>(input : &'a Box<T> ) -> &'a T { &**input }

    #[test]
    fn parse_should_call_following_call() {
        let input = "x(1, 2, 3)(4, 5, 6)";
        let mut input = input.char_indices();
        let ls = lex(&mut input).unwrap();
        let mut ls = ls.iter().enumerate();
        let output = parse(&mut ls).unwrap();

        let mut matched = false;
        atom!( output => [ref x] x
                       ; slice $ [[ DefOrExpr::Expr(expr) ]] expr 
                       ; [ Expr::Call { fn_expr: outer_fn_expr, params: outer_params, .. } ] outer_fn_expr
                       ; unbox $ [ Expr::Call { fn_expr: inner_fn_expr, params: inner_params, .. } ] inner_fn_expr
                       ; unbox $ [ Expr::Var { value, .. } ]
                       => { 
                        assert_eq!(value, "x");
                        assert_eq!(inner_params.len(), 3);
                        assert!( matches!(inner_params[0], Expr::Float{ value: 1.0, .. }) );
                        assert!( matches!(inner_params[1], Expr::Float{ value: 2.0, .. }) );
                        assert!( matches!(inner_params[2], Expr::Float{ value: 3.0, .. }) );
                        assert_eq!(outer_params.len(), 3);
                        assert!( matches!(outer_params[0], Expr::Float{ value: 4.0, .. }) );
                        assert!( matches!(outer_params[1], Expr::Float{ value: 5.0, .. }) );
                        assert!( matches!(outer_params[2], Expr::Float{ value: 6.0, .. }) );

                        matched = true; 
                    } );

        assert!(matched);
    }

    #[test]
    fn parse_should_parse_call_with_inner_calls() {
        let input = "x(y(), 2, 3)";
        let mut input = input.char_indices();
        let ls = lex(&mut input).unwrap();
        let mut ls = ls.iter().enumerate();
        let output = parse(&mut ls).unwrap();

        let mut matched = false;
        atom!( output => [ref x] x
                       ; slice $ [[ DefOrExpr::Expr(expr) ]] expr 
                       ; [ Expr::Call { fn_expr, params, .. } ] fn_expr
                       ; unbox $ [ Expr::Var { value, .. } ]
                       => { 
                        assert_eq!(value, "x");
                        assert_eq!(params.len(), 3);

                        assert!( matches!(params[1], Expr::Float{ value: 2.0, .. }) );
                        assert!( matches!(params[2], Expr::Float{ value: 3.0, .. }) );

                        let inner = &params[0];

                        atom!(inner => [ Expr::Call { fn_expr, params, ..} ] fn_expr 
                                          ; unbox $ [ Expr::Var { value, .. } ]
                                         => {
                            assert_eq!( value, "y" );
                            assert_eq!( params.len(), 0 );

                            matched = true;
                        } );
                    } );

        assert!(matched);
    }
    
    #[test]
    fn parse_should_parse_call() {
        let input = "x(1, 2, 3)";
        let mut input = input.char_indices();
        let ls = lex(&mut input).unwrap();
        let mut ls = ls.iter().enumerate();
        let output = parse(&mut ls).unwrap();

        let mut matched = false;
        atom!( output => [ref x] x
                       ; slice $ [[ DefOrExpr::Expr(expr) ]] expr 
                       ; [ Expr::Call { fn_expr, params, .. } ] fn_expr
                       ; unbox $ [ Expr::Var { value, .. } ]
                       => { 
                        assert_eq!(value, "x");
                        assert_eq!(params.len(), 3);
                        assert!( matches!(params[0], Expr::Float{ value: 1.0, .. }) );
                        assert!( matches!(params[1], Expr::Float{ value: 2.0, .. }) );
                        assert!( matches!(params[2], Expr::Float{ value: 3.0, .. }) );

                        matched = true; 
                    } );

        assert!(matched);
    }

    #[test]
    fn parse_should_parse_fn_def() {
        let input = "fn name(a, b, c) 1.0;";
        let mut input = input.char_indices();
        let ls = lex(&mut input).unwrap();
        let mut ls = ls.iter().enumerate();
        let output = parse(&mut ls).unwrap();

        let mut matched = false;
        atom!( output => [ref x] x
                       ; slice $ [[ DefOrExpr::FnDef(def) ]] def
                       ; [ FnDef { name, params, body, .. } ] body
                       ; [ Expr::Float { value: 1.0, .. } ]
                       => { 
                        assert_eq!(name, "name");
                        assert_eq!(params[0], "a");
                        assert_eq!(params[1], "b");
                        assert_eq!(params[2], "c");

                        matched = true; 
                    } );

        assert!(matched);
    }

    #[test]
    fn parse_should_parse_float() {
        let input = "1.0";
        let mut input = input.char_indices();
        let ls = lex(&mut input).unwrap();
        let mut ls = ls.iter().enumerate();
        let output = parse(&mut ls).unwrap();

        let mut matched = false;
        atom!( output => [ref x] x
                       ; slice $ [[ DefOrExpr::Expr(e) ]] e
                       ; [ Expr::Float { value: 1.0, .. } ] 
                       => { matched = true; } );

        assert!(matched);
    }

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