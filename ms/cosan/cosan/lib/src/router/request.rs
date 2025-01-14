use anyhow::{anyhow, Ok, Result};
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetUserRequest {
    pub login_id: String,
    pub password: String,
}

impl GetUserRequest {
    pub fn new(login_id: String, password: String) -> Self {
        Self { login_id, password }
    }

    pub async fn validate(self) -> Result<Self, anyhow::Error> {
        let login_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !login_regex.is_match(&self.login_id) {
            return Err(anyhow!("Invalid login id format."));
        }
        if !login_regex.is_match(&self.password) {
            return Err(anyhow!("Invalid password format."));
        }

        Ok(self)
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    pub last_name: String,
    pub first_name: String,
    pub login_id: String,
    pub password: String,
    pub email: String,
    pub country: String,
}

impl CreateUserRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let name_regex = Regex::new(r"^[\p{L}\p{N}\s'-]+$").unwrap();
        if !name_regex.is_match(&self.last_name) {
            return Err(anyhow!("Invalid last name: Contains invalid characters. Only alphabets, numbers, spaces, hyphens, and apostrophes are allowed."));
        }
        if !name_regex.is_match(&self.first_name) {
            return Err(anyhow!("Invalid first name: Contains invalid characters. Only alphabets, numbers, spaces, hyphens, and apostrophes are allowed."));
        }

        let login_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !login_regex.is_match(&self.login_id) {
            return Err(anyhow!("Invalid login id format."));
        }
        if !login_regex.is_match(&self.password) {
            return Err(anyhow!("Invalid password format."));
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
pub struct UpdateUserRequest {
    pub user_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub login_id: String,
    pub password: String,
    pub email: String,
    pub country: String,
}

impl UpdateUserRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let id_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !id_regex.is_match(&self.user_id.to_string()) {
            return Err(anyhow!("Invalid id format."));
        }

        let name_regex = Regex::new(r"^[\p{L}\p{N}\s'-]+$").unwrap();
        if !name_regex.is_match(&self.last_name) {
            return Err(anyhow!("Invalid last name: Contains invalid characters. Only alphabets, numbers, spaces, hyphens, and apostrophes are allowed."));
        }
        if !name_regex.is_match(&self.first_name) {
            return Err(anyhow!("Invalid first name: Contains invalid characters. Only alphabets, numbers, spaces, hyphens, and apostrophes are allowed."));
        }

        let login_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !login_regex.is_match(&self.login_id) {
            return Err(anyhow!("Invalid login id format."));
        }
        if !login_regex.is_match(&self.password) {
            return Err(anyhow!("Invalid password format."));
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
pub struct CreateWordRequest {
    pub word: String,
}

impl CreateWordRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let word_regex = Regex::new(r"^[\p{L}\p{N}\s'-]+$").unwrap();
        if !word_regex.is_match(&self.word) {
            return Err(anyhow!("Invalid word format."));
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdateWordRequest {
    pub word_id: u64,
    pub word: String,
}

impl UpdateWordRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let word_regex = Regex::new(r"^[\p{L}\p{N}\s'-]+$").unwrap();
        if !word_regex.is_match(&self.word) {
            return Err(anyhow!("Invalid word format."));
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct GetUserWordRequest {
    pub user_id: u64,
    pub word_id: u64,
}

impl GetUserWordRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let id_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !id_regex.is_match(&self.user_id.to_string()) {
            return Err(anyhow!("Invalid id format."));
        }
        if !id_regex.is_match(&self.word_id.to_string()) {
            return Err(anyhow!("Invalid id format."));
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateUserWordRequest {
    pub user_id: u64,
    pub word_id: u64,
}

impl CreateUserWordRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let id_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !id_regex.is_match(&self.user_id.to_string()) {
            return Err(anyhow!("Invalid id format."));
        }
        if !id_regex.is_match(&self.word_id.to_string()) {
            return Err(anyhow!("Invalid id format."));
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct DeleteUserWordRequest {
    pub user_word_id: u64,
}

impl DeleteUserWordRequest {
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        let id_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !id_regex.is_match(&self.user_word_id.to_string()) {
            return Err(anyhow!("Invalid id format."));
        }

        Ok(())
    }
}
