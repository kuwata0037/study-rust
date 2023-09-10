//!
//! - /todos
//!     - GET: Todo情報の一覧取得
//!     - POST: Todo情報の作成
//! - /todos/:id
//!     - GET: idに対応するTodo情報の取得
//!     - PATCH: Todo情報の更新
//!     - DELETE: Todo情報の削除

mod handler;
mod repository;

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{
    routing::{get, post},
    Router,
};
use repository::TodoRepository;

use crate::repository::TodoRepositoryForMemory;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let repository = TodoRepositoryForMemory::new();

    let app = create_app(repository);

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));

    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app<R: TodoRepository>(repository: R) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(handler::user::create_user))
        .route("/todos", post(handler::todo::create_todo))
        .with_state(Arc::new(repository))
}

async fn root() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::handler::user::User;

    use super::*;

    #[tokio::test]
    async fn should_return_hello_world() {
        let repository = TodoRepositoryForMemory::new();

        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Hello, world!");
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let repository = TodoRepositoryForMemory::new();

        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "username": "田中 太郎" }"#))
            .unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("cannot convert User instance.");

        assert_eq!(user, User::new(1337, "田中 太郎".to_string()));
    }

    #[tokio::test]
    async fn should_not_found() {
        let repository = TodoRepositoryForMemory::new();

        let req = Request::builder()
            .uri("/not-exist")
            .body(Body::empty())
            .unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
