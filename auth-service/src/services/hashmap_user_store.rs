use crate::domain::{Email, Password, User, UserStore, UserStoreError};
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if *password == user.password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let email = Email::parse("user@gmail.com".to_owned()).unwrap();
        let password = Password::parse("validpass".to_owned()).unwrap();
        let user = User::new(email, password, false);

        // Test adding new user
        let result = store.add_user(user.clone()).await;
        assert!(result.is_ok());

        // Test adding existing user
        let result = store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let email = Email::parse("user@gmail.com".to_owned()).unwrap();
        let password = Password::parse("validpass".to_owned()).unwrap();
        let user = User::new(email.clone(), password, false);
        let _ = store.add_user(user.clone()).await;

        // Test getting valid user
        let result = store.get_user(&email).await;
        assert_eq!(result, Ok(user));

        let invalid_email = Email::parse("nobody@gmail.com".to_owned()).unwrap();
        // Test getting invalid user
        let result = store.get_user(&invalid_email).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let email = Email::parse("user@gmail.com".to_owned()).unwrap();
        let password = Password::parse("validpass".to_owned()).unwrap();
        let user = User::new(email.clone(), password.clone(), false);
        let _ = store.add_user(user.clone()).await;

        // Test correct password
        let result = store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));

        let incorrect_password = Password::parse("incorrectpass".to_owned()).unwrap();
        // Test incorrect password
        let result = store.validate_user(&email, &incorrect_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        let invalid_email = Email::parse("nobody@gmail.com".to_owned()).unwrap();
        // Test unknown user
        let result = store.validate_user(&invalid_email, &password).await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
