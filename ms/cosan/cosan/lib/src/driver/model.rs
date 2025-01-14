use crate::util;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct GetUser {
    pub user_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub login_id: String,
    pub password: String,
    pub email: String,
    pub country: String,
}

impl GetUser {
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

    pub fn is_valid(&self) -> bool {
        self.user_id >= 0
            && !self.last_name.is_empty()
            && !self.first_name.is_empty()
            && !self.login_id.is_empty()
            && !self.password.is_empty()
            && !self.email.is_empty()
            && !self.country.is_empty()
    }

    pub async fn convert_hash_password(self) -> Result<Self, bcrypt::BcryptError> {
        let hashed_password = util::crypt::hash_password(&self.password).await?;
        Ok(Self {
            password: hashed_password,
            ..self
        })
    }
}

#[derive(Debug, FromRow)]
pub struct CreateUser {
    pub user_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub login_id: String,
    pub password: String,
    pub email: String,
    pub country: String,
}

impl CreateUser {
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

    pub fn is_valid(&self) -> bool {
        self.user_id >= 0
            && !self.last_name.is_empty()
            && !self.first_name.is_empty()
            && !self.login_id.is_empty()
            && !self.password.is_empty()
            && !self.email.is_empty()
            && !self.country.is_empty()
    }

    pub async fn convert_hash_password(self) -> Result<Self, bcrypt::BcryptError> {
        let hashed_password = util::crypt::hash_password(&self.password).await?;
        Ok(Self {
            password: hashed_password,
            ..self
        })
    }
}

#[derive(Debug, FromRow)]
pub struct UpdateUser {
    pub user_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub login_id: String,
    pub password: String,
    pub email: String,
    pub country: String,
}

impl UpdateUser {
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

    pub fn is_valid(&self) -> bool {
        self.user_id >= 0
            && !self.last_name.is_empty()
            && !self.first_name.is_empty()
            && !self.login_id.is_empty()
            && !self.password.is_empty()
            && !self.email.is_empty()
            && !self.country.is_empty()
    }

    pub async fn convert_hash_password(self) -> Result<Self, bcrypt::BcryptError> {
        let hashed_password = util::crypt::hash_password(&self.password).await?;
        Ok(Self {
            password: hashed_password,
            ..self
        })
    }
}

#[derive(Debug, FromRow)]
pub struct GetWord {
    pub word_id: i64,
    pub word: String,
}

impl GetWord {
    pub fn new(word_id: i64, word: String) -> Self {
        Self { word_id, word }
    }

    pub fn is_valid(&self) -> bool {
        self.word_id >= 0 && !self.word.is_empty()
    }
}

#[derive(Debug, FromRow)]
pub struct CreateWord {
    pub word_id: i64,
    pub word: String,
}

impl CreateWord {
    pub fn new(word_id: i64, word: String) -> Self {
        Self { word_id, word }
    }

    pub fn is_valid(&self) -> bool {
        self.word_id >= 0 && self.word.is_empty()
    }
}

#[derive(Debug, FromRow)]
pub struct UpdateWord {
    pub word_id: i64,
    pub word: String,
}

impl UpdateWord {
    pub fn new(word_id: i64, word: String) -> Self {
        Self { word_id, word }
    }

    pub fn is_valid(&self) -> bool {
        self.word_id >= 0 && !self.word.is_empty()
    }
}

#[derive(Debug, FromRow)]
pub struct GetUserWord {
    pub user_word_id: i64,
    pub user_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
    pub word_id: i64,
    pub word: String,
    pub created_at: String,
}

impl GetUserWord {
    pub fn new(
        user_word_id: i64,
        user_id: i64,
        last_name: String,
        first_name: String,
        email: String,
        country: String,
        word_id: i64,
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

    pub fn is_valid(&self) -> bool {
        self.user_word_id >= 0
            && self.user_id >= 0
            && !self.last_name.is_empty()
            && !self.first_name.is_empty()
            && !self.email.is_empty()
            && !self.country.is_empty()
            && self.word_id >= 0
            && !self.word.is_empty()
            && !self.created_at.is_empty()
    }
}

#[derive(Debug, FromRow)]
pub struct GetUserWordId {
    pub user_id: i64,
    pub word_id: i64,
}

impl GetUserWordId {
    pub fn new(user_id: i64, word_id: i64) -> Self {
        Self { user_id, word_id }
    }

    pub fn is_valid(&self) -> bool {
        self.user_id >= 0 && self.word_id >= 0
    }
}

#[derive(Debug, FromRow)]
pub struct GetUserWordRelation {
    pub user_id: i64,
    pub word_id: i64,
    pub created_at: String,
}

impl GetUserWordRelation {
    pub fn new(user_id: i64, word_id: i64, created_at: String) -> Self {
        Self {
            user_id,
            word_id,
            created_at,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.user_id >= 0 && self.word_id >= 0
    }
}

#[derive(Debug, FromRow)]
pub struct CreateUserWord {
    pub user_id: i64,
    pub word_id: i64,
}

impl CreateUserWord {
    pub fn new(user_id: i64, word_id: i64) -> Self {
        Self { user_id, word_id }
    }

    pub fn is_valid(&self) -> bool {
        self.user_id >= 0 && self.word_id >= 0
    }
}
