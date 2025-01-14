use crate::domain::entity;
use crate::driver::model;
use sqlx;

#[derive(Clone)]
pub struct CosanRepository {
    db: sqlx::PgPool,
}

impl CosanRepository {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }

    pub async fn get_user(&self, user_id: i64) -> Result<Option<entity::User>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::GetUser>(
            r#"
            SELECT 
                user_id, last_name, first_name, login_id, password, email, country
            FROM 
                users
            WHERE 
                user_id = $1;
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::User::new(
            row.user_id,
            row.last_name,
            row.first_name,
            row.login_id,
            row.password,
            row.email,
            row.country,
        )))
    }

    pub async fn create_user(
        &self,
        model: model::CreateUser,
    ) -> Result<Option<entity::User>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::CreateUser>(
            r#"
            INSERT INTO 
                users (last_name, first_name, login_id, password, email, country)
            VALUES 
                ($1, $2, $3, $4, $5, $6)
            RETURNING 
                user_id, last_name, first_name, login_id, password, email, country;
            "#,
        )
        .bind(model.last_name)
        .bind(model.first_name)
        .bind(model.login_id)
        .bind(model.password)
        .bind(model.email)
        .bind(model.country)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::User::new(
            row.user_id,
            row.last_name,
            row.first_name,
            row.login_id,
            row.password,
            row.email,
            row.country,
        )))
    }

    pub async fn update_user(
        &self,
        model: model::UpdateUser,
    ) -> Result<Option<entity::User>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::UpdateUser>(
            r#"
            UPDATE users
                SET last_name = $1, first_name = $2, login_id = $3, password = $4, email = $5, country = $6
            WHERE 
                user_id = $7
            RETURNING 
                user_id, last_name, first_name, login_id, password, email, country;
            "#,
        )
        .bind(model.last_name)
        .bind(model.first_name)
        .bind(model.login_id)
        .bind(model.password)
        .bind(model.email)
        .bind(model.country)
        .bind(model.user_id)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::User::new(
            row.user_id,
            row.last_name,
            row.first_name,
            row.login_id,
            row.password,
            row.email,
            row.country,
        )))
    }

    pub async fn delete_user(&self, id: i64) -> Result<Option<()>, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM 
                users
            WHERE 
                user_id = $1;
            "#,
        )
        .bind(i64::try_from(id).unwrap())
        .execute(&self.db)
        .await?;

        Ok(Some(()))
    }

    pub async fn get_user_by_login_id_and_password(
        &self,
        login_id: &str,
    ) -> Result<Option<entity::User>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::GetUser>(
            r#"
            SELECT 
                user_id, last_name, first_name, login_id, password, email, country
            FROM 
                users
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

        Ok(Some(entity::User::new(
            row.user_id,
            row.last_name,
            row.first_name,
            row.login_id,
            row.password,
            row.email,
            row.country,
        )))
    }

    pub async fn get_word(&self, word_id: i64) -> Result<Option<entity::Word>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::GetWord>(
            r#"
            SELECT 
                word_id, word
            FROM 
                words
            WHERE 
                word_id = $1;
            "#,
        )
        .bind(word_id)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Word::new(
            u64::try_from(row.word_id).unwrap(),
            row.word,
        )))
    }

    pub async fn create_word(
        &self,
        model: model::CreateWord,
    ) -> Result<Option<entity::Word>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::CreateWord>(
            r#"
            INSERT INTO 
                words (word)
            VALUES 
                ($1)
            RETURNING 
                word_id, word;
            "#,
        )
        .bind(model.word)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Word::new(
            u64::try_from(row.word_id).unwrap(),
            row.word,
        )))
    }

    pub async fn update_word(
        &self,
        model: model::UpdateWord,
    ) -> Result<Option<entity::Word>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::UpdateWord>(
            r#"
            UPDATE words
                SET word = $1
            WHERE 
                word_id = $2
            RETURNING 
                word_id, word;
            "#,
        )
        .bind(model.word)
        .bind(model.word_id)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Word::new(
            u64::try_from(row.word_id).unwrap(),
            row.word,
        )))
    }

    pub async fn delete_word(&self, id: i64) -> Result<Option<()>, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM 
                words
            WHERE 
                word_id = $1;
            "#,
        )
        .bind(i64::try_from(id).unwrap())
        .execute(&self.db)
        .await?;

        Ok(Some(()))
    }

    pub async fn get_user_word_by_user_id_and_word_id(
        &self,
        model: model::GetUserWordId,
    ) -> Result<Option<entity::UserWord>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::GetUserWord>(
            r#"
            SELECT 
                uw.user_word_id,
                u.user_id,
                u.last_name,
                u.first_name,
                u.email,
                u.country,
                w.word_id,
                w.word,
                uw.created_at
            FROM 
                user_words AS uw
            INNER JOIN
                users AS u 
                    ON uw.user_id = u.user_id
            INNER JOIN
                words AS w 
                    ON uw.word_id = w.word_id
            WHERE 
                uw.user_id = $1
                AND uw.word_id = $2
            "#,
        )
        .bind(model.user_id)
        .bind(model.word_id)
        .fetch_one(&self.db)
        .await?;

        if row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::UserWord::new(
            u64::try_from(row.user_word_id).unwrap(),
            u64::try_from(row.user_id).unwrap(),
            row.last_name,
            row.first_name,
            row.email,
            row.country,
            u64::try_from(row.word_id).unwrap(),
            row.word,
            row.created_at,
        )))
    }

    pub async fn get_user_word_by_user_id(
        &self,
        model: model::GetUserWordId,
    ) -> Result<Option<Vec<entity::UserWord>>, sqlx::Error> {
        let rows = sqlx::query_as::<_, model::GetUserWord>(
            r#"
            SELECT 
                uw.user_word_id,
                u.user_id,
                u.last_name,
                u.first_name,
                u.email,
                u.country,
                w.word_id,
                w.word,
                uw.created_at
            FROM 
                user_words AS uw
            INNER JOIN
                users AS u 
                    ON uw.user_id = u.user_id
            INNER JOIN
                words AS w 
                    ON uw.word_id = w.word_id
            WHERE 
                uw.user_id = $1
            ORDER BY
                created_at ASC;
            "#,
        )
        .bind(model.user_id)
        .fetch_all(&self.db)
        .await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let user_words = rows
            .into_iter()
            .map(|row| {
                entity::UserWord::new(
                    u64::try_from(row.user_word_id).unwrap(),
                    u64::try_from(row.user_id).unwrap(),
                    row.last_name,
                    row.first_name,
                    row.email,
                    row.country,
                    u64::try_from(row.word_id).unwrap(),
                    row.word,
                    row.created_at,
                )
            })
            .collect();

        Ok(Some(user_words))
    }

    pub async fn get_user_word_by_word_id(
        &self,
        model: model::GetUserWordId,
    ) -> Result<Option<Vec<entity::UserWord>>, sqlx::Error> {
        let rows = sqlx::query_as::<_, model::GetUserWord>(
            r#"
            SELECT 
                uw.user_word_id,
                u.user_id,
                u.last_name,
                u.first_name,
                u.email,
                u.country,
                w.word_id,
                w.word,
                uw.created_at
            FROM 
                user_words AS uw
            INNER JOIN
                users AS u 
                    ON uw.user_id = u.user_id
            INNER JOIN
                words AS w 
                    ON uw.word_id = w.word_id
            WHERE 
                uw.word_id = $2
            ORDER BY
                created_at ASC;
            "#,
        )
        .bind(model.word_id)
        .fetch_all(&self.db)
        .await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let user_words = rows
            .into_iter()
            .map(|row| {
                entity::UserWord::new(
                    u64::try_from(row.user_word_id).unwrap(),
                    u64::try_from(row.user_id).unwrap(),
                    row.last_name,
                    row.first_name,
                    row.email,
                    row.country,
                    u64::try_from(row.word_id).unwrap(),
                    row.word,
                    row.created_at,
                )
            })
            .collect();

        Ok(Some(user_words))
    }

    pub async fn create_user_word(
        &self,
        model: model::CreateUserWord,
    ) -> Result<Option<entity::UserWordRelation>, sqlx::Error> {
        let row = sqlx::query_as::<_, model::GetUserWordRelation>(
            r#"
            INSERT INTO 
                user_words (user_id, word_id)
            VALUES 
                ($1, $2)
            RETURNING 
                user_id, word_id;
            "#,
        )
        .bind(model.user_id)
        .bind(model.word_id)
        .fetch_one(&self.db)
        .await?;

        if !row.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::UserWordRelation::new(
            u64::try_from(row.user_id).unwrap(),
            u64::try_from(row.word_id).unwrap(),
            row.created_at,
        )))
    }

    pub async fn delete_user_word(&self, id: i64) -> Result<Option<()>, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM 
                user_words
            WHERE 
                user_word_id = $1;
            "#,
        )
        .bind(i64::try_from(id).unwrap())
        .execute(&self.db)
        .await?;

        Ok(Some(()))
    }
}
