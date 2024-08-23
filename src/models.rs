pub struct Repeater(i8);
impl TryFrom<i8> for Repeater {
    type Error = &'static str;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(
                "The provided value must comply to the following invariant: 1 <= value <= 127",
            );
        }
        return Ok(Self(value));
    }
}

impl From<Repeater> for i8 {
    fn from(repeater: Repeater) -> Self {
        return repeater.0;
    }
}

#[derive(PartialEq)]
pub enum AlternateDirection { Yes, No }

impl From<bool> for AlternateDirection {
    fn from(value: bool) -> Self {
        if value { return Self::Yes; }
        return Self::No;
    }
}


mod tests {
    #[test]
    fn creating_a_repeater_with_0_value_returns_error() {
        let repeater_result: Result<super::Repeater, _> = 0.try_into();
        assert!(repeater_result.is_err());
        let error = repeater_result.err().unwrap();
        assert_eq!(
            "The provided value must comply to the following invariant: 1 <= value <= 127",
            error
        );
    }
}
