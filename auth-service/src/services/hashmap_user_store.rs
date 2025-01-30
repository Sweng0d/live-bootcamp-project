use std::collections::HashMap;
use async_trait::async_trait;

use crate::domain::{user::User, email::Email, password::Password}; 
use crate::domain::data_stores::{UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
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

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let user_found = self.users.get(email);

        if let Some(u) = user_found {
            Ok(u.clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError>{
        let user_found = self.users.get(email);

        match user_found {
            None => {Err(UserStoreError::UserNotFound)}
            Some(u) => if u.password == *password {
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

    fn make_user(email_str: &str, password_str: &str) -> User {
        let email = Email::parse(email_str).unwrap();
        let password = Password::parse(password_str).unwrap();
        User {
            email,
            password,
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

        // Em vez de passar &str, convertendo para Email
        let email_ok = Email::parse("test@example.com").unwrap();
        let user_existed = store.get_user(&email_ok).await;

        assert!(user_existed.is_ok(), "We expected success to get the user");

        // Comparar a string interna do Email com "test@example.com"
        // ou comparar `user_existed.unwrap().email` com user.email
        assert_eq!(
            user_existed.as_ref().unwrap().email.as_ref(),
            "test@example.com",
            "The email of the user is not correct"
        );

        // Agora testamos com um email inexistente
        let email_none = Email::parse("emailnotexistedtodayandnever@nevercreated.com.br").unwrap();
        let user_non_existed = store.get_user(&email_none).await;
        assert_eq!(
            user_non_existed,
            Err(UserStoreError::UserNotFound),
            "we expected this email emailnotexistedtodayandnever@nevercreated.com.br did not exist"
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();

        let user = make_user("test@example.com", "secretpassword");
        store.add_user(user.clone()).await.expect("Failed to add the user");

        // Caso de sucesso
        let email_ok = Email::parse("test@example.com").unwrap();
        let password_ok = Password::parse("secretpassword").unwrap();
        let validation = store.validate_user(&email_ok, &password_ok).await;
        assert!(validation.is_ok(), "Validation was not approved");

        // Senha errada => InvalidCredentials
        let wrong_password = Password::parse("wrongpassword").unwrap();
        let validation2 = store.validate_user(&email_ok, &wrong_password).await;
        assert_eq!(
            validation2,
            Err(UserStoreError::InvalidCredentials),
            "Expected wrong password"
        );

        // Email inexistente => UserNotFound
        let email_none = Email::parse("emailnotvalid@example.com").unwrap();
        let pass_any = Password::parse("anypassword").unwrap();
        let validation3 = store.validate_user(&email_none, &pass_any).await;
        assert_eq!(
            validation3,
            Err(UserStoreError::UserNotFound),
            "Expected user not found"
        );
    }

}