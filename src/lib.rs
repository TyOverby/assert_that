pub mod contains;

pub fn assert_that<'a,'b, A, B>(subject: &'a A, property: fn((&'a A, &'b B)) -> Result<(), String>, arg: &'b B) {
    match property((subject, arg)) {
        Ok(()) => { }
        Err(message) => {
            fail!("{}", message);
        }
    }
}
