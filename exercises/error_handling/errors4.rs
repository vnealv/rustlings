// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
    Unknown,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            value if value > 0  => return Ok(PositiveNonzeroInteger(value as u64)),
            value if value == 0 => return Err(CreationError::Zero),
            value if value < 0  => return Err(CreationError::Negative),
            _                   => return Err(CreationError::Unknown),
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
