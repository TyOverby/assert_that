use std::collections::Collection;

trait HasLength {
    fn a_has_length_b(p: Self) -> bool;
}


pub fn has_length<T: HasLength>(args: T) -> Result<(), String> {
    if HasLength::a_has_length_b(args) {
        Ok(())
    } else {
        Err("".to_string())
    }
}

impl <'a, T: Collection> HasLength for (&'a T, uint) {
    fn a_has_length_b(p: (&T, uint)) -> bool {
        let (c, l) = p;
        c.len() == l
    }
}

#[cfg(test)]
mod test {
    use super::has_length;
    use super::super::assert_that;

    #[test]
    fn test_vec() {
        assert_that(&vec![1u,3,4], has_length, 3u);
    }
}
