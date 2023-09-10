use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: String,
}

impl User {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        name: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
