use crate::domain::entity;
use crate::driver::model;
use sqlx;

#[derive(Clone)]
pub struct SupportRepository {
    db: sqlx::PgPool,
}

impl SupportRepository {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }

    pub async fn get_protagonist(
        &self,
        protagonist_id: i64,
    ) -> Result<Option<entity::Protagonist>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::GetProtagonist>(
            r#"
            SELECT 
                protagonist_id, last_name, first_name, login_id, password, email, country
            FROM 
                protagonists
            WHERE 
                protagonist_id = $1;
            "#,
        )
        .bind(protagonist_id)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Protagonist::new(
            row.protagonist_id,
            row.last_name,
            row.first_name,
            row.login_id,
            row.password,
            row.email,
            row.country,
        )))
    }

    pub async fn create_protagonist(
        &self,
        protagonist: model::CreateProtagonist,
    ) -> Result<Option<entity::Protagonist>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::CreateProtagonist>(
            r#"
            INSERT INTO 
                protagonists (last_name, first_name, login_id, password, email, country)
            VALUES 
                ($1, $2, $3, $4, $5, $6)
            RETURNING 
                protagonist_id, last_name, first_name, login_id, password, email, country;
            "#,
        )
        .bind(protagonist.last_name)
        .bind(protagonist.first_name)
        .bind(protagonist.login_id)
        .bind(protagonist.password)
        .bind(protagonist.email)
        .bind(protagonist.country)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Protagonist::new(
            row.protagonist_id,
            row.last_name,
            row.first_name,
            row.login_id,
            row.password,
            row.email,
            row.country,
        )))
    }

    pub async fn update_protagonist(
        &self,
        protagonist: model::UpdateProtagonist,
    ) -> Result<Option<entity::Protagonist>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::UpdateProtagonist>(
            r#"
            UPDATE protagonists
                SET last_name = $1, first_name = $2, login_id = $3, password = $4, email = $5, country = $6
            WHERE 
                protagonist_id = $7
            RETURNING 
                protagonist_id, last_name, first_name, login_id, password, email, country;
            "#,
        )
        .bind(protagonist.last_name)
        .bind(protagonist.first_name)
        .bind(protagonist.login_id)
        .bind(protagonist.password)
        .bind(protagonist.email)
        .bind(protagonist.country)
        .bind(protagonist.protagonist_id)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Protagonist::new(
            row.protagonist_id,
            row.last_name,
            row.first_name,
            row.login_id,
            row.password,
            row.email,
            row.country,
        )))
    }

    pub async fn delete_protagonist(&self, id: i64) -> Result<Option<()>, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM 
                protagonists
            WHERE 
                protagonist_id = $1;
            "#,
        )
        .bind(i64::try_from(id).unwrap())
        .execute(&self.db)
        .await?;

        Ok(Some(()))
    }

    pub async fn get_protagonist_by_login_id_and_password(
        &self,
        login_id: &str,
    ) -> Result<Option<entity::Protagonist>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::GetProtagonist>(
            r#"
            SELECT 
                protagonist_id, last_name, first_name, login_id, password, email, country
            FROM 
                protagonists
            WHERE 
                login_id = $1 
            "#,
        )
        .bind(login_id)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Protagonist::new(
            row.protagonist_id,
            row.last_name,
            row.first_name,
            row.login_id,
            row.password,
            row.email,
            row.country,
        )))
    }

    pub async fn get_supporter(
        &self,
        supporter_id: i64,
    ) -> Result<Option<entity::Supporter>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::GetSupporter>(
            r#"
            SELECT 
                supporter_id, last_name, first_name, login_id, password, email, country
            FROM 
                supporters
            WHERE 
                supporter_id = $1;
            "#,
        )
        .bind(supporter_id)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Supporter::new(
            row.supporter_id,
            row.last_name,
            row.first_name,
            row.login_id,
            row.password,
            row.email,
            row.country,
        )))
    }

    pub async fn create_supporter(
        &self,
        supporter: model::CreateSupporter,
    ) -> Result<Option<entity::Supporter>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::CreateSupporter>(
            r#"
            INSERT INTO 
                supporters (last_name, first_name, login_id, password, email, country)
            VALUES 
                ($1, $2, $3, $4, $5, $6)
            RETURNING 
                supporter_id, last_name, first_name, login_id, password, email, country;
            "#,
        )
        .bind(supporter.last_name)
        .bind(supporter.first_name)
        .bind(supporter.login_id)
        .bind(supporter.password)
        .bind(supporter.email)
        .bind(supporter.country)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Supporter::new(
            row.supporter_id,
            row.last_name,
            row.first_name,
            row.login_id,
            row.password,
            row.email,
            row.country,
        )))
    }

    pub async fn update_supporter(
        &self,
        supporter: model::UpdateSupporter,
    ) -> Result<Option<entity::Supporter>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::UpdateSupporter>(
            r#"
            UPDATE supporters
                SET last_name = $1, first_name = $2, login_id = $3, password = $4, email = $5, country = $6
            WHERE 
                supporter_id = $7
            RETURNING 
                supporter_id, last_name, first_name, login_id, password, email, country;
            "#,
        )
        .bind(supporter.last_name)
        .bind(supporter.first_name)
        .bind(supporter.login_id)
        .bind(supporter.password)
        .bind(supporter.email)
        .bind(supporter.country)
        .bind(supporter.supporter_id)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Supporter::new(
            row.supporter_id,
            row.last_name,
            row.first_name,
            row.login_id,
            row.password,
            row.email,
            row.country,
        )))
    }

    pub async fn delete_supporter(&self, id: i64) -> Result<Option<()>, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM 
                supporters
            WHERE 
                supporter_id = $1;
            "#,
        )
        .bind(i64::try_from(id).unwrap())
        .execute(&self.db)
        .await?;

        Ok(Some(()))
    }

    pub async fn get_supporter_by_login_id_and_password(
        &self,
        login_id: &str,
    ) -> Result<Option<entity::Supporter>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::GetSupporter>(
            r#"
            SELECT 
                supporter_id, last_name, first_name, login_id, password, email, country
            FROM 
                supporters
            WHERE 
                login_id = $1 
            "#,
        )
        .bind(login_id)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Supporter::new(
            row.supporter_id,
            row.last_name,
            row.first_name,
            row.login_id,
            row.password,
            row.email,
            row.country,
        )))
    }

    pub async fn get_protagonist_supporter(
        &self,
        id: i64,
    ) -> Result<Option<Vec<entity::ProtagonistSupporter>>, sqlx::Error> {
        let rows = sqlx::query_as::<_, model::GetProtagonistSupporter>(
            r#"
            SELECT 
                s.supporter_id, s.last_name, s.first_name, s.country
            FROM 
                protagonist_supporters AS p
            INNER JOIN
                supporters AS s 
                    ON p.supporter_id = s.supporter_id
            WHERE 
                p.protagonist_id = $1
            ORDER BY
                created_at DESC;
            "#,
        )
        .bind(i64::try_from(id).unwrap())
        .fetch_all(&self.db)
        .await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let protagonist_supporters = rows
            .into_iter()
            .map(|row| {
                entity::ProtagonistSupporter::new(
                    row.supporter_id,
                    row.last_name,
                    row.first_name,
                    row.country,
                )
            })
            .collect();

        Ok(Some(protagonist_supporters))
    }

    pub async fn create_protagonist_supporter(
        &self,
        protagonist_supporter: model::CreateProtagonistSupporter,
    ) -> Result<Option<entity::ProtagonistSupporterRelation>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::CreateProtagonistSupporter>(
            r#"
            INSERT INTO 
                protagonist_supporters (protagonist_id, supporter_id)
            VALUES 
                ($1, $2)
            RETURNING 
                protagonist_supporter_id, protagonist_id, supporter_id;
            "#,
        )
        .bind(protagonist_supporter.protagonist_id)
        .bind(protagonist_supporter.supporter_id)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::ProtagonistSupporterRelation::new(
            row.protagonist_supporter_id,
        )))
    }

    pub async fn delete_protagonist_supporter(&self, id: i64) -> Result<Option<()>, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM 
                protagonist_supporters
            WHERE 
                protagonist_id = $1;
            "#,
        )
        .bind(i64::try_from(id).unwrap())
        .execute(&self.db)
        .await?;

        Ok(Some(()))
    }
}
