//!
//! - /todos
//!     - GET: Todo情報の一覧取得
//!     - POST: Todo情報の作成
//! - /todos/:id
//!     - GET: idに対応するTodo情報の取得
//!     - PATCH: Todo情報の更新
//!     - DELETE: Todo情報の削除

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{
    routing::{get, post},
    Router,
};

use sqlx::PgPool;
use web_rust_my_todo::{
    handler::{
        todo::{all_todo, create_todo, delete_todo, find_todo, update_todo},
        user::create_user,
    },
    repository::todo::{TodoRepository, TodoRepositoryForPostgres},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("Undefined [DATABASE_URL]");

    tracing::debug!("start connect database");
    let pool = PgPool::connect(&database_url)
        .await
        .expect(&format!("fail connect database, url is [{database_url}]"));
    let repository = TodoRepositoryForPostgres::new(pool);

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
        .route("/todos", get(all_todo).post(create_todo))
        .route(
            "/todos/:id",
            get(find_todo).patch(update_todo).delete(delete_todo),
        )
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
        response::Response,
    };
    use tower::ServiceExt;
    use web_rust_my_todo::{
        handler::user::User,
        repository::todo::{CreateTodo, Todo, TodoRepositoryForMemory},
    };

    use super::*;

    async fn res_to_todo(res: Response) -> Todo {
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Todo = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo instance. body: {body}"));
        todo
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
    async fn should_created_todo() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder()
            .uri("/todos")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "text": "should_return_created_todo" }"#))
            .unwrap();

        let res = create_app(repository).oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);

        let todo = res_to_todo(res).await;
        let expected = Todo::new(1, "should_return_created_todo".to_string());
        assert_eq!(todo, expected);
    }

    #[tokio::test]
    async fn should_find_todo() {
        let repository = TodoRepositoryForMemory::new();
        repository
            .create(CreateTodo::new("should_find_todo".to_string()))
            .await
            .unwrap();

        let req = Request::builder()
            .uri("/todos/1")
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let todo = res_to_todo(res).await;
        let expected = Todo::new(1, "should_find_todo".to_string());
        assert_eq!(todo, expected);
    }

    #[tokio::test]
    async fn should_all_todos() {
        let repository = TodoRepositoryForMemory::new();
        repository
            .create(CreateTodo::new("should_all_todos".to_string()))
            .await
            .unwrap();

        let req = Request::builder()
            .uri("/todos")
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let todos: Vec<Todo> = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo instance. body: {body}"));
        let expected = Todo::new(1, "should_all_todos".to_string());
        assert_eq!(todos, vec![expected]);
    }

    #[tokio::test]
    async fn should_update_todo() {
        let repository = TodoRepositoryForMemory::new();
        repository
            .create(CreateTodo::new("before_update_todo".to_string()))
            .await
            .unwrap();

        let req = Request::builder()
            .uri("/todos/1")
            .method(Method::PATCH)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                r#"
                {
                    "text": "should_update_todo",
                    "completed": false
                }"#,
            ))
            .unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);

        let todo = res_to_todo(res).await;
        let expected = Todo::new(1, "should_update_todo".to_string());
        assert_eq!(todo, expected);
    }

    #[tokio::test]
    async fn should_delete_todo() {
        let repository = TodoRepositoryForMemory::new();
        repository
            .create(CreateTodo::new("should_delete_todo".to_string()))
            .await
            .unwrap();

        let req = Request::builder()
            .uri("/todos/1")
            .method(Method::DELETE)
            .body(Body::empty())
            .unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
