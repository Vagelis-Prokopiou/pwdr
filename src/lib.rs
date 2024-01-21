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

pub fn generate_password(password: &str, repeater: Repeater, alternate_direction: bool) -> String {
    let repeat: i8 = repeater.into();
    if repeat == 1 {
        return password.to_string();
    }

    let mut _p = String::new();
    _p.push_str(&password);

    if alternate_direction {
        let mut alternate_password_vector: Vec<_> = password.chars().collect();
        alternate_password_vector.reverse();
        let alternate_password: String = alternate_password_vector.into_iter().collect();
        for i in 1..repeat {
            if i % 2 == 0 {
                _p.push_str(&password);
            } else {
                _p.push_str(&alternate_password);
            }
        }
        return _p;
    }

    for _ in 1..repeat {
        _p.push_str(&password);
    }

    return _p;
}

#[cfg(test)]
mod tests {
    use crate::{generate_password, Repeater};

    #[test]
    fn creating_a_repeater_with_0_value_returns_error() {
        let repeater_result: Result<Repeater, _> = 0.try_into();
        assert!(repeater_result.is_err());
        let error = repeater_result.err().unwrap();
        assert_eq!(
            "The provided value must comply to the following invariant: 1 <= value <= 127",
            error
        );
    }

    #[test]
    fn generate_password_correctly_repeats_with_direction_alternation() {
        let password = "aBcdef123";
        let alternate_password = "321fedcBa";
        assert_eq!(
            format!("{}{}", password, alternate_password),
            generate_password(&password, 2.try_into().unwrap(), true)
        );
        assert_eq!(
            format!("{}{}{}", password, alternate_password, password),
            generate_password(&password, 3.try_into().unwrap(), true)
        );
        assert_eq!(
            format!(
                "{}{}{}{}",
                password, alternate_password, password, alternate_password
            ),
            generate_password(&password, 4.try_into().unwrap(), true)
        );
    }

    #[test]
    fn generate_password_correctly_repeats_without_direction_alternation() {
        let password = "aBcdef123";
        assert_eq!(
            format!("{}{}", password, password),
            generate_password(&password, 2.try_into().unwrap(), false)
        );
        assert_eq!(
            format!("{}{}{}", password, password, password),
            generate_password(&password, 3.try_into().unwrap(), false)
        );
        assert_eq!(
            format!("{}{}{}{}", password, password, password, password),
            generate_password(&password, 4.try_into().unwrap(), false)
        );
    }

    #[test]
    fn generate_password_returns_initial_pasword_if_repeat_is_one_regardles_of_direction_alternation(
    ) {
        let password = "aBcdef123";
        assert_eq!(
            password,
            generate_password(&password, 1.try_into().unwrap(), true)
        );
        assert_eq!(
            password,
            generate_password(&password, 1.try_into().unwrap(), false)
        );
    }
}
