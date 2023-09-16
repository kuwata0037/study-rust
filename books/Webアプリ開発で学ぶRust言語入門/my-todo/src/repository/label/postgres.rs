use axum::async_trait;
use sqlx::PgPool;

use crate::repository::RepositoryError;

use super::{CreateLabel, Label, LabelRepository};

#[derive(Debug, Clone)]
pub struct LabelRepositoryForPostgres {
    pool: PgPool,
}

impl LabelRepositoryForPostgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LabelRepository for LabelRepositoryForPostgres {
    async fn all(&self) -> Result<Vec<Label>, RepositoryError> {
        let labels = sqlx::query_as::<_, Label>(
            r#"
                SELECT *
                FROM label
                ORDER BY id ASC;
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(handle_sqlx_error)?;

        Ok(labels)
    }

    async fn create(&self, payload: CreateLabel) -> Result<Label, RepositoryError> {
        let label = sqlx::query_as::<_, Label>(
            r#"
                SELECT *
                FROM label
                WHERE name = $1;
            "#,
        )
        .bind(&payload.name)
        .fetch_optional(&self.pool)
        .await
        .map_err(handle_sqlx_error)?;

        if let Some(label) = label {
            return Err(RepositoryError::Duplicate(label.id));
        }

        let label = sqlx::query_as::<_, Label>(
            r#"
                INSERT INTO label (name)
                VALUES ($1)
                RETURNING *;
            "#,
        )
        .bind(&payload.name)
        .fetch_one(&self.pool)
        .await
        .map_err(handle_sqlx_error)?;

        Ok(label)
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        let result = sqlx::query(
            r#"
                DELETE
                FROM label
                WHERE id = $1;
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(handle_sqlx_error)?;
        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound(id as u32));
        }

        Ok(())
    }
}

fn handle_sqlx_error(error: sqlx::Error) -> RepositoryError {
    RepositoryError::Unexpected(error.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "Dependence of database"]
    #[tokio::test]
    async fn crud_scenario() {
        let database_url = std::env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("fail connect database");
        let repository = LabelRepositoryForPostgres::new(pool);

        let label_text = "test_label";

        // create
        let created = repository
            .create(CreateLabel {
                name: label_text.to_string(),
            })
            .await
            .expect("fail create label");
        assert_eq!(created.name, label_text);

        // all
        let labels = repository.all().await.expect("fail fetch all labels");
        let label = labels.into_iter().next().unwrap();
        assert_eq!(label.name, label_text);

        // delete
        repository
            .delete(created.id)
            .await
            .expect("fail delete label");
    }
}
