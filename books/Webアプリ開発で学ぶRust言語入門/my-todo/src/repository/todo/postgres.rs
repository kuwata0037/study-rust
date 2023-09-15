use sqlx::{FromRow, PgPool};

use crate::repository::RepositoryError;

use super::{CreateTodo, Todo, TodoRepository, UpdateTodo};

#[derive(Debug, Clone)]
pub struct TodoRepositoryForPostgres {
    pool: PgPool,
}

impl TodoRepositoryForPostgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, Clone, FromRow)]
struct TodoDto {
    id: i32,
    text: String,
    completed: bool,
}

impl From<TodoDto> for Todo {
    fn from(dto: TodoDto) -> Self {
        Self {
            id: dto.id as u32,
            text: dto.text,
            completed: dto.completed,
        }
    }
}

#[axum::async_trait]
impl TodoRepository for TodoRepositoryForPostgres {
    async fn all(&self) -> Result<Vec<Todo>, RepositoryError> {
        let todos = sqlx::query_as::<_, TodoDto>(
            r#"
                SELECT *
                FROM todo
                ORDER BY id DESC;
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(handle_sqlx_error)?;

        let todos = todos.into_iter().map(Todo::from).collect();
        Ok(todos)
    }

    async fn find(&self, id: u32) -> Result<Todo, RepositoryError> {
        let todo = sqlx::query_as::<_, TodoDto>(
            r#"
                SELECT *
                FROM todo
                WHERE id = $1;
            "#,
        )
        .bind(id as i32)
        .fetch_optional(&self.pool)
        .await
        .map_err(handle_sqlx_error)?
        .ok_or_else(|| RepositoryError::NotFound(id))?;

        let todo = Todo::from(todo);
        Ok(todo)
    }

    async fn create(&self, payload: CreateTodo) -> Result<Todo, RepositoryError> {
        let todo = sqlx::query_as::<_, TodoDto>(
            r#"
                INSERT INTO todo (text, completed)
                VALUES ($1, false)
                returning *;
            "#,
        )
        .bind(payload.text)
        .fetch_one(&self.pool)
        .await
        .unwrap();

        let todo = Todo::from(todo);
        Ok(todo)
    }

    async fn update(&self, id: u32, payload: UpdateTodo) -> Result<Todo, RepositoryError> {
        let before_todo = self.find(id).await?;

        let todo = sqlx::query_as::<_, TodoDto>(
            r#"
                UPDATE todo
                set text     = $1,
                    completed= $2
                WHERE id = $3
                RETURNING *;
            "#,
        )
        .bind(payload.text.unwrap_or(before_todo.text))
        .bind(payload.completed.unwrap_or(before_todo.completed))
        .bind(id as i32)
        .fetch_one(&self.pool)
        .await
        .map_err(handle_sqlx_error)?;

        let todo = Todo::from(todo);
        Ok(todo)
    }

    async fn delete(&self, id: u32) -> Result<(), RepositoryError> {
        let result = sqlx::query(
            r#"
                DELETE
                FROM todo
                WHERE id = $1;
            "#,
        )
        .bind(id as i32)
        .execute(&self.pool)
        .await
        .map_err(handle_sqlx_error)?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound(id));
        }

        Ok(())
    }
}

fn handle_sqlx_error(error: sqlx::Error) -> RepositoryError {
    RepositoryError::Unexpected(error.into())
}
