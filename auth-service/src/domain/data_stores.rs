use crate::domain::{user::User, email::Email, password::Password}; 
use async_trait::async_trait;

#[async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

pub trait BannedTokenStore  {
    fn store_token(&mut self, token: &str);

    fn is_banned(&self, token: &str) -> bool;
}