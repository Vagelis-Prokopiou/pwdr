pub mod models;
use models::*;

pub fn generate_password(password: &str, repeater: Repeater, alternate_direction: AlternateDirection) -> String {
    let repeat: i8 = repeater.into();
    if repeat == 1 {
        return password.to_string();
    }

    let mut _p = String::new();
    _p.push_str(&password);

    if alternate_direction == AlternateDirection::Yes {
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
    use crate::generate_password;

    #[test]
    fn generate_password_correctly_repeats_with_direction_alternation() {
        let password = "aBcdef123";
        let alternate_password = "321fedcBa";
        assert_eq!(
            format!("{}{}", password, alternate_password),
            generate_password(&password, 2.try_into().unwrap(), true.into())
        );
        assert_eq!(
            format!("{}{}{}", password, alternate_password, password),
            generate_password(&password, 3.try_into().unwrap(), true.into())
        );
        assert_eq!(
            format!(
                "{}{}{}{}",
                password, alternate_password, password, alternate_password
            ),
            generate_password(&password, 4.try_into().unwrap(), true.into())
        );
    }

    #[test]
    fn generate_password_correctly_repeats_without_direction_alternation() {
        let password = "aBcdef123";
        assert_eq!(
            format!("{}{}", password, password),
            generate_password(&password, 2.try_into().unwrap(), false.into())
        );
        assert_eq!(
            format!("{}{}{}", password, password, password),
            generate_password(&password, 3.try_into().unwrap(), false.into())
        );
        assert_eq!(
            format!("{}{}{}{}", password, password, password, password),
            generate_password(&password, 4.try_into().unwrap(), false.into())
        );
    }

    #[test]
    fn generate_password_returns_initial_pasword_if_repeat_is_one_regardles_of_direction_alternation(
    ) {
        let password = "aBcdef123";
        assert_eq!(
            password,
            generate_password(&password, 1.try_into().unwrap(), true.into())
        );
        assert_eq!(
            password,
            generate_password(&password, 1.try_into().unwrap(), false.into())
        );
    }
}
