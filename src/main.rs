
mod data;
mod parsing;
mod compiling;
mod vm;

use crate::data::*;

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
        let lexemes = parsing::lexer::lex(&mut input).unwrap();
        let mut lexemes = lexemes.iter().enumerate();
        let ast = parsing::parser::parse(&mut lexemes).unwrap();
        let mut il = compiling::compiler::compile(ast).unwrap();
        // TODO nope
        il.push(Il::Exit);
        let exe_env = vm::execute(il);

        println!("exe_env:");
        // TODO this could be better
        println!("data stack:\n {:?}", exe_env.data_stack);
        println!("def stack:\n {:?}", exe_env.def_stack);
    } 
}
