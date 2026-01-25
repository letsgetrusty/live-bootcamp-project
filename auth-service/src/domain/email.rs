#[derive(Clone)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email, String> {
        if !s.is_empty() && s.contains('@') {
            Ok(Email(s))
        } else {
            Err(format!("'{}' is not a valid email address.", s))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_parse() {
        let email = Email::parse("test@example.com".to_string()).unwrap();
        assert_eq!(email.as_ref(), "test@example.com");
    }

    #[test]
    fn test_email_parse_invalid() {
        let result = Email::parse("invalid-email".to_string());
        assert!(result.is_err());

        let result = Email::parse("".to_string());
        assert!(result.is_err());
    }
}
