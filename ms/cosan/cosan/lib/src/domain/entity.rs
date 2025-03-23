use crate::util;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct UserId(i64);
impl UserId {
    pub fn new(user_id: i64) -> Self {
        Self(user_id)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct LastName(String);
impl LastName {
    pub fn new(last_name: &str) -> Self {
        Self(last_name.to_string())
    }

    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct FirstName(String);
impl FirstName {
    pub fn new(first_name: &str) -> Self {
        Self(first_name.to_string())
    }

    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct LoginId(String);
impl LoginId {
    pub fn new(login_id: &str) -> Self {
        Self(login_id.to_string())
    }

    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct PasswordHash(String);
impl PasswordHash {
    pub fn new(password_hash: &str) -> Self {
        Self(password_hash.to_string())
    }

    pub fn value(&self) -> &str {
        self.0.as_str()
    }

    pub async fn verify(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        util::crypt::verify_password(password, &self.0).await
    }
}

#[derive(Debug, Clone)]
pub struct Password(String);
impl Password {
    pub fn new(password: &str) -> Self {
        Self(password.to_string())
    }

    pub async fn hash(&self) -> Result<PasswordHash, bcrypt::BcryptError> {
        let hashed_password = util::crypt::hash_password(&self.0).await?;
        Ok(PasswordHash::new(hashed_password.as_str()))
    }

    pub async fn verify(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        util::crypt::verify_password(password, &self.0).await
    }

    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct Email(String);
impl Email {
    pub fn new(email: &str) -> Self {
        Self(email.to_string())
    }

    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct Country(String);
impl Country {
    pub fn new(country: &str) -> Self {
        Self(country.to_string())
    }

    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct WordId(i64);
impl WordId {
    pub fn new(word_id: i64) -> Self {
        Self(word_id)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct WordString(String);
impl WordString {
    pub fn new(word: &str) -> Self {
        Self(word.to_string())
    }

    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct UserWordId(i64);
impl UserWordId {
    pub fn new(user_word_id: i64) -> Self {
        Self(user_word_id)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct CreatedAt(DateTime<Utc>);
impl CreatedAt {
    pub fn new(created_at: DateTime<Utc>) -> Self {
        Self(created_at)
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: UserId,
    pub last_name: LastName,
    pub first_name: FirstName,
    pub login_id: LoginId,
    pub password: PasswordHash,
    pub email: Email,
    pub country: Country,
}

impl User {
    pub fn new(
        user_id: UserId,
        last_name: LastName,
        first_name: FirstName,
        login_id: LoginId,
        password: PasswordHash,
        email: Email,
        country: Country,
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
}

#[derive(Debug, Clone)]
pub struct Word {
    pub word_id: WordId,
    pub word: WordString,
}
impl Word {
    pub fn new(word_id: WordId, word: WordString) -> Self {
        Self { word_id, word }
    }
}

#[derive(Debug, Clone)]
pub struct UserWord {
    pub user_word_id: UserWordId,
    pub user_id: UserId,
    pub last_name: LastName,
    pub first_name: FirstName,
    pub email: Email,
    pub country: Country,
    pub word_id: WordId,
    pub word: WordString,
    pub created_at: CreatedAt,
}
impl UserWord {
    pub fn new(
        user_word_id: UserWordId,
        user_id: UserId,
        last_name: LastName,
        first_name: FirstName,
        email: Email,
        country: Country,
        word_id: WordId,
        word: WordString,
        created_at: CreatedAt,
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

#[derive(Debug, Clone)]
pub struct UserWordRelation {
    pub user_id: UserId,
    pub word_id: WordId,
    pub created_at: CreatedAt,
}
impl UserWordRelation {
    pub fn new(user_id: UserId, word_id: WordId, created_at: CreatedAt) -> Self {
        Self {
            user_id,
            word_id,
            created_at,
        }
    }
}
