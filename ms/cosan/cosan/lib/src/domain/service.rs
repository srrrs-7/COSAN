use crate::{
    driver::{model, repository},
    router::{request, response},
};

#[derive(Clone)]
pub struct CosanService {
    repository: repository::CosanRepository,
}

impl CosanService {
    pub fn new(repository: repository::CosanRepository) -> Self {
        Self { repository }
    }

    pub async fn get_user(&self, id: i64) -> Result<response::GetUserResponse, anyhow::Error> {
        let user = self.repository.get_user(id).await?;
        match user {
            Some(user) => Ok(response::GetUserResponse {
                user_id: u64::try_from(user.user_id).unwrap(),
                user_last_name: user.last_name,
                user_first_name: user.first_name,
                user_email: user.email,
                user_country: user.country,
            }),
            None => Err(anyhow::anyhow!("User not found")),
        }
    }

    pub async fn create_user(
        &self,
        request: request::CreateUserRequest,
    ) -> Result<response::CreateUserResponse, anyhow::Error> {
        // protagonist_id is set to -1 because it is auto-incremented in the database
        let result = self
            .repository
            .create_user(
                model::CreateUser::new(
                    -1,
                    request.last_name,
                    request.first_name,
                    request.login_id,
                    request.password,
                    request.email,
                    request.country,
                )
                .convert_hash_password()
                .await?,
            )
            .await?;

        match result {
            Some(user) => Ok(response::CreateUserResponse {
                user_id: u64::try_from(user.user_id).unwrap(),
                user_last_name: user.last_name,
                user_first_name: user.first_name,
                user_email: user.email,
                user_country: user.country,
            }),
            None => Err(anyhow::anyhow!("User not created")),
        }
    }

    pub async fn update_user(
        &self,
        request: request::UpdateUserRequest,
    ) -> Result<response::UpdateUserResponse, anyhow::Error> {
        let user = self
            .repository
            .update_user(
                model::UpdateUser::new(
                    request.user_id,
                    request.last_name,
                    request.first_name,
                    request.login_id,
                    request.password,
                    request.email,
                    request.country,
                )
                .convert_hash_password()
                .await?,
            )
            .await?;

        match user {
            Some(user) => Ok(response::UpdateUserResponse {
                user_id: u64::try_from(user.user_id).unwrap(),
                user_last_name: user.last_name,
                user_first_name: user.first_name,
                user_email: user.email,
                user_country: user.country,
            }),
            None => Err(anyhow::anyhow!("User not updated")),
        }
    }

    pub async fn delete_user(&self, id: i64) -> Result<(), anyhow::Error> {
        let result = self.repository.delete_user(id).await?;
        match result {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("User not deleted")),
        }
    }

    pub async fn get_user_by_login_id_and_password(
        &self,
        login_request: request::GetUserRequest,
    ) -> Result<response::GetUserResponse, anyhow::Error> {
        let user = self
            .repository
            .get_user_by_login_id_and_password(login_request.login_id.as_str())
            .await?;

        match user {
            Some(user) => {
                let valid = user
                    .verify_password(login_request.password.as_str())
                    .await
                    .map_err(|e| anyhow::anyhow!("Error verifying password: {}", e.to_string()))?;
                if !valid {
                    return Err(anyhow::anyhow!("Invalid password"));
                }

                Ok(response::GetUserResponse {
                    user_id: u64::try_from(user.user_id).unwrap(),
                    user_last_name: user.last_name,
                    user_first_name: user.first_name,
                    user_email: user.email,
                    user_country: user.country,
                })
            }
            None => Err(anyhow::anyhow!("User not found")),
        }
    }

    pub async fn get_word(&self, id: i64) -> Result<response::GetWordResponse, anyhow::Error> {
        let word = self.repository.get_word(id).await?;
        match word {
            Some(word) => Ok(response::GetWordResponse {
                word_id: u64::try_from(word.word_id).unwrap(),
                word: word.word,
            }),
            None => Err(anyhow::anyhow!("Word not found")),
        }
    }

    pub async fn create_word(
        &self,
        request: request::CreateWordRequest,
    ) -> Result<response::CreateWordResponse, anyhow::Error> {
        // support_id is set to -1 because it is auto-incremented in the database
        let word = self
            .repository
            .create_word(model::CreateWord::new(-1, request.word))
            .await?;

        match word {
            Some(word) => Ok(response::CreateWordResponse {
                word_id: u64::try_from(word.word_id).unwrap(),
                word: word.word,
            }),
            None => Err(anyhow::anyhow!("Word not created")),
        }
    }

    pub async fn update_word(
        &self,
        request: request::UpdateWordRequest,
    ) -> Result<response::UpdateWordResponse, anyhow::Error> {
        let word = self
            .repository
            .update_word(model::UpdateWord::new(
                i64::try_from(request.word_id).unwrap(),
                request.word,
            ))
            .await?;

        match word {
            Some(word) => Ok(response::UpdateWordResponse {
                word_id: u64::try_from(word.word_id).unwrap(),
                word: word.word,
            }),
            None => Err(anyhow::anyhow!("Word not updated")),
        }
    }

    pub async fn delete_word(&self, id: i64) -> Result<(), anyhow::Error> {
        let result = self.repository.delete_word(id).await?;
        match result {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("Word not deleted")),
        }
    }

    pub async fn get_user_word_by_user_id_and_word_id(
        &self,
        request: request::GetUserWordRequest,
    ) -> Result<response::GetUserWordResponse, anyhow::Error> {
        let user_word = self
            .repository
            .get_user_word_by_user_id_and_word_id(model::GetUserWordId::new(
                i64::try_from(request.user_id).unwrap(),
                i64::try_from(request.word_id).unwrap(),
            ))
            .await?;

        match user_word {
            Some(user_word) => Ok(response::GetUserWordResponse {
                user_word_id: u64::try_from(user_word.user_word_id).unwrap(),
                user_id: u64::try_from(user_word.user_id).unwrap(),
                last_name: user_word.last_name,
                first_name: user_word.first_name,
                email: user_word.email,
                country: user_word.country,
                word_id: u64::try_from(user_word.word_id).unwrap(),
                word: user_word.word,
                created_at: user_word.created_at,
            }),
            None => Err(anyhow::anyhow!("User word not found")),
        }
    }

    pub async fn get_user_word_by_word_id(
        &self,
        request: request::GetUserWordRequest,
    ) -> Result<Vec<response::GetUserWordResponse>, anyhow::Error> {
        let user_words = self
            .repository
            .get_user_word_by_word_id(model::GetUserWordId::new(
                -1,
                i64::try_from(request.word_id).unwrap(),
            ))
            .await?;

        match user_words {
            Some(user_words) => Ok(user_words
                .into_iter()
                .map(|user_word| response::GetUserWordResponse {
                    user_word_id: u64::try_from(user_word.user_word_id).unwrap(),
                    user_id: u64::try_from(user_word.user_id).unwrap(),
                    last_name: user_word.last_name,
                    first_name: user_word.first_name,
                    email: user_word.email,
                    country: user_word.country,
                    word_id: u64::try_from(user_word.word_id).unwrap(),
                    word: user_word.word,
                    created_at: user_word.created_at,
                })
                .collect()),
            None => Err(anyhow::anyhow!("User word not found")),
        }
    }

    pub async fn get_user_word_by_user_id(
        &self,
        request: request::GetUserWordRequest,
    ) -> Result<Vec<response::GetUserWordResponse>, anyhow::Error> {
        let user_words = self
            .repository
            .get_user_word_by_user_id(model::GetUserWordId::new(
                i64::try_from(request.user_id).unwrap(),
                -1,
            ))
            .await?;

        match user_words {
            Some(user_words) => Ok(user_words
                .into_iter()
                .map(|user_word| response::GetUserWordResponse {
                    user_word_id: u64::try_from(user_word.user_word_id).unwrap(),
                    user_id: u64::try_from(user_word.user_id).unwrap(),
                    last_name: user_word.last_name,
                    first_name: user_word.first_name,
                    email: user_word.email,
                    country: user_word.country,
                    word_id: u64::try_from(user_word.word_id).unwrap(),
                    word: user_word.word,
                    created_at: user_word.created_at,
                })
                .collect()),
            None => Err(anyhow::anyhow!("User word not found")),
        }
    }

    pub async fn create_user_word(
        &self,
        request: request::CreateUserWordRequest,
    ) -> Result<response::CreateUserWordRelationResponse, anyhow::Error> {
        let user_word = self
            .repository
            .create_user_word(model::CreateUserWord::new(
                i64::try_from(request.user_id).unwrap(),
                i64::try_from(request.word_id).unwrap(),
            ))
            .await?;

        match user_word {
            Some(user_word) => Ok(response::CreateUserWordRelationResponse {
                user_id: u64::try_from(user_word.user_id).unwrap(),
                word_id: u64::try_from(user_word.word_id).unwrap(),
                created_at: user_word.created_at,
            }),
            None => Err(anyhow::anyhow!("User word relation not created")),
        }
    }

    pub async fn delete_user_word(&self, id: i64) -> Result<(), anyhow::Error> {
        let result = self.repository.delete_user_word(id).await?;
        match result {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("User word not deleted")),
        }
    }
}
