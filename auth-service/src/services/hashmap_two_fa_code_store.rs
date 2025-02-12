use std::collections::HashMap;
use async_trait::async_trait;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(&mut self, email: Email, login_attempt_id: LoginAttemptId, code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.remove(email) {
            Some(_) => Ok(()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some((attempt_id, two_fa_code)) => Ok((attempt_id.clone(), two_fa_code.clone())),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStoreError},
        email::Email,
    };

    #[tokio::test]
    async fn test_add_and_get() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse("test@example.com").expect("Invalid email");
        let login_attempt_id = LoginAttemptId::default();
        let two_fa_code = TwoFACode::default();

        // Adiciona
        store.add_code(
            email.clone(),
            login_attempt_id.clone(),
            two_fa_code.clone()
        ).await.expect("Failed to add 2FA code");

        // Recupera
        let (la_id, code) = store.get_code(&email).await.expect("Code not found");
        assert_eq!(la_id, login_attempt_id);
        assert_eq!(code, two_fa_code);
    }
    
    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse("remove@example.com").expect("Invalid email");
        let login_attempt_id = LoginAttemptId::default();
        let two_fa_code = TwoFACode::default();

        // Adiciona e depois remove
        store.add_code(
            email.clone(),
            login_attempt_id,
            two_fa_code,
        ).await.expect("Failed to add 2FA code");

        store.remove_code(&email).await.expect("Failed to remove 2FA code");

        // Verifica se de fato foi removido
        let result = store.get_code(&email).await;
        assert!(matches!(result, Err(TwoFACodeStoreError::LoginAttemptIdNotFound)));
    }
}