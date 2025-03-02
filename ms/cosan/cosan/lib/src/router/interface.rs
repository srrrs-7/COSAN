use crate::router::request;
use crate::router::response;

pub trait CosanServiceTrait {
    fn get_user(&self, id: i64) -> Result<response::GetUserResponse, anyhow::Error>;
    fn create_user(
        &self,
        request: request::CreateUserRequest,
    ) -> Result<response::CreateUserResponse, anyhow::Error>;
    fn update_user(
        &self,
        request: request::UpdateUserRequest,
    ) -> Result<response::UpdateUserResponse, anyhow::Error>;
    fn delete_user(&self, id: i64) -> Result<(), anyhow::Error>;
    fn get_user_by_login_id_and_password(
        &self,
        login_request: request::GetUserRequest,
    ) -> Result<response::GetUserResponse, anyhow::Error>;
    fn get_word(&self, id: i64) -> Result<response::GetWordResponse, anyhow::Error>;
    fn create_word(
        &self,
        request: request::CreateWordRequest,
    ) -> Result<response::CreateWordResponse, anyhow::Error>;
    fn update_word(
        &self,
        request: request::UpdateWordRequest,
    ) -> Result<response::UpdateWordResponse, anyhow::Error>;
    fn delete_word(&self, id: i64) -> Result<(), anyhow::Error>;
    fn get_user_word_by_user_id_and_word_id(
        &self,
        request: request::GetUserWordRequest,
    ) -> Result<response::GetUserWordResponse, anyhow::Error>;
    fn get_user_word_by_word_id(
        &self,
        request: request::GetUserWordRequest,
    ) -> Result<Vec<response::GetUserWordResponse>, anyhow::Error>;
    fn get_user_word_by_user_id(
        &self,
        request: request::GetUserWordRequest,
    ) -> Result<Vec<response::GetUserWordResponse>, anyhow::Error>;
    fn create_user_word(
        &self,
        request: request::CreateUserWordRequest,
    ) -> Result<response::CreateUserWordRelationResponse, anyhow::Error>;
    fn delete_user_word(&self, id: i64) -> Result<(), anyhow::Error>;
}
