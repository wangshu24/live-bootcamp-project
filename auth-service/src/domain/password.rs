#[derive(Debug, PartialEq, Clone)]
pub struct Password(String);

impl Password {
    pub fn parse(str: String) -> Result<Password, String> {
        if str.len() < 8 {
            Err(format!("{} is invalid password", str))
        } else {
            Ok(Password(str))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
