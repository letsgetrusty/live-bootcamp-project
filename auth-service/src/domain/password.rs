#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(s: String) -> Result<Password, String> {
        if s.len() < 8 {
            Err("Password must be at least 8 characters long".to_string())
        } else {
            Ok(Password(s))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_parse() {
        let password = Password::parse("strongpassword".to_string()).unwrap();
        assert_eq!(password.as_ref(), "strongpassword");
    }

    #[test]
    fn test_password_parse_invalid() {
        let result = Password::parse("short".to_string());
        assert!(result.is_err());
    }
}
