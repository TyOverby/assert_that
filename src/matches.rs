use regex::Regex;

trait Matches<A, B> {
    fn a_matches_b(p: Self) -> bool;
}

pub fn matches<A, B, T: Matches<A,B>>(args: T) -> Result<(), String> {
    if Matches::a_matches_b(args) {
        Ok(())
    } else {
        Err("".to_string())
    }
}

impl <'a, 'b> Matches<&'a String, &'b String> for (&'a String, &'b String) {
    fn a_matches_b(p: (&String, &String)) -> bool {
        let (a, b) = p;
        let regex = Regex::new(b.as_slice()).unwrap();
        regex.is_match(a.as_slice())
    }
}

impl <'a, 'b> Matches<&'a str, &'b str> for (&'a str, &'b str) {
    fn a_matches_b(p: (&str, &str)) -> bool {
        let (a, b) = p;
        let regex = Regex::new(b).unwrap();
        regex.is_match(a)
    }
}

impl <'a, 'b> Matches<&'a String, &'b str> for (&'a String, &'b str) {
    fn a_matches_b(p: (&String, &str)) -> bool {
        let (a, b) = p;
        let regex = Regex::new(b).unwrap();
        regex.is_match(a.as_slice())
    }
}

impl <'a, 'b> Matches<&'a str, &'b String> for (&'a str, &'b String) {
    fn a_matches_b(p: (&str, &String)) -> bool {
        let (a, b) = p;
        let regex = Regex::new(b.as_slice()).unwrap();
        regex.is_match(a)
    }
}

#[cfg(test)]
mod test {
    use super::super::assert_that;
    use super::matches;

    #[test]
    fn test_string_string() {
        assert_that(&"123".to_string(), matches, "[1-9]{3}");
    }
    #[test]
    fn test_str_str() {
        assert_that("123", matches, "[1-9]{3}");
    }
    #[test]
    fn test_str_string() {
        assert_that("123", matches, &"[1-9]{3}".to_string());
    }
    #[test]
    fn test_string_str() {
        assert_that(&"123".to_string(), matches, "[1-9]{3}");
    }
}
