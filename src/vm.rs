
use std::collections::HashMap;

use crate::data::*;

pub fn execute(mut il : Vec<Il>) {
    il.reverse();
    let mut data_stack : Vec<IlData> = vec![];

    while il.len() > 0 {
        let i = il.pop().unwrap();

        match i {
            Il::Print => { 
                let data = data_stack.pop().expect("TODO:  handle calling print without anything on the data stack");
                println!("{:?}", data);
            },
            Il::Push(data) => {
                data_stack.push(data);
            },
            Il::TupleCons(count) => {
                let params = data_stack.drain((data_stack.len() - count)..).collect::<Vec<_>>();
                data_stack.push(IlData::Tuple(params));
            },
            Il::Def => {
                panic!("todo")
            },
            _ => panic!("!"),
        }
    }
}