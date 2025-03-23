use crate::domain::entity;
use crate::domain::interface;
use crate::driver::model;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use sqlx;
use sqlx::Pool;

#[derive(Clone)]
pub struct UserRepository {
    pool: sqlx::PgPool,
}

#[async_trait]
impl interface::UserRepositoryTrait for UserRepository {
    fn new(pool: Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }
    async fn get_user(&self, user_id: i64) -> Result<Option<entity::User>, sqlx::Error> {
        let record = sqlx::query_as::<_, model::GetUser>(
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
        .fetch_one(&self.pool)
        .await?;

        if !record.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::User::new(
            entity::UserId::new(record.user_id),
            entity::LastName::new(record.last_name.as_str()),
            entity::FirstName::new(record.first_name.as_str()),
            entity::LoginId::new(record.login_id.as_str()),
            entity::PasswordHash::new(record.password.as_str()),
            entity::Email::new(record.email.as_str()),
            entity::Country::new(record.country.as_str()),
        )))
    }

    async fn create_user(
        &self,
        last_name: &str,
        first_name: &str,
        login_id: &str,
        password: &str,
        email: &str,
        country: &str,
    ) -> Result<Option<entity::User>, sqlx::Error> {
        let record = sqlx::query_as::<_, model::CreateUser>(
            r#"
            INSERT INTO 
                users (last_name, first_name, login_id, password, email, country)
            VALUES 
                ($1, $2, $3, $4, $5, $6)
            RETURNING 
                user_id, last_name, first_name, login_id, password, email, country;
            "#,
        )
        .bind(last_name)
        .bind(first_name)
        .bind(login_id)
        .bind(password)
        .bind(email)
        .bind(country)
        .fetch_one(&self.pool)
        .await?;

        if !record.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::User::new(
            entity::UserId::new(record.user_id),
            entity::LastName::new(record.last_name.as_str()),
            entity::FirstName::new(record.first_name.as_str()),
            entity::LoginId::new(record.login_id.as_str()),
            entity::PasswordHash::new(record.password.as_str()),
            entity::Email::new(record.email.as_str()),
            entity::Country::new(record.country.as_str()),
        )))
    }

    async fn update_user(
        &self,
        user_id: i64,
        last_name: &str,
        first_name: &str,
        login_id: &str,
        password: &str,
        email: &str,
        country: &str,
    ) -> Result<Option<entity::User>, sqlx::Error> {
        let record = sqlx::query_as::<_, model::UpdateUser>(
            r#"
            UPDATE users
                SET last_name = $1, first_name = $2, login_id = $3, password = $4, email = $5, country = $6
            WHERE 
                user_id = $7
            RETURNING 
                user_id, last_name, first_name, login_id, password, email, country;
            "#,
        )
        .bind(last_name)
        .bind(first_name)
        .bind(login_id)
        .bind(password)
        .bind(email)
        .bind(country)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        if !record.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::User::new(
            entity::UserId::new(record.user_id),
            entity::LastName::new(record.last_name.as_str()),
            entity::FirstName::new(record.first_name.as_str()),
            entity::LoginId::new(record.login_id.as_str()),
            entity::PasswordHash::new(record.password.as_str()),
            entity::Email::new(record.email.as_str()),
            entity::Country::new(record.country.as_str()),
        )))
    }

    async fn delete_user(&self, id: i64) -> Result<Option<()>, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM 
                users
            WHERE 
                user_id = $1;
            "#,
        )
        .bind(i64::try_from(id).unwrap())
        .execute(&self.pool)
        .await?;

        Ok(Some(()))
    }

    async fn get_user_by_login_id_and_password(
        &self,
        login_id: &str,
        hashed_password: &str,
    ) -> Result<Option<entity::User>, sqlx::Error> {
        let record = sqlx::query_as::<_, model::GetUser>(
            r#"
            SELECT 
                user_id, last_name, first_name, login_id, password, email, country
            FROM 
                users
            WHERE 
                login_id = $1 
                AND password = $2;
            "#,
        )
        .bind(login_id)
        .bind(hashed_password)
        .fetch_one(&self.pool)
        .await?;

        if !record.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::User::new(
            entity::UserId::new(record.user_id),
            entity::LastName::new(record.last_name.as_str()),
            entity::FirstName::new(record.first_name.as_str()),
            entity::LoginId::new(record.login_id.as_str()),
            entity::PasswordHash::new(record.password.as_str()),
            entity::Email::new(record.email.as_str()),
            entity::Country::new(record.country.as_str()),
        )))
    }
}

#[derive(Clone)]
pub struct WordRepository {
    pool: Pool<sqlx::Postgres>,
}

#[async_trait]
impl interface::WordRepositoryTrait for WordRepository {
    fn new(pool: Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }
    async fn get_word(&self, word_id: i64) -> Result<Option<entity::Word>, sqlx::Error> {
        let record = sqlx::query_as::<_, model::GetWord>(
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
        .fetch_one(&self.pool)
        .await?;

        if !record.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Word::new(
            entity::WordId::new(record.word_id),
            entity::WordString::new(record.word.as_str()),
        )))
    }

    async fn create_word(&self, word: &str) -> Result<Option<entity::Word>, sqlx::Error> {
        let record = sqlx::query_as::<_, model::CreateWord>(
            r#"
            INSERT INTO 
                words (word)
            VALUES 
                ($1)
            RETURNING 
                word_id, word;
            "#,
        )
        .bind(word)
        .fetch_one(&self.pool)
        .await?;

        if !record.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Word::new(
            entity::WordId::new(record.word_id),
            entity::WordString::new(record.word.as_str()),
        )))
    }

    async fn update_word(
        &self,
        word_id: i64,
        word: &str,
    ) -> Result<Option<entity::Word>, sqlx::Error> {
        let record = sqlx::query_as::<_, model::UpdateWord>(
            r#"
            UPDATE words
                SET word = $1
            WHERE 
                word_id = $2
            RETURNING 
                word_id, word;
            "#,
        )
        .bind(word)
        .bind(word_id)
        .fetch_one(&self.pool)
        .await?;

        if !record.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::Word::new(
            entity::WordId::new(record.word_id),
            entity::WordString::new(record.word.as_str()),
        )))
    }

    async fn delete_word(&self, id: i64) -> Result<Option<()>, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM 
                words
            WHERE 
                word_id = $1;
            "#,
        )
        .bind(i64::try_from(id).unwrap())
        .execute(&self.pool)
        .await?;

        Ok(Some(()))
    }
}

#[derive(Clone)]
pub struct UserWordRepository {
    pool: Pool<sqlx::Postgres>,
}

#[async_trait]
impl interface::UserWordRepositoryTrait for UserWordRepository {
    fn new(pool: Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }
    async fn get_user_word_by_user_id_and_word_id(
        &self,
        user_id: i64,
        word_id: i64,
    ) -> Result<Option<entity::UserWord>, sqlx::Error> {
        let record = sqlx::query_as::<_, model::GetUserWord>(
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
        .bind(user_id)
        .bind(word_id)
        .fetch_one(&self.pool)
        .await?;

        if record.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::UserWord::new(
            entity::UserWordId::new(record.user_word_id),
            entity::UserId::new(record.user_id),
            entity::LastName::new(record.last_name.as_str()),
            entity::FirstName::new(record.first_name.as_str()),
            entity::Email::new(record.email.as_str()),
            entity::Country::new(record.country.as_str()),
            entity::WordId::new(record.word_id),
            entity::WordString::new(record.word.as_str()),
            entity::CreatedAt::new(
                DateTime::parse_from_rfc3339(record.created_at.as_str())
                    .expect("Invalid date")
                    .with_timezone(&Utc),
            ),
        )))
    }

    async fn get_user_word_by_user_id(
        &self,
        user_id: i64,
    ) -> Result<Option<Vec<entity::UserWord>>, sqlx::Error> {
        let records = sqlx::query_as::<_, model::GetUserWord>(
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
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        if records.is_empty() {
            return Ok(None);
        }

        let user_words = records
            .into_iter()
            .map(|record| {
                entity::UserWord::new(
                    entity::UserWordId::new(record.user_word_id),
                    entity::UserId::new(record.user_id),
                    entity::LastName::new(record.last_name.as_str()),
                    entity::FirstName::new(record.first_name.as_str()),
                    entity::Email::new(record.email.as_str()),
                    entity::Country::new(record.country.as_str()),
                    entity::WordId::new(record.word_id),
                    entity::WordString::new(record.word.as_str()),
                    entity::CreatedAt::new(
                        DateTime::parse_from_rfc3339(record.created_at.as_str())
                            .expect("Invalid date")
                            .with_timezone(&Utc),
                    ),
                )
            })
            .collect();

        Ok(Some(user_words))
    }

    async fn get_user_word_by_word_id(
        &self,
        word_id: i64,
    ) -> Result<Option<Vec<entity::UserWord>>, sqlx::Error> {
        let records = sqlx::query_as::<_, model::GetUserWord>(
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
        .bind(word_id)
        .fetch_all(&self.pool)
        .await?;

        if records.is_empty() {
            return Ok(None);
        }

        let user_words = records
            .into_iter()
            .map(|record| {
                entity::UserWord::new(
                    entity::UserWordId::new(record.user_word_id),
                    entity::UserId::new(record.user_id),
                    entity::LastName::new(record.last_name.as_str()),
                    entity::FirstName::new(record.first_name.as_str()),
                    entity::Email::new(record.email.as_str()),
                    entity::Country::new(record.country.as_str()),
                    entity::WordId::new(record.word_id),
                    entity::WordString::new(record.word.as_str()),
                    entity::CreatedAt::new(
                        DateTime::parse_from_rfc3339(record.created_at.as_str())
                            .expect("Invalid date")
                            .with_timezone(&Utc),
                    ),
                )
            })
            .collect();

        Ok(Some(user_words))
    }

    async fn create_user_word(
        &self,
        user_id: i64,
        word_id: i64,
    ) -> Result<Option<entity::UserWordRelation>, sqlx::Error> {
        let record = sqlx::query_as::<_, model::GetUserWordRelation>(
            r#"
            INSERT INTO 
                user_words (user_id, word_id)
            VALUES 
                ($1, $2)
            RETURNING 
                user_id, word_id;
            "#,
        )
        .bind(user_id)
        .bind(word_id)
        .fetch_one(&self.pool)
        .await?;

        if !record.is_valid() {
            return Ok(None);
        }

        Ok(Some(entity::UserWordRelation::new(
            entity::UserId::new(record.user_id),
            entity::WordId::new(record.word_id),
            entity::CreatedAt::new(
                DateTime::parse_from_rfc3339(record.created_at.as_str())
                    .expect("Invalid date")
                    .with_timezone(&Utc),
            ),
        )))
    }

    async fn delete_user_word(&self, id: i64) -> Result<Option<()>, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM 
                user_words
            WHERE 
                user_word_id = $1;
            "#,
        )
        .bind(i64::try_from(id).unwrap())
        .execute(&self.pool)
        .await?;

        Ok(Some(()))
    }
}
