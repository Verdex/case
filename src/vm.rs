
use crate::data::*;

pub fn execute(mut il : Vec<Il>) {
    let mut stack : Vec<IlData> = vec![];

    while il.len() > 0 {
        let i = il.pop().unwrap();

        match i {
            Il::Print => { 
                let data = stack.pop().expect("TODO:  handle calling print without anything on the stack");
                println!("{:?}", data);
            },
            _ => panic!("!"),
        }
    }
}