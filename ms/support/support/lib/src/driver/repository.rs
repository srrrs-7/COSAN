use super::model::GetProtagonist;
use sqlx;

pub struct SupportRepository {
    db: sqlx::PgPool,
}

impl SupportRepository {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }

    pub async fn get_protagonist(&self, id: i64) -> Result<GetProtagonist, sqlx::Error> {
        let row = sqlx::query_as::<_, GetProtagonist>(
            r#"
            SELECT 
                protagonist_id, last_name, first_name, email, country
            FROM 
                protagonist
            WHERE 
                protagonist_id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.db)
        .await?;

        Ok(GetProtagonist {
            protagonist_id: row.protagonist_id,
            last_name: row.last_name,
            first_name: row.first_name,
            email: row.email,
            country: row.country,
        })
    }
}
