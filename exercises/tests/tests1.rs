// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests1` for hints :)



fn check_me() -> bool {
    return 1 == 1;
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn you_can_assert() {
        assert!(1 == 1, "MATCHING raw condition {}", 1+1);
        assert!(check_me(), "MATCHING function call {}", "check_me");
    }
}
