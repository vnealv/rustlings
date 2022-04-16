// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)


fn add_nael(s: &mut String) -> String {
    s.push_str("NAEL");
    s.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(2, 1 + 1);
        let mut s = String::from("test");
        assert_eq!("testNAEL", add_nael(&mut s));
    }
}
