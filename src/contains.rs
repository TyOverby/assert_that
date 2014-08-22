use std::collections::Set;
use std::collections::hashmap::HashSet;
use std::collections::TreeSet;
use std::hash::Hash;

trait Contains<A, B> {
    fn a_contains_b(p: Self) -> bool;
}

pub fn contains<A, B, T: Contains<A,B>>(args: T) -> Result<(), String> {
    if Contains::a_contains_b(args) {
        Ok(())
    } else {
        Err("".to_string())
    }
}

impl <'a, 'b> Contains<&'a String, &'b String> for (&'a String, &'b String) {
    fn a_contains_b(p: (&String, &String)) -> bool {
        let (a, b) = p;
        a.as_slice().contains(b.as_slice())
    }
}

impl <'a, 'b> Contains<&&'a str, &&'b str> for (&&'a str, &&'b str) {
    fn a_contains_b(p: (&&str, &&str)) -> bool {
        let (a, b) = p;
        a.contains(b)
    }
}

impl <'a, 'b> Contains<&'a String, &&'b str> for (&'a String, &&'b str) {
    fn a_contains_b(p: (&String, &&str)) -> bool {
        let (a, b) = p;
        a.as_slice().contains(b)
    }
}

impl <'a, 'b> Contains<&&'a str, &'b String> for (&&'a str, &'b String) {
    fn a_contains_b(p: (&&str, &String)) -> bool {
        let (a, b) = p;
        a.contains(b.as_slice())
    }
}

impl <'a, 'b> Contains<&&'a str, &'b char> for (&&'a str, &'b char) {
    fn a_contains_b(p: (&&str, &char)) -> bool {
        let (a, b) = p;
        a.contains_char(*b)
    }
}

impl <'a, 'b> Contains<&'a String, &'b char> for (&'a String, &'b char) {
    fn a_contains_b(p: (&String, &char)) -> bool {
        let (a, b) = p;
        a.as_slice().contains_char(*b)
    }
}

/*
  Apparently this doesn't work?

impl <'a, 'b, A, S: Set<A>> Contains<S, A> for (&'a S, &'a A) {
    fn a_contains_b(p: (&S, &A)) -> bool {
        let (a, b) = p;
        a.contains(b);
    }
}
*/

// But this does?
impl <'a, 'b, A: Hash + Eq > Contains<HashSet<A>, A> for (&'a HashSet<A>, &'a A) {
    fn a_contains_b(p: (&HashSet<A>, &A)) -> bool {
        let (a, b) = p;
        a.contains(b)
    }
}

// But this does?
impl <'a, 'b, A: Ord> Contains<TreeSet<A>, A> for (&'a TreeSet<A>, &'a A) {
    fn a_contains_b(p: (&TreeSet<A>, &A)) -> bool {
        let (a, b) = p;
        a.contains(b)
    }
}

impl <'a, 'b, A: PartialEq> Contains<Vec<A>, A> for (&'a Vec<A>, &'a A) {
    fn a_contains_b(p: (&Vec<A>, &A)) -> bool {
        let (a, b) = p;
        a.contains(b)
    }
}

#[cfg(test)]
mod tests {
    use super::super::assert_that;
    use super::contains;
    #[test]
    fn test_str_str() {
        assert_that(&"foo".to_string(), contains, &"oo".to_string());
        assert_that("foo", contains, "oo");
    }
}
