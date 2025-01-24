mod login;
mod logout;
mod signup;
mod verify_2fa;
mod verify_token;

pub use login::*;
pub use logout::*;
pub use signup::*;
pub use verify_2fa::*;
pub use verify_token::*;

//mod login; etc. – Diz ao compilador “tenho um submódulo login (em login.rs)”.
//pub use login::*; – Pega tudo que for público dentro do módulo login e o torna acessível diretamente no módulo “pai”.


use axum::response::IntoResponse;
use axum::http::StatusCode;
