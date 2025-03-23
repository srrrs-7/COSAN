use crate::domain::interface;
use crate::router::request;
use crate::router::response;

use super::entity;

#[derive(Clone)]
pub struct CosanService<U, W, UW>
where
    U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
    W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
    UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
{
    user_repository: U,
    word_repository: W,
    user_word_repository: UW,
}

impl<
        U: interface::UserRepositoryTrait + Clone + Send + Sync + 'static,
        W: interface::WordRepositoryTrait + Clone + Send + Sync + 'static,
        UW: interface::UserWordRepositoryTrait + Clone + Send + Sync + 'static,
    > CosanService<U, W, UW>
{
    pub fn new(user_repository: U, word_repository: W, user_word_repository: UW) -> Self {
        Self {
            user_repository,
            word_repository,
            user_word_repository,
        }
    }

    pub async fn get_user(&self, id: i64) -> Result<response::GetUserResponse, anyhow::Error> {
        let user_id = entity::UserId::new(id);

        let user = self.user_repository.get_user(user_id.value()).await?;
        match user {
            Some(user) => Ok(response::GetUserResponse {
                user_id: u64::try_from(user.user_id.value()).unwrap(),
                user_last_name: user.last_name.value().to_string(),
                user_first_name: user.first_name.value().to_string(),
                user_email: user.email.value().to_string(),
                user_country: user.country.value().to_string(),
            }),
            None => Err(anyhow::anyhow!("User not found")),
        }
    }

    pub async fn create_user(
        &self,
        request: request::CreateUserRequest,
    ) -> Result<response::CreateUserResponse, anyhow::Error> {
        let last_name = entity::LastName::new(request.last_name.as_str());
        let first_name = entity::FirstName::new(request.first_name.as_str());
        let login_id = entity::LoginId::new(request.login_id.as_str());
        let hashed_password = entity::Password::new(request.password.as_str())
            .hash()
            .await?;
        let email = entity::Email::new(request.email.as_str());
        let country = entity::Country::new(request.country.as_str());

        let result = self
            .user_repository
            .create_user(
                last_name.value(),
                first_name.value(),
                login_id.value(),
                hashed_password.value(),
                email.value(),
                country.value(),
            )
            .await?;

        match result {
            Some(user) => Ok(response::CreateUserResponse {
                user_id: user.user_id.value() as u64,
                user_last_name: user.last_name.value().to_string(),
                user_first_name: user.first_name.value().to_string(),
                user_email: user.email.value().to_string(),
                user_country: user.country.value().to_string(),
            }),
            None => Err(anyhow::anyhow!("User not created")),
        }
    }

    pub async fn update_user(
        &self,
        request: request::UpdateUserRequest,
    ) -> Result<response::UpdateUserResponse, anyhow::Error> {
        let user_id = entity::UserId::new(request.user_id);
        let last_name = entity::LastName::new(request.last_name.as_str());
        let first_name = entity::FirstName::new(request.first_name.as_str());
        let login_id = entity::LoginId::new(request.login_id.as_str());
        let hashed_password = entity::Password::new(request.password.as_str())
            .hash()
            .await?;
        let email = entity::Email::new(request.email.as_str());
        let country = entity::Country::new(request.country.as_str());

        let user = self
            .user_repository
            .update_user(
                user_id.value(),
                last_name.value(),
                first_name.value(),
                login_id.value(),
                hashed_password.value(),
                email.value(),
                country.value(),
            )
            .await?;

        match user {
            Some(user) => Ok(response::UpdateUserResponse {
                user_id: user.user_id.value() as u64,
                user_last_name: user.last_name.value().to_string(),
                user_first_name: user.first_name.value().to_string(),
                user_email: user.email.value().to_string(),
                user_country: user.country.value().to_string(),
            }),
            None => Err(anyhow::anyhow!("User not updated")),
        }
    }

    pub async fn delete_user(
        &self,
        id: i64,
    ) -> Result<response::DeleteUserResponse, anyhow::Error> {
        let user_id = entity::UserId::new(id);

        let result = self.user_repository.delete_user(user_id.value()).await?;
        match result {
            Some(_) => Ok(response::DeleteUserResponse {
                status: "success".to_string(),
            }),
            None => Err(anyhow::anyhow!("User not deleted")),
        }
    }

    pub async fn get_user_by_login_id_and_password(
        &self,
        login_id: String,
        password: String,
    ) -> Result<response::GetUserResponse, anyhow::Error> {
        let login_id = entity::LoginId::new(login_id.as_str());
        let hashed_password = entity::Password::new(password.as_str()).hash().await?;

        let user = self
            .user_repository
            .get_user_by_login_id_and_password(login_id.value(), hashed_password.clone().value())
            .await?;

        match user {
            Some(user) => {
                let valid = user.password.verify(hashed_password.value()).await?;
                if !valid {
                    return Err(anyhow::anyhow!("Invalid password"));
                }

                Ok(response::GetUserResponse {
                    user_id: user.user_id.value() as u64,
                    user_last_name: user.last_name.value().to_string(),
                    user_first_name: user.first_name.value().to_string(),
                    user_email: user.email.value().to_string(),
                    user_country: user.country.value().to_string(),
                })
            }
            None => Err(anyhow::anyhow!("User not found")),
        }
    }

    pub async fn get_word(&self, id: i64) -> Result<response::GetWordResponse, anyhow::Error> {
        let word_id = entity::WordId::new(id);

        let word = self.word_repository.get_word(word_id.value()).await?;
        match word {
            Some(word) => Ok(response::GetWordResponse {
                word_id: word.word_id.value() as u64,
                word: word.word.value().to_string(),
            }),
            None => Err(anyhow::anyhow!("Word not found")),
        }
    }

    pub async fn create_word(
        &self,
        request: request::CreateWordRequest,
    ) -> Result<response::CreateWordResponse, anyhow::Error> {
        let word = entity::WordString::new(request.word.as_str());

        let word = self.word_repository.create_word(word.value()).await?;

        match word {
            Some(word) => Ok(response::CreateWordResponse {
                word_id: word.word_id.value() as u64,
                word: word.word.value().to_string(),
            }),
            None => Err(anyhow::anyhow!("Word not created")),
        }
    }

    pub async fn update_word(
        &self,
        request: request::UpdateWordRequest,
    ) -> Result<response::UpdateWordResponse, anyhow::Error> {
        let word_id = entity::WordId::new(request.word_id as i64);
        let word = entity::WordString::new(request.word.as_str());

        let word = self
            .word_repository
            .update_word(word_id.value(), word.value())
            .await?;

        match word {
            Some(word) => Ok(response::UpdateWordResponse {
                word_id: word.word_id.value() as u64,
                word: word.word.value().to_string(),
            }),
            None => Err(anyhow::anyhow!("Word not updated")),
        }
    }

    pub async fn delete_word(
        &self,
        id: i64,
    ) -> Result<response::DeleteWordResponse, anyhow::Error> {
        let result = self.word_repository.delete_word(id).await?;
        match result {
            Some(_) => Ok(response::DeleteWordResponse {
                status: "success".to_string(),
            }),
            None => Err(anyhow::anyhow!("Word not deleted")),
        }
    }

    pub async fn get_user_word_by_user_id_and_word_id(
        &self,
        request: request::GetUserWordRequest,
    ) -> Result<response::GetUserWordResponse, anyhow::Error> {
        let user_id = entity::UserId::new(request.user_id as i64);
        let word_id = entity::WordId::new(request.word_id as i64);

        let user_word = self
            .user_word_repository
            .get_user_word_by_user_id_and_word_id(user_id.value(), word_id.value())
            .await?;

        match user_word {
            Some(user_word) => Ok(response::GetUserWordResponse {
                user_word_id: user_word.user_word_id.value() as u64,
                user_id: user_word.user_id.value() as u64,
                last_name: user_word.last_name.value().to_string(),
                first_name: user_word.first_name.value().to_string(),
                email: user_word.email.value().to_string(),
                country: user_word.country.value().to_string(),
                word_id: user_word.word_id.value() as u64,
                word: user_word.word.value().to_string(),
                created_at: user_word.created_at.value().to_string(),
            }),
            None => Err(anyhow::anyhow!("User word not found")),
        }
    }

    pub async fn get_user_word_by_word_id(
        &self,
        request: request::GetUserWordRequest,
    ) -> Result<Vec<response::GetUserWordResponse>, anyhow::Error> {
        let word_id = entity::WordId::new(request.word_id as i64);

        let user_words = self
            .user_word_repository
            .get_user_word_by_word_id(word_id.value())
            .await?;

        match user_words {
            Some(user_words) => Ok(user_words
                .into_iter()
                .map(|user_word| response::GetUserWordResponse {
                    user_word_id: user_word.user_word_id.value() as u64,
                    user_id: user_word.user_id.value() as u64,
                    last_name: user_word.last_name.value().to_string(),
                    first_name: user_word.first_name.value().to_string(),
                    email: user_word.email.value().to_string(),
                    country: user_word.country.value().to_string(),
                    word_id: user_word.word_id.value() as u64,
                    word: user_word.word.value().to_string(),
                    created_at: user_word.created_at.value().to_string(),
                })
                .collect()),
            None => Err(anyhow::anyhow!("User word not found")),
        }
    }

    pub async fn get_user_word_by_user_id(
        &self,
        request: request::GetUserWordRequest,
    ) -> Result<Vec<response::GetUserWordResponse>, anyhow::Error> {
        let user_id = entity::UserId::new(request.user_id as i64);

        let user_words = self
            .user_word_repository
            .get_user_word_by_user_id(user_id.value())
            .await?;

        match user_words {
            Some(user_words) => Ok(user_words
                .into_iter()
                .map(|user_word| response::GetUserWordResponse {
                    user_word_id: user_word.user_word_id.value() as u64,
                    user_id: user_word.user_id.value() as u64,
                    last_name: user_word.last_name.value().to_string(),
                    first_name: user_word.first_name.value().to_string(),
                    email: user_word.email.value().to_string(),
                    country: user_word.country.value().to_string(),
                    word_id: user_word.word_id.value() as u64,
                    word: user_word.word.value().to_string(),
                    created_at: user_word.created_at.value().to_string(),
                })
                .collect()),
            None => Err(anyhow::anyhow!("User word not found")),
        }
    }

    pub async fn create_user_word(
        &self,
        request: request::CreateUserWordRequest,
    ) -> Result<response::CreateUserWordRelationResponse, anyhow::Error> {
        let user_id = entity::UserId::new(request.user_id as i64);
        let word_id = entity::WordId::new(request.word_id as i64);

        let user_word = self
            .user_word_repository
            .create_user_word(user_id.value(), word_id.value())
            .await?;

        match user_word {
            Some(user_word) => Ok(response::CreateUserWordRelationResponse {
                user_id: user_word.user_id.value() as u64,
                word_id: user_word.word_id.value() as u64,
                created_at: user_word.created_at.value().to_string(),
            }),
            None => Err(anyhow::anyhow!("User word relation not created")),
        }
    }

    pub async fn delete_user_word(
        &self,
        id: i64,
    ) -> Result<response::DeleteUserWordResponse, anyhow::Error> {
        let result = self.user_word_repository.delete_user_word(id).await?;
        match result {
            Some(_) => Ok(response::DeleteUserWordResponse {
                status: "success".to_string(),
            }),
            None => Err(anyhow::anyhow!("User word not deleted")),
        }
    }
}
