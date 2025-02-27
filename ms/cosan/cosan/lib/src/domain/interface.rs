use crate::domain::entity;
use crate::driver::model;
use sqlx;

pub trait UserRepositoryTrait: Clone + Send + Sync + 'static {
    async fn new(pool: sqlx::PgPool) -> Self;
    async fn get_user(&self, user_id: i64) -> Result<Option<entity::User>, sqlx::Error>;
    async fn create_user(
        &self,
        model: model::CreateUser,
    ) -> Result<Option<entity::User>, sqlx::Error>;
    async fn update_user(
        &self,
        model: model::UpdateUser,
    ) -> Result<Option<entity::User>, sqlx::Error>;
    async fn delete_user(&self, id: i64) -> Result<Option<()>, sqlx::Error>;
    async fn get_user_by_login_id_and_password(
        &self,
        login_id: &str,
    ) -> Result<Option<entity::User>, sqlx::Error>;
}

pub trait WordRepositoryTrait: Clone + Send + Sync + 'static {
    async fn new(pool: sqlx::PgPool) -> Self;
    async fn get_word(&self, word_id: i64) -> Result<Option<entity::Word>, sqlx::Error>;
    async fn create_word(
        &self,
        model: model::CreateWord,
    ) -> Result<Option<entity::Word>, sqlx::Error>;
    async fn update_word(
        &self,
        model: model::UpdateWord,
    ) -> Result<Option<entity::Word>, sqlx::Error>;
    async fn delete_word(&self, id: i64) -> Result<Option<()>, sqlx::Error>;
}

pub trait UserWordRepositoryTrait: Clone + Send + Sync + 'static {
    async fn new(pool: sqlx::PgPool) -> Self;
    async fn get_user_word_by_user_id_and_word_id(
        &self,
        model: model::GetUserWordId,
    ) -> Result<Option<entity::UserWord>, sqlx::Error>;
    async fn get_user_word_by_user_id(
        &self,
        model: model::GetUserWordId,
    ) -> Result<Option<Vec<entity::UserWord>>, sqlx::Error>;
    async fn get_user_word_by_word_id(
        &self,
        model: model::GetUserWordId,
    ) -> Result<Option<Vec<entity::UserWord>>, sqlx::Error>;
    async fn create_user_word(
        &self,
        model: model::CreateUserWord,
    ) -> Result<Option<entity::UserWordRelation>, sqlx::Error>;
    async fn delete_user_word(&self, id: i64) -> Result<Option<()>, sqlx::Error>;
}
