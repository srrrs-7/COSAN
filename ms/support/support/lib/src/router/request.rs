use anyhow::{anyhow, Ok, Result};
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateProtagonistRequest {
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
}

impl CreateProtagonistRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let name_regex = Regex::new(r"^[\p{L}\p{N}\s'-]+$").unwrap();
        if !name_regex.is_match(&self.last_name) {
            return Err(anyhow!("Invalid last name: Contains invalid characters. Only alphabets, numbers, spaces, hyphens, and apostrophes are allowed."));
        }
        if !name_regex.is_match(&self.first_name) {
            return Err(anyhow!("Invalid first name: Contains invalid characters. Only alphabets, numbers, spaces, hyphens, and apostrophes are allowed."));
        }

        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(&self.email) {
            return Err(anyhow!("Invalid email format."));
        }

        if self.country.is_empty() {
            return Err(anyhow!("Country cannot be empty."));
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdateProtagonistRequest {
    pub protagonist_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
}

impl UpdateProtagonistRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let id_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !id_regex.is_match(&self.protagonist_id.to_string()) {
            return Err(anyhow!("Invalid id format."));
        }

        let name_regex = Regex::new(r"^[\p{L}\p{N}\s'-]+$").unwrap();
        if !name_regex.is_match(&self.last_name) {
            return Err(anyhow!("Invalid last name: Contains invalid characters. Only alphabets, numbers, spaces, hyphens, and apostrophes are allowed."));
        }
        if !name_regex.is_match(&self.first_name) {
            return Err(anyhow!("Invalid first name: Contains invalid characters. Only alphabets, numbers, spaces, hyphens, and apostrophes are allowed."));
        }

        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(&self.email) {
            return Err(anyhow!("Invalid email format."));
        }

        if self.country.is_empty() {
            return Err(anyhow!("Country cannot be empty."));
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateSupporterRequest {
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
}

impl CreateSupporterRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let name_regex = Regex::new(r"^[\p{L}\p{N}\s'-]+$").unwrap();
        if !name_regex.is_match(&self.last_name) {
            return Err(anyhow!("Invalid last name: Contains invalid characters. Only alphabets, numbers, spaces, hyphens, and apostrophes are allowed."));
        }
        if !name_regex.is_match(&self.first_name) {
            return Err(anyhow!("Invalid first name: Contains invalid characters. Only alphabets, numbers, spaces, hyphens, and apostrophes are allowed."));
        }

        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(&self.email) {
            return Err(anyhow!("Invalid email format."));
        }

        if self.country.is_empty() {
            return Err(anyhow!("Country cannot be empty."));
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdateSupporterRequest {
    pub supporter_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
}

impl UpdateSupporterRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let id_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !id_regex.is_match(&self.supporter_id.to_string()) {
            return Err(anyhow!("Invalid id format."));
        }

        let name_regex = Regex::new(r"^[\p{L}\p{N}\s'-]+$").unwrap();
        if !name_regex.is_match(&self.last_name) {
            return Err(anyhow!("Invalid last name: Contains invalid characters. Only alphabets, numbers, spaces, hyphens, and apostrophes are allowed."));
        }
        if !name_regex.is_match(&self.first_name) {
            return Err(anyhow!("Invalid first name: Contains invalid characters. Only alphabets, numbers, spaces, hyphens, and apostrophes are allowed."));
        }

        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(&self.email) {
            return Err(anyhow!("Invalid email format."));
        }

        if self.country.is_empty() {
            return Err(anyhow!("Country cannot be empty."));
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateProtagonistSupporterRequest {
    pub protagonist_id: u64,
    pub supporter_id: u64,
}

impl CreateProtagonistSupporterRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let id_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !id_regex.is_match(&self.protagonist_id.to_string()) {
            return Err(anyhow!("Invalid id format."));
        }
        if !id_regex.is_match(&self.supporter_id.to_string()) {
            return Err(anyhow!("Invalid id format."));
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct DeleteProtagonistSupporterRequest {
    pub protagonist_supporter_id: u64,
}

impl DeleteProtagonistSupporterRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let id_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !id_regex.is_match(&self.protagonist_supporter_id.to_string()) {
            return Err(anyhow!("Invalid id format."));
        }

        Ok(())
    }
}
