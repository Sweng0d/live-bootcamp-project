use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::user::User;
use crate::domain::error::AuthAPIError;
use crate::domain::data_stores::UserStoreError;

// Se `UserStore` for realmente assíncrono, você chamará `.await` nos métodos
// *Se* seu store atual for assíncrono, lembre de `await` no get_user e add_user
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;

    // 1) Validação rápida
    // Retorna 400 (InvalidCredentials) se email ou password forem inválidos
    if email.is_empty() || !email.contains('@') || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    // 2) Cria User
    let user = User::new(email, password, request.requires_2fa);

    // 3) Obtem lock de escrita no store
    let mut user_store = state.user_store.write().await;

    // 4) Verifica se já existe usuário com este email
    //    Se `get_user(...)` for assíncrono, use `.await`
    match user_store.get_user(&user.email).await {
        Ok(_) => {
            // Se OK => já existe esse email
            return Err(AuthAPIError::UserAlreadyExists);
        }
        Err(UserStoreError::UserNotFound) => {
            // Se for UserNotFound => não existe => prossegue
        }
        Err(_) => {
            // Se for qualquer outro erro => Unexpected
            return Err(AuthAPIError::UnexpectedError);
        }
    }

    // 5) Tenta adicionar de fato
    match user_store.add_user(user).await {
        Ok(_) => {
            // Sucesso => 201
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });
            Ok((StatusCode::CREATED, response))
        }
        Err(_) => {
            // Qualquer erro => Unexpected
            Err(AuthAPIError::UnexpectedError)
        }
    }
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
