use std::collections::HashSet;
use std::sync::{Arc, Mutex}; // ou RwLock, se preferir
use crate::domain::data_stores::BannedTokenStore;

/// Implementação concreta que usa um HashSet<String>
pub struct HashsetBannedTokenStore {
    // Usa Mutex ou RwLock para permitir acesso concorrente
    tokens: Arc<Mutex<HashSet<String>>>,
}

impl HashsetBannedTokenStore {
    /// Cria uma nova instância com HashSet vazio
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(Mutex::new(HashSet::new())),
        }
    }
}

impl BannedTokenStore for HashsetBannedTokenStore {
    fn store_token(&mut self, token: &str) {
        // Bloqueia o Mutex antes de inserir no HashSet
        let mut guard = self.tokens.lock().unwrap();
        guard.insert(token.to_string());
    }

    fn is_banned(&self, token: &str) -> bool {
        // Verifica se o token está presente
        let guard = self.tokens.lock().unwrap();
        guard.contains(token)
    }
}

// Se quiser permitir clonagem, por exemplo:
impl Clone for HashsetBannedTokenStore {
    fn clone(&self) -> Self {
        Self {
            tokens: Arc::clone(&self.tokens),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::data_stores::BannedTokenStore;

    #[test]
    fn test_store_and_check_token() {
        let mut store = HashsetBannedTokenStore::new();
        let token = "test_token_123";

        assert_eq!(store.is_banned(token), false);

        store.store_token(token);

        assert_eq!(store.is_banned(token), true);
    }
}
