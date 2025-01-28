use std::collections::HashMap;

use crate::domain::user;
use crate::services::hashmap_user_store::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// : Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        let already_exists = self
        .users.contains_key(&user.email);

        if already_exists {
            return Err(UserStoreError::UserAlreadyExists);
        }
        
        self.users.insert(user.email.clone(), user);
        Ok(())
        
    }

    // : Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        let user_found = self.users.get(email);

        if let Some(u) = user_found {
            Ok(u.clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    // : Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.

    pub fn validate_user(&mut self, email: &str, password: &str) -> Result<(), UserStoreError>{
        let user_found = self.users.get(email);

        match user_found {
            None => {Err(UserStoreError::UserNotFound)}
            Some(u) => if u.password == password {
                Ok(())
            } else {
                Err(UserStoreError::InvalidCredentials)
            }
        }

    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    fn make_user(email: &str, password: &str) -> User {
        User {
            email: email.to_string(), //trocar para função de gerador aleatorio de email
            password: password.to_string(),
            requires_2fa: false,
        }
    }

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();

        let user = make_user("test@example.com", "secretpassword");

        let result = store.add_user(user.clone());

        assert!(result.is_ok(), "We expected success to add a new user");

        let result2 = store.add_user(user);
        assert_eq!(result2, Err(UserStoreError::UserAlreadyExists), "we expected UserAlreadyExistis when adding the same user twice");
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();

        let user = make_user("test@example.com", "secretpassword");
        store.add_user(user.clone()).expect("Failed to add the user");

        let user_existed = store.get_user("test@example.com");

        assert!(user_existed.is_ok(), "We expected success to get the user");
        assert_eq!(user_existed.unwrap().email, "test@example.com", "The email of the user is not corret");

        let user_non_existed = store.get_user("emailnotexistedtodayandnever@nevercreated.com.br");
        assert_eq!(user_non_existed, Err(UserStoreError::UserNotFound), "we expected this email emailnotexistedtodayandnever@nevercreated.com.br did not exist");
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();

        let user = make_user("test@example.com", "secretpassword");
        store.add_user(user.clone()).expect("Failed to add the user");

        let validation = store.validate_user("test@example.com", "secretpassword");
        assert!(validation.is_ok(), "Validation was not approved");

        let validation2 = store.validate_user("test@example.com", "wrongpassword");
        assert_eq!(validation2, Err(UserStoreError::InvalidCredentials), "Expected wrong password");

        let validation3 = store.validate_user("emailnotvalid@example.com", "anypassword");
        assert_eq!(validation3, Err(UserStoreError::UserNotFound), "Expected user not found");
    }
}