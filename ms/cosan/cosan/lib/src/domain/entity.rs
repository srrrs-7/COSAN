use crate::util;

pub struct User {
    pub user_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub login_id: String,
    pub password: String,
    pub email: String,
    pub country: String,
}

impl User {
    pub fn new(
        user_id: i64,
        last_name: String,
        first_name: String,
        login_id: String,
        password: String,
        email: String,
        country: String,
    ) -> Self {
        Self {
            user_id,
            last_name,
            first_name,
            login_id,
            password,
            email,
            country,
        }
    }

    pub async fn verify_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        util::crypt::verify_password(password, &self.password).await
    }
}

pub struct Word {
    pub word_id: u64,
    pub word: String,
}

impl Word {
    pub fn new(word_id: u64, word: String) -> Self {
        Self { word_id, word }
    }
}

pub struct UserWord {
    pub user_word_id: u64,
    pub user_id: u64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
    pub word_id: u64,
    pub word: String,
    pub created_at: String,
}

impl UserWord {
    pub fn new(
        user_word_id: u64,
        user_id: u64,
        last_name: String,
        first_name: String,
        email: String,
        country: String,
        word_id: u64,
        word: String,
        created_at: String,
    ) -> Self {
        Self {
            user_word_id,
            user_id,
            last_name,
            first_name,
            email,
            country,
            word_id,
            word,
            created_at,
        }
    }
}

pub struct UserWordRelation {
    pub user_id: u64,
    pub word_id: u64,
    pub created_at: String,
}

impl UserWordRelation {
    pub fn new(user_id: u64, word_id: u64, created_at: String) -> Self {
        Self {
            user_id,
            word_id,
            created_at,
        }
    }
}
