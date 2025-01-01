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
                protagonist_id, last_name, first_name, email, country
            FROM 
                protagonists
            WHERE 
                protagonist_id = $1;
            "#,
        )
        .bind(protagonist_id)
        .fetch_one(&self.db)
        .await?;

        Ok(Some(entity::Protagonist {
            protagonist_id: row.protagonist_id,
            last_name: row.last_name,
            first_name: row.first_name,
            email: row.email,
            country: row.country,
        }))
    }

    pub async fn create_protagonist(
        &self,
        protagonist: model::CreateProtagonist,
    ) -> Result<Option<entity::Protagonist>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::CreateProtagonist>(
            r#"
            INSERT INTO 
                protagonists (last_name, first_name, email, country)
            VALUES 
                ($1, $2, $3, $4)
            RETURNING 
                protagonist_id, last_name, first_name, email, country;
            "#,
        )
        .bind(protagonist.last_name)
        .bind(protagonist.first_name)
        .bind(protagonist.email)
        .bind(protagonist.country)
        .fetch_one(&self.db)
        .await?;

        Ok(Some(entity::Protagonist {
            protagonist_id: row.protagonist_id,
            last_name: row.last_name,
            first_name: row.first_name,
            email: row.email,
            country: row.country,
        }))
    }

    pub async fn update_protagonist(
        &self,
        protagonist: model::UpdateProtagonist,
    ) -> Result<Option<entity::Protagonist>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::UpdateProtagonist>(
            r#"
            UPDATE protagonists
                SET last_name = $1, first_name = $2, email = $3, country = $4
            WHERE 
                protagonist_id = $5
            RETURNING 
                protagonist_id, last_name, first_name, email, country;
            "#,
        )
        .bind(protagonist.last_name)
        .bind(protagonist.first_name)
        .bind(protagonist.email)
        .bind(protagonist.country)
        .bind(protagonist.protagonist_id)
        .fetch_one(&self.db)
        .await?;

        Ok(Some(entity::Protagonist {
            protagonist_id: row.protagonist_id,
            last_name: row.last_name,
            first_name: row.first_name,
            email: row.email,
            country: row.country,
        }))
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
        login_id: String,
        password: String,
    ) -> Result<Option<entity::Protagonist>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::GetProtagonist>(
            r#"
            SELECT 
                protagonist_id, last_name, first_name, email, country
            FROM 
                protagonists
            WHERE 
                login_id = $1 AND password = $2;
            "#,
        )
        .bind(login_id)
        .bind(password)
        .fetch_one(&self.db)
        .await?;

        Ok(Some(entity::Protagonist {
            protagonist_id: row.protagonist_id,
            last_name: row.last_name,
            first_name: row.first_name,
            email: row.email,
            country: row.country,
        }))
    }

    pub async fn get_supporter(
        &self,
        supporter_id: i64,
    ) -> Result<Option<entity::Supporter>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::GetSupporter>(
            r#"
            SELECT 
                supporter_id, last_name, first_name, email, country
            FROM 
                supporters
            WHERE 
                supporter_id = $1;
            "#,
        )
        .bind(supporter_id)
        .fetch_one(&self.db)
        .await?;

        Ok(Some(entity::Supporter {
            supporter_id: row.supporter_id,
            last_name: row.last_name,
            first_name: row.first_name,
            email: row.email,
            country: row.country,
        }))
    }

    pub async fn create_supporter(
        &self,
        supporter: model::CreateSupporter,
    ) -> Result<Option<entity::Supporter>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::CreateSupporter>(
            r#"
            INSERT INTO 
                supporters (last_name, first_name, email, country)
            VALUES 
                ($1, $2, $3, $4)
            RETURNING 
                supporter_id, last_name, first_name, email, country;
            "#,
        )
        .bind(supporter.last_name)
        .bind(supporter.first_name)
        .bind(supporter.email)
        .bind(supporter.country)
        .fetch_one(&self.db)
        .await?;

        Ok(Some(entity::Supporter {
            supporter_id: row.supporter_id,
            last_name: row.last_name,
            first_name: row.first_name,
            email: row.email,
            country: row.country,
        }))
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
                supporter_id = $5
            RETURNING 
                supporter_id, last_name, first_name, email, country;
            "#,
        )
        .bind(supporter.last_name)
        .bind(supporter.first_name)
        .bind(supporter.email)
        .bind(supporter.country)
        .bind(supporter.supporter_id)
        .fetch_one(&self.db)
        .await?;

        Ok(Some(entity::Supporter {
            supporter_id: row.supporter_id,
            last_name: row.last_name,
            first_name: row.first_name,
            email: row.email,
            country: row.country,
        }))
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
        login_id: String,
        password: String,
    ) -> Result<Option<entity::Supporter>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::GetSupporter>(
            r#"
            SELECT 
                supporter_id, last_name, first_name, email, country
            FROM 
                supporters
            WHERE 
                login_id = $1 AND password = $2;
            "#,
        )
        .bind(login_id)
        .bind(password)
        .fetch_one(&self.db)
        .await?;

        Ok(Some(entity::Supporter {
            supporter_id: row.supporter_id,
            last_name: row.last_name,
            first_name: row.first_name,
            email: row.email,
            country: row.country,
        }))
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
                p.protagonist_id = $1;
            "#,
        )
        .bind(i64::try_from(id).unwrap())
        .fetch_all(&self.db)
        .await?;

        let protagonist_supporters = rows
            .iter()
            .map(|row| entity::ProtagonistSupporter {
                supporter_id: row.supporter_id,
                last_name: row.last_name.clone(),
                first_name: row.first_name.clone(),
                country: row.country.clone(),
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

        Ok(Some(entity::ProtagonistSupporterRelation {
            protagonist_supporter_id: row.protagonist_supporter_id,
        }))
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
