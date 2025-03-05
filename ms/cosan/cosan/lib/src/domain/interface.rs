use crate::domain::entity;
use crate::driver::model;
use async_trait;
use sqlx;
use sqlx::Pool;
use std::future::Future;

#[async_trait::async_trait]
pub trait UserRepositoryTrait: Clone + Send + Sync + 'static {
    fn new(pool: Pool<sqlx::Postgres>) -> Self;
    fn get_user(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Option<entity::User>, sqlx::Error>> + Send;
    fn create_user(
        &self,
        model: model::CreateUser,
    ) -> impl Future<Output = Result<Option<entity::User>, sqlx::Error>> + Send;
    fn update_user(
        &self,
        model: model::UpdateUser,
    ) -> impl Future<Output = Result<Option<entity::User>, sqlx::Error>> + Send;
    fn delete_user(&self, id: i64) -> impl Future<Output = Result<Option<()>, sqlx::Error>> + Send;
    fn get_user_by_login_id_and_password(
        &self,
        login_id: &str,
    ) -> impl Future<Output = Result<Option<entity::User>, sqlx::Error>> + Send;
}

#[async_trait::async_trait]
pub trait WordRepositoryTrait: Clone + Send + Sync + 'static {
    fn new(pool: Pool<sqlx::Postgres>) -> Self;
    fn get_word(
        &self,
        word_id: i64,
    ) -> impl Future<Output = Result<Option<entity::Word>, sqlx::Error>> + Send;
    fn create_word(
        &self,
        model: model::CreateWord,
    ) -> impl Future<Output = Result<Option<entity::Word>, sqlx::Error>> + Send;
    fn update_word(
        &self,
        model: model::UpdateWord,
    ) -> impl Future<Output = Result<Option<entity::Word>, sqlx::Error>> + Send;
    fn delete_word(&self, id: i64) -> impl Future<Output = Result<Option<()>, sqlx::Error>> + Send;
}

#[async_trait::async_trait]
pub trait UserWordRepositoryTrait: Clone + Send + Sync + 'static {
    fn new(pool: Pool<sqlx::Postgres>) -> Self;
    fn get_user_word_by_user_id_and_word_id(
        &self,
        model: model::GetUserWordId,
    ) -> impl Future<Output = Result<Option<entity::UserWord>, sqlx::Error>> + Send;
    fn get_user_word_by_user_id(
        &self,
        model: model::GetUserWordId,
    ) -> impl Future<Output = Result<Option<Vec<entity::UserWord>>, sqlx::Error>> + Send;
    fn get_user_word_by_word_id(
        &self,
        model: model::GetUserWordId,
    ) -> impl Future<Output = Result<Option<Vec<entity::UserWord>>, sqlx::Error>> + Send;
    fn create_user_word(
        &self,
        model: model::CreateUserWord,
    ) -> impl Future<Output = Result<Option<entity::UserWordRelation>, sqlx::Error>> + Send;
    fn delete_user_word(
        &self,
        id: i64,
    ) -> impl Future<Output = Result<Option<()>, sqlx::Error>> + Send;
}
