use crate::domain::entity;
use async_trait::async_trait;
use sqlx;
use sqlx::Pool;

#[async_trait]
pub trait UserRepositoryTrait: Clone + Send + Sync + 'static {
    fn new(pool: Pool<sqlx::Postgres>) -> Self;

    async fn get_user(&self, user_id: i64) -> Result<Option<entity::User>, sqlx::Error>;

    async fn create_user(
        &self,
        last_name: &str,
        first_name: &str,
        login_id: &str,
        password: &str,
        email: &str,
        country: &str,
    ) -> Result<Option<entity::User>, sqlx::Error>;

    async fn update_user(
        &self,
        user_id: i64,
        last_name: &str,
        first_name: &str,
        login_id: &str,
        password: &str,
        email: &str,
        country: &str,
    ) -> Result<Option<entity::User>, sqlx::Error>;

    async fn delete_user(&self, id: i64) -> Result<Option<()>, sqlx::Error>;

    async fn get_user_by_login_id_and_password(
        &self,
        login_id: &str,
        hashed_password: &str,
    ) -> Result<Option<entity::User>, sqlx::Error>;
}

#[async_trait]
pub trait WordRepositoryTrait: Clone + Send + Sync + 'static {
    fn new(pool: Pool<sqlx::Postgres>) -> Self;

    async fn get_word(&self, word_id: i64) -> Result<Option<entity::Word>, sqlx::Error>;

    async fn create_word(&self, word: &str) -> Result<Option<entity::Word>, sqlx::Error>;

    async fn update_word(
        &self,
        word_id: i64,
        word: &str,
    ) -> Result<Option<entity::Word>, sqlx::Error>;

    async fn delete_word(&self, id: i64) -> Result<Option<()>, sqlx::Error>;
}

#[async_trait]
pub trait UserWordRepositoryTrait: Clone + Send + Sync + 'static {
    fn new(pool: Pool<sqlx::Postgres>) -> Self;

    async fn get_user_word_by_user_id_and_word_id(
        &self,
        user_id: i64,
        word_id: i64,
    ) -> Result<Option<entity::UserWord>, sqlx::Error>;

    async fn get_user_word_by_user_id(
        &self,
        user_id: i64,
    ) -> Result<Option<Vec<entity::UserWord>>, sqlx::Error>;

    async fn get_user_word_by_word_id(
        &self,
        word_id: i64,
    ) -> Result<Option<Vec<entity::UserWord>>, sqlx::Error>;

    async fn create_user_word(
        &self,
        user_id: i64,
        word_id: i64,
    ) -> Result<Option<entity::UserWordRelation>, sqlx::Error>;

    async fn delete_user_word(&self, id: i64) -> Result<Option<()>, sqlx::Error>;
}
