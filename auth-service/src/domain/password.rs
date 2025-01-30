#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(s: &str) -> Result<Self, String> {
        if s.len() < 8 {
            return Err(format!(
                "`{}` is not a valid password: must be at least 8 characters long",
                s
            ));
        }
        Ok(Self(s.to_owned()))
    }   
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    #[test]
    fn valid_password() {
        let password_str = "senha123456";
        let password = Password::parse(password_str).expect("Expected a valid password");
        assert_eq!(password.as_ref(), password_str);
    }

    #[test]
    fn invalid_password() {
        let password_str = "123456";
        let result = Password::parse(password_str);
        assert!(result.is_err(), "Expected an error for password with len < 8");
    }

    #[test]
    fn invalid_empty_password() {
        let password_str = "";
        let result = Password::parse(password_str);
        assert!(result.is_err(), "Expected an error for empty password");
    }
}