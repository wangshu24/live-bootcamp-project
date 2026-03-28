use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    pub async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if password == user.password {
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
        let user = User::new("user@gmail.com".to_owned(), "pass".to_owned(), false);

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
        let user = User::new("user@gmail.com".to_owned(), "pass".to_owned(), false);
        let _ = store.add_user(user.clone()).await;
        // Test getting valid user
        let result = store.get_user("user@gmail.com").await;
        assert_eq!(result, Ok(user));

        // Test getting invalid user
        let result = store.get_user("nobody@gmail.com").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("user@gmail.com".to_owned(), "pass".to_owned(), false);
        let _ = store.add_user(user.clone()).await;

        // Test correct password
        let result = store.validate_user("user@gmail.com", "pass").await;
        assert_eq!(result, Ok(()));

        // Test incorrect password
        let result = store.validate_user("user@gmail.com", "incorrect").await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // Test unknown user
        let result = store.validate_user("unknown@gmail.com", "pass").await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
