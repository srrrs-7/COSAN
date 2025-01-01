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
    ) -> Result<response::GetProtagonistResponse, anyhow::Error> {
        let protagonist = self.repository.get_protagonist(id).await?;

        match protagonist {
            Some(protagonist) => Ok(response::GetProtagonistResponse {
                protagonist_id: u64::try_from(protagonist.protagonist_id).unwrap(),
                protagonist_last_name: protagonist.last_name,
                protagonist_first_name: protagonist.first_name,
                protagonist_email: protagonist.email,
                protagonist_country: protagonist.country,
            }),
            None => Err(anyhow::anyhow!("Protagonist not found")),
        }
    }

    pub async fn create_protagonist(
        &self,
        protagonist: model::CreateProtagonist,
    ) -> Result<response::CreateProtagonistResponse, anyhow::Error> {
        let result = self.repository.create_protagonist(protagonist).await?;

        match result {
            Some(protagonist) => Ok(response::CreateProtagonistResponse {
                protagonist_id: u64::try_from(protagonist.protagonist_id).unwrap(),
                protagonist_last_name: protagonist.last_name,
                protagonist_first_name: protagonist.first_name,
                protagonist_email: protagonist.email,
                protagonist_country: protagonist.country,
            }),
            None => Err(anyhow::anyhow!("Protagonist not created")),
        }
    }

    pub async fn update_protagonist(
        &self,
        protagonist: model::UpdateProtagonist,
    ) -> Result<response::UpdateProtagonistResponse, anyhow::Error> {
        let protagonist = self.repository.update_protagonist(protagonist).await?;

        match protagonist {
            Some(protagonist) => Ok(response::UpdateProtagonistResponse {
                protagonist_id: u64::try_from(protagonist.protagonist_id).unwrap(),
                protagonist_last_name: protagonist.last_name,
                protagonist_first_name: protagonist.first_name,
                protagonist_email: protagonist.email,
                protagonist_country: protagonist.country,
            }),
            None => Err(anyhow::anyhow!("Protagonist not updated")),
        }
    }

    pub async fn delete_protagonist(&self, id: i64) -> Result<(), anyhow::Error> {
        let result = self.repository.delete_protagonist(id).await?;

        match result {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("Protagonist not deleted")),
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
        supporter: model::CreateSupporter,
    ) -> Result<response::CreateSupporterResponse, anyhow::Error> {
        let supporter = self.repository.create_supporter(supporter).await?;

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
        supporter: model::UpdateSupporter,
    ) -> Result<response::UpdateSupporterResponse, anyhow::Error> {
        let supporter = self.repository.update_supporter(supporter).await?;

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
        protagonist_supporter: model::CreateProtagonistSupporter,
    ) -> Result<response::CreateProtagonistSupporterResponse, anyhow::Error> {
        let protagonist_supporter = self
            .repository
            .create_protagonist_supporter(protagonist_supporter)
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
