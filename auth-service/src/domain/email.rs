#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(s: &str) -> Result<Self, String> {
        if !s.contains('@') {
            return Err("The email does not contain @".to_string())
        }
        Ok(Self(s.to_owned()))
    }
}

// Implement `AsRef<str>` so that `&Email` can be used as a `&str`.
impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;

    #[test]
    fn valid_email() {
        let email_str = "testexample@test.com";
        let email = Email::parse(email_str).expect("Expected a valid email");
        assert_eq!(email.as_ref(), email_str);
    }

    #[test]
    fn invalid_email() {
        let email_str = "shouldfailemailinvalid.com";
        let result = Email::parse(email_str);
        assert!(result.is_err(), "Expected an error for missing '@'");
    }

    #[test]
    fn invalid_email_empty_string() {
        let email_str = "";
        let result = Email::parse(email_str);
        assert!(result.is_err(), "Expected an error for empty string");
    }
}