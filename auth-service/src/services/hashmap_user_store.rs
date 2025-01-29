use std::collections::HashMap;
use async_trait::async_trait;

use crate::domain::user;
use crate::services::hashmap_user_store::user::User;
use crate::domain::data_stores::{UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

#[async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let already_exists = self
        .users.contains_key(&user.email);

        if already_exists {
            return Err(UserStoreError::UserAlreadyExists);
        }
        
        self.users.insert(user.email.clone(), user);
        Ok(())
        
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        let user_found = self.users.get(email);

        if let Some(u) = user_found {
            Ok(u.clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError>{
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::User;

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

        let result = store.add_user(user.clone()).await;

        assert!(result.is_ok(), "We expected success to add a new user");

        let result2 = store.add_user(user).await;
        assert_eq!(result2, Err(UserStoreError::UserAlreadyExists), "we expected UserAlreadyExistis when adding the same user twice");
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();

        let user = make_user("test@example.com", "secretpassword");
        store.add_user(user.clone()).await.expect("Failed to add the user");

        let user_existed = store.get_user("test@example.com").await;

        assert!(user_existed.is_ok(), "We expected success to get the user");
        assert_eq!(user_existed.unwrap().email, "test@example.com", "The email of the user is not corret");

        let user_non_existed = store.get_user("emailnotexistedtodayandnever@nevercreated.com.br").await;
        assert_eq!(user_non_existed, Err(UserStoreError::UserNotFound), "we expected this email emailnotexistedtodayandnever@nevercreated.com.br did not exist");
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();

        let user = make_user("test@example.com", "secretpassword");
        store.add_user(user.clone()).await.expect("Failed to add the user");

        let validation = store.validate_user("test@example.com", "secretpassword").await;
        assert!(validation.is_ok(), "Validation was not approved");

        let validation2 = store.validate_user("test@example.com", "wrongpassword").await;
        assert_eq!(validation2, Err(UserStoreError::InvalidCredentials), "Expected wrong password");

        let validation3 = store.validate_user("emailnotvalid@example.com", "anypassword").await;
        assert_eq!(validation3, Err(UserStoreError::UserNotFound), "Expected user not found");
    }
}