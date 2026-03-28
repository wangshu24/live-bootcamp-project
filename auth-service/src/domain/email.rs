use validator::ValidateEmail;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email, String> {
        if !s.validate_email() {
            return Err(format!("{} not valid email format", s));
        } else {
            return Ok(Self(s));
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
