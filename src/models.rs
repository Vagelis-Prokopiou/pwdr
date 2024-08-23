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
    fn from(r: Repeater) -> Self {
        return r.0;
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
