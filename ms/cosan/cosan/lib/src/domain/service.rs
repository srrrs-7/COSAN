use crate::{
    driver::{model, repository},
    router::{request, response},
};

#[derive(Clone)]
pub struct SupportService {
    repository: repository::SupportRepository,
}

impl SupportService {
    pub fn new(repository: repository::SupportRepository) -> Self {
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
        protagonist: request::CreateUserRequest,
    ) -> Result<response::CreateUserResponse, anyhow::Error> {
        // protagonist_id is set to -1 because it is auto-incremented in the database
        let result = self
            .repository
            .create_user(
                model::CreateUser::new(
                    -1,
                    protagonist.last_name,
                    protagonist.first_name,
                    protagonist.login_id,
                    protagonist.password,
                    protagonist.email,
                    protagonist.country,
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
        user: request::UpdateUserRequest,
    ) -> Result<response::UpdateUserResponse, anyhow::Error> {
        let user = self
            .repository
            .update_user(
                model::UpdateUser::new(
                    user.user_id,
                    user.last_name,
                    user.first_name,
                    user.login_id,
                    user.password,
                    user.email,
                    user.country,
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
            None => Err(anyhow::anyhow!("Protagonist not found")),
        }
    }

    pub async fn get_supporter(
        &self,
        id: i64,
    ) -> Result<response::GetSupporterResponse, anyhow::Error> {
        let supporter = self.repository.get_supporter(id).await?;
        match supporter {
            Some(supporter) => Ok(response::GetSupporterResponse {
                supporter_id: u64::try_from(supporter.supporter_id).unwrap(),
                supporter_last_name: supporter.last_name,
                supporter_first_name: supporter.first_name,
                supporter_email: supporter.email,
                supporter_country: supporter.country,
            }),
            None => Err(anyhow::anyhow!("Supporter not found")),
        }
    }

    pub async fn create_supporter(
        &self,
        supporter: request::CreateSupporterRequest,
    ) -> Result<response::CreateSupporterResponse, anyhow::Error> {
        // support_id is set to -1 because it is auto-incremented in the database
        let supporter = self
            .repository
            .create_supporter(
                model::CreateSupporter::new(
                    -1,
                    supporter.last_name,
                    supporter.first_name,
                    supporter.login_id,
                    supporter.password,
                    supporter.email,
                    supporter.country,
                )
                .convert_hash_password()
                .await?,
            )
            .await?;

        match supporter {
            Some(supporter) => Ok(response::CreateSupporterResponse {
                supporter_id: u64::try_from(supporter.supporter_id).unwrap(),
                supporter_last_name: supporter.last_name,
                supporter_first_name: supporter.first_name,
                supporter_email: supporter.email,
                supporter_country: supporter.country,
            }),
            None => Err(anyhow::anyhow!("Supporter not created")),
        }
    }

    pub async fn update_supporter(
        &self,
        supporter: request::UpdateSupporterRequest,
    ) -> Result<response::UpdateSupporterResponse, anyhow::Error> {
        let supporter = self
            .repository
            .update_supporter(
                model::UpdateSupporter::new(
                    supporter.supporter_id,
                    supporter.last_name,
                    supporter.first_name,
                    supporter.login_id,
                    supporter.password,
                    supporter.email,
                    supporter.country,
                )
                .convert_hash_password()
                .await?,
            )
            .await?;

        match supporter {
            Some(supporter) => Ok(response::UpdateSupporterResponse {
                supporter_id: u64::try_from(supporter.supporter_id).unwrap(),
                supporter_last_name: supporter.last_name,
                supporter_first_name: supporter.first_name,
                supporter_email: supporter.email,
                supporter_country: supporter.country,
            }),
            None => Err(anyhow::anyhow!("Supporter not updated")),
        }
    }

    pub async fn delete_supporter(&self, id: i64) -> Result<(), anyhow::Error> {
        let result = self.repository.delete_supporter(id).await?;
        match result {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("Supporter not deleted")),
        }
    }

    pub async fn get_supporter_by_login_id_and_password(
        &self,
        login_request: request::GetSupporterRequest,
    ) -> Result<response::GetSupporterResponse, anyhow::Error> {
        let supporter = self
            .repository
            .get_supporter_by_login_id_and_password(login_request.login_id.as_str())
            .await?;

        match supporter {
            Some(supporter) => {
                let valid = supporter
                    .verify_password(login_request.password.as_str())
                    .await
                    .map_err(|e| anyhow::anyhow!("Error verifying password: {}", e.to_string()))?; // This is a bug, the error should be returned instead of being ignored
                if !valid {
                    return Err(anyhow::anyhow!("Invalid password"));
                }

                Ok(response::GetSupporterResponse {
                    supporter_id: u64::try_from(supporter.supporter_id).unwrap(),
                    supporter_last_name: supporter.last_name,
                    supporter_first_name: supporter.first_name,
                    supporter_email: supporter.email,
                    supporter_country: supporter.country,
                })
            }
            None => Err(anyhow::anyhow!("Supporter not found")),
        }
    }

    pub async fn get_protagonist_supporter(
        &self,
        id: i64,
    ) -> Result<Vec<response::GetProtagonistSupporterResponse>, anyhow::Error> {
        let protagonist_supporters = self.repository.get_protagonist_supporter(id).await?;
        match protagonist_supporters {
            Some(protagonist_supporters) => Ok(protagonist_supporters
                .into_iter()
                .map(
                    |protagonist_supporter| response::GetProtagonistSupporterResponse {
                        supporter_id: u64::try_from(protagonist_supporter.supporter_id).unwrap(),
                        last_name: protagonist_supporter.last_name,
                        first_name: protagonist_supporter.first_name,
                        country: protagonist_supporter.country,
                    },
                )
                .collect()),
            None => Err(anyhow::anyhow!("Protagonist supporter not found")),
        }
    }

    pub async fn create_protagonist_supporter(
        &self,
        protagonist_supporter_request: request::CreateProtagonistSupporterRequest,
    ) -> Result<response::CreateProtagonistSupporterResponse, anyhow::Error> {
        let protagonist_supporter = self
            .repository
            .create_protagonist_supporter(model::CreateProtagonistSupporter::new(
                -1,
                i64::try_from(protagonist_supporter_request.protagonist_id).unwrap(),
                i64::try_from(protagonist_supporter_request.supporter_id).unwrap(),
            ))
            .await?;

        match protagonist_supporter {
            Some(protagonist_supporter) => Ok(response::CreateProtagonistSupporterResponse {
                protagonist_supporter_id: u64::try_from(
                    protagonist_supporter.protagonist_supporter_id,
                )
                .unwrap(),
            }),
            None => Err(anyhow::anyhow!("Protagonist supporter not created")),
        }
    }

    pub async fn delete_protagonist_supporter(&self, id: i64) -> Result<(), anyhow::Error> {
        let result = self.repository.delete_protagonist_supporter(id).await?;
        match result {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("Protagonist supporter not deleted")),
        }
    }
}
