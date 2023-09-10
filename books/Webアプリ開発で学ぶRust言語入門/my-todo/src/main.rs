//!
//! - /todos
//!     - GET: Todo情報の一覧取得
//!     - POST: Todo情報の作成
//! - /todos/:id
//!     - GET: idに対応するTodo情報の取得
//!     - PATCH: Todo情報の更新
//!     - DELETE: Todo情報の削除

use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr},
    sync::{Arc, RwLock},
};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
        .route("/users", post(create_user))
        .route("/todos", post(create_todo))
        .with_state(Arc::new(repository))
}

async fn root() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        name: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

async fn create_todo<R: TodoRepository>(
    State(repository): State<Arc<R>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload).unwrap();

    (StatusCode::CREATED, Json(todo))
}

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

trait TodoRepository: Clone + Send + Sync + 'static {
    fn all(&self) -> Vec<Todo>;
    fn find(&self, id: i32) -> Option<Todo>;
    fn create(&self, payload: CreateTodo) -> Result<Todo, RepositoryError>;
    fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo, RepositoryError>;
    fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

impl Todo {
    fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CreateTodo {
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

type TodoDatas = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,
}

impl TodoRepositoryForMemory {
    fn new() -> Self {
        Self {
            store: Arc::default(),
        }
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn all(&self) -> Vec<Todo> {
        todo!()
    }

    fn find(&self, id: i32) -> Option<Todo> {
        todo!()
    }

    fn create(&self, payload: CreateTodo) -> Result<Todo, RepositoryError> {
        todo!()
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo, RepositoryError> {
        todo!()
    }

    fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::header, http::Method, http::Request};
    use tower::ServiceExt;

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

        assert_eq!(
            user,
            User {
                id: 1337,
                name: "田中 太郎".to_string()
            }
        );
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
