use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetProtagonistRequest {
    pub id: u64,
}

#[derive(Deserialize, Debug)]
pub struct CreateProtagonistRequest {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateProtagonistRequest {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct GetSupporterRequest {
    pub id: u64,
}

#[derive(Deserialize, Debug)]
pub struct CreateSupporterRequest {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateSupporterRequest {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct GetProtagonistSupporterResponse {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateProtagonistSupporterRequest {
    pub protagonist_id: u64,
    pub supporter_id: u64,
}

#[derive(Deserialize, Debug)]
pub struct UpdateProtagonistSupporterRequest {
    pub id: u64,
    pub protagonist_id: u64,
    pub supporter_id: u64,
}
