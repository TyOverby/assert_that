extern crate regex;

pub mod contains;
pub mod matches;
pub mod len;

// TODO: change this to not use procs when owned closures come out.
pub fn assert_that<A, B>(subject: A, property: proc((A, B)) -> Result<(), String>, arg: B) {
    match property((subject, arg)) {
        Ok(()) => { }
        Err(message) => {
            fail!("{}", message);
        }
    }
}
/*
pub fn not<A, B>(property: proc((A, B)):'static -> Result<(), String>) -> proc((A, B)) -> Result<(), String> {
    proc(val: (A, B)) -> Result<(), String> {
        match property(val) {
            Ok(()) => { Err("".to_string()) },
            Err(message) => { Ok(()) }
        }
    }
}
*/
