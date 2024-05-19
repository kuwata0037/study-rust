use std::sync::Arc;

use axum::{
    http::StatusCode,
    routing::{delete, get, post},
    Extension, Router,
};

use crate::repository::{label::LabelRepository, todo::TodoRepository, RepositoryError};

use self::{
    label::{all_label, create_label, delete_label},
    todo::{all_todo, create_todo, delete_todo, find_todo, update_todo},
    user::create_user,
};

mod label;
mod todo;
mod user;

pub fn create_app<Todo: TodoRepository, Label: LabelRepository>(
    todo_repository: Todo,
    label_repository: Label,
) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/todos", get(all_todo::<Todo>).post(create_todo::<Todo>))
        .route(
            "/todos/:id",
            get(find_todo::<Todo>)
                .patch(update_todo::<Todo>)
                .delete(delete_todo::<Todo>),
        )
        .layer(Extension(Arc::new(todo_repository)))
        .route(
            "/labels",
            get(all_label::<Label>).post(create_label::<Label>),
        )
        .route("/label/:id", delete(delete_label::<Label>))
        .layer(Extension(Arc::new(label_repository)))
}

async fn root() -> &'static str {
    "Hello, world!"
}

fn handle_error(error: RepositoryError) -> StatusCode {
    match error {
        RepositoryError::NotFound(_id) => StatusCode::NOT_FOUND,
        RepositoryError::Duplicate(_id) => StatusCode::BAD_REQUEST,
        RepositoryError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::{
        label::LabelRepositoryForMemory,
        todo::{CreateTodo, Todo, TodoRepository, TodoRepositoryForMemory},
    };
    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode},
        response::Response,
    };
    use tower::ServiceExt;
    use user::User;

    use super::*;

    async fn res_to_todo(res: Response) -> Todo {
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Todo = serde_json::from_str(&body)
            .unwrap_or_else(|_| panic!("cannot convert Todo instance. body: {body}"));
        todo
    }

    #[tokio::test]
    async fn should_not_found() {
        let todo_repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();

        let req = Request::builder()
            .uri("/not-exist")
            .body(Body::empty())
            .unwrap();
        let res = create_app(todo_repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn should_return_hello_world() {
        let todo_repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();

        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(todo_repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Hello, world!");
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let todo_repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();

        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "username": "田中 太郎" }"#))
            .unwrap();
        let res = create_app(todo_repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("cannot convert User instance.");

        assert_eq!(user, User::new(1337, "田中 太郎".to_string()));
    }

    #[tokio::test]
    async fn should_created_todo() {
        let todo_repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();

        let req = Request::builder()
            .uri("/todos")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "text": "should_return_created_todo" }"#))
            .unwrap();

        let res = create_app(todo_repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);

        let todo = res_to_todo(res).await;
        let expected = Todo::new(1, "should_return_created_todo".to_string());
        assert_eq!(todo, expected);
    }

    #[tokio::test]
    async fn should_find_todo() {
        let repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();

        repository
            .create(CreateTodo::new("should_find_todo".to_string()))
            .await
            .unwrap();

        let req = Request::builder()
            .uri("/todos/1")
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();
        let res = create_app(repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let todo = res_to_todo(res).await;
        let expected = Todo::new(1, "should_find_todo".to_string());
        assert_eq!(todo, expected);
    }

    #[tokio::test]
    async fn should_all_todos() {
        let repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();

        repository
            .create(CreateTodo::new("should_all_todos".to_string()))
            .await
            .unwrap();

        let req = Request::builder()
            .uri("/todos")
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();
        let res = create_app(repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let todos: Vec<Todo> = serde_json::from_str(&body)
            .unwrap_or_else(|_| panic!("cannot convert Todo instance. body: {body}"));
        let expected = Todo::new(1, "should_all_todos".to_string());
        assert_eq!(todos, vec![expected]);
    }

    #[tokio::test]
    async fn should_update_todo() {
        let repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();

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
        let res = create_app(repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);

        let todo = res_to_todo(res).await;
        let expected = Todo::new(1, "should_update_todo".to_string());
        assert_eq!(todo, expected);
    }

    #[tokio::test]
    async fn should_delete_todo() {
        let repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();

        repository
            .create(CreateTodo::new("should_delete_todo".to_string()))
            .await
            .unwrap();

        let req = Request::builder()
            .uri("/todos/1")
            .method(Method::DELETE)
            .body(Body::empty())
            .unwrap();
        let res = create_app(repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
