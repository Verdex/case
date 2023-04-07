
use std::collections::HashMap;

use crate::data::*;

pub fn execute(mut il : Vec<Il>) {
    il.reverse();
    let mut stack : Vec<IlData> = vec![];

    while il.len() > 0 {
        let i = il.pop().unwrap();

        match i {
            Il::Print => { 
                let data = stack.pop().expect("TODO:  handle calling print without anything on the stack");
                println!("{:?}", data);
            },
            Il::Push(data) => {
                stack.push(data);
            },
            Il::TupleCons(count) => {
                let params = stack.drain((stack.len() - count)..).collect::<Vec<_>>();
                stack.push(IlData::Tuple(params));
            },
            Il::Def => {
                panic!("todo")
            },
            _ => panic!("!"),
        }
    }
}