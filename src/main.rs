extern crate num;

mod ast;
mod grammar;
mod vm;
mod test_helpers;

fn main() {
    match grammar::statements(&"let a = 12345") {
        Ok(v) => {
            println!("got value {:?}", v);
        }
        Err(err) => panic!("parse error: {:?}", err),
    }
}
