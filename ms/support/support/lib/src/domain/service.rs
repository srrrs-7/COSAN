use crate::{
    driver::{model, repository},
    router::response,
};

#[derive(Clone)]
pub struct SupportService {
    repository: repository::SupportRepository,
}

impl SupportService {
    pub fn new(repository: repository::SupportRepository) -> Self {
        Self { repository }
    }

    pub async fn get_protagonist(
        &self,
        id: i64,
    ) -> Result<response::GetProtagonistResponse, sqlx::Error> {
        let protagonist = self.repository.get_protagonist(id).await;

        match protagonist {
            Ok(protagonist) => Ok(response::GetProtagonistResponse {
                protagonist_id: u64::try_from(protagonist.protagonist_id).unwrap(),
                protagonist_last_name: protagonist.last_name,
                protagonist_first_name: protagonist.first_name,
                protagonist_email: protagonist.email,
                protagonist_country: protagonist.country,
            }),
            Err(err) => Err(err),
        }
    }

    pub async fn create_protagonist(
        &self,
        protagonist: model::CreateProtagonist,
    ) -> Result<response::CreateProtagonistResponse, sqlx::Error> {
        let result = self.repository.create_protagonist(protagonist).await;

        match result {
            Ok(protagonist) => Ok(response::CreateProtagonistResponse {
                protagonist_id: u64::try_from(protagonist.protagonist_id).unwrap(),
                protagonist_last_name: protagonist.last_name,
                protagonist_first_name: protagonist.first_name,
                protagonist_email: protagonist.email,
                protagonist_country: protagonist.country,
            }),
            Err(err) => Err(err),
        }
    }

    pub async fn update_protagonist(
        &self,
        protagonist: model::UpdateProtagonist,
    ) -> Result<response::UpdateProtagonistResponse, sqlx::Error> {
        let protagonist = self.repository.update_protagonist(protagonist).await;

        match protagonist {
            Ok(protagonist) => Ok(response::UpdateProtagonistResponse {
                protagonist_id: u64::try_from(protagonist.protagonist_id).unwrap(),
                protagonist_last_name: protagonist.last_name,
                protagonist_first_name: protagonist.first_name,
                protagonist_email: protagonist.email,
                protagonist_country: protagonist.country,
            }),
            Err(err) => Err(err),
        }
    }

    pub async fn delete_protagonist(&self, id: i64) -> Result<(), sqlx::Error> {
        self.repository.delete_protagonist(id).await
    }

    pub async fn get_supporter(
        &self,
        id: i64,
    ) -> Result<response::GetSupporterResponse, sqlx::Error> {
        let supporter = self.repository.get_supporter(id).await;

        match supporter {
            Ok(supporter) => Ok(response::GetSupporterResponse {
                supporter_id: u64::try_from(supporter.supporter_id).unwrap(),
                supporter_last_name: supporter.last_name,
                supporter_first_name: supporter.first_name,
                supporter_email: supporter.email,
                supporter_country: supporter.country,
            }),
            Err(err) => Err(err),
        }
    }

    pub async fn create_supporter(
        &self,
        supporter: model::CreateSupporter,
    ) -> Result<response::CreateSupporterResponse, sqlx::Error> {
        let supporter = self.repository.create_supporter(supporter).await;

        match supporter {
            Ok(supporter) => Ok(response::CreateSupporterResponse {
                supporter_id: u64::try_from(supporter.supporter_id).unwrap(),
                supporter_last_name: supporter.last_name,
                supporter_first_name: supporter.first_name,
                supporter_email: supporter.email,
                supporter_country: supporter.country,
            }),
            Err(err) => Err(err),
        }
    }

    pub async fn update_supporter(
        &self,
        supporter: model::UpdateSupporter,
    ) -> Result<response::UpdateSupporterResponse, sqlx::Error> {
        let supporter = self.repository.update_supporter(supporter).await;

        match supporter {
            Ok(supporter) => Ok(response::UpdateSupporterResponse {
                supporter_id: u64::try_from(supporter.supporter_id).unwrap(),
                supporter_last_name: supporter.last_name,
                supporter_first_name: supporter.first_name,
                supporter_email: supporter.email,
                supporter_country: supporter.country,
            }),
            Err(err) => Err(err),
        }
    }

    pub async fn delete_supporter(&self, id: i64) -> Result<(), sqlx::Error> {
        self.repository.delete_supporter(id).await
    }

    pub async fn get_protagonist_supporter(
        &self,
        id: i64,
    ) -> Result<Vec<response::GetProtagonistSupporterResponse>, sqlx::Error> {
        let protagonist_supporters = self.repository.get_protagonist_supporter(id).await;

        match protagonist_supporters {
            Ok(protagonist_supporters) => {
                let mut response = Vec::new();
                for protagonist_supporter in protagonist_supporters {
                    response.push(response::GetProtagonistSupporterResponse {
                        supporter_id: u64::try_from(protagonist_supporter.supporter_id).unwrap(),
                        last_name: protagonist_supporter.last_name,
                        first_name: protagonist_supporter.first_name,
                        country: protagonist_supporter.country,
                    })
                }
                Ok(response)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn create_protagonist_supporter(
        &self,
        protagonist_supporter: model::CreateProtagonistSupporter,
    ) -> Result<response::CreateProtagonistSupporterResponse, sqlx::Error> {
        let protagonist_supporter = self
            .repository
            .create_protagonist_supporter(protagonist_supporter)
            .await;

        match protagonist_supporter {
            Ok(protagonist_supporter) => Ok(response::CreateProtagonistSupporterResponse {
                protagonist_supporter_id: u64::try_from(
                    protagonist_supporter.protagonist_supporter_id,
                )
                .unwrap(),
            }),
            Err(err) => Err(err),
        }
    }

    pub async fn delete_protagonist_supporter(&self, id: i64) -> Result<(), sqlx::Error> {
        self.repository.delete_protagonist_supporter(id).await
    }
}
