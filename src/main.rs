
mod data;
mod parsing;

fn main() {
    use std::io::{stdout, stdin, Write};

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // TODO: renounce leaves the iterator mostly alone on a fatal error, so you can do input.next to
        // see what the cause of the fatal result was in addition to looking at the parse error contents

        let mut input = input.char_indices();
        let x = parsing::lexer::lex(&mut input).unwrap();
        println!("{:?}", x);
    } 
}
