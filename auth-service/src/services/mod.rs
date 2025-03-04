pub mod hashmap_user_store;
pub mod hashset_banned_token_store;
pub mod hashmap_two_fa_code_store;
pub mod mock_email_client;

//O lib.rs diz pub mod services;, então o compilador procura services/mod.rs. Mas dentro de services/mod.rs, se não tiver pub mod hashmap_user_store;, o Rust não vai incluir hashmap_user_store.rs automaticamente. Consequentemente, seus testes não serão compilados nem executados.