//! # My todo API
//!
//! ## API
//!
//! - /todos
//!     - GET: Todo情報の一覧取得
//!     - POST: Todo情報の作成
//! - /todos/:id
//!     - GET: idに対応するTodo情報の取得
//!     - PATCH: Todo情報の更新
//!     - DELETE: Todo情報の削除

use std::net::{Ipv4Addr, SocketAddr};

use sqlx::PgPool;
use web_rust_my_todo::{
    handler::create_app,
    repository::{label::LabelRepositoryForPostgres, todo::TodoRepositoryForPostgres},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::debug!("start connect database");
    let database_url = std::env::var("DATABASE_URL").expect("Undefined [DATABASE_URL]");
    let pool = PgPool::connect(&database_url)
        .await
        .expect(&format!("fail connect database"));

    let todo_repository = TodoRepositoryForPostgres::new(pool.clone());
    let label_repository = LabelRepositoryForPostgres::new(pool);

    let app = create_app(todo_repository, label_repository);

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
