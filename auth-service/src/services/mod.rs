pub mod hashmap_user_store;

//O lib.rs diz pub mod services;, então o compilador procura services/mod.rs. Mas dentro de services/mod.rs, se não tiver pub mod hashmap_user_store;, o Rust não vai incluir hashmap_user_store.rs automaticamente. Consequentemente, seus testes não serão compilados nem executados.