use crate::util;

pub struct Protagonist {
    pub protagonist_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub login_id: String,
    pub password: String,
    pub email: String,
    pub country: String,
}

impl Protagonist {
    pub fn new(
        protagonist_id: i64,
        last_name: String,
        first_name: String,
        login_id: String,
        password: String,
        email: String,
        country: String,
    ) -> Self {
        Self {
            protagonist_id,
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

pub struct Supporter {
    pub supporter_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub login_id: String,
    pub password: String,
    pub email: String,
    pub country: String,
}

impl Supporter {
    pub fn new(
        supporter_id: i64,
        last_name: String,
        first_name: String,
        login_id: String,
        password: String,
        email: String,
        country: String,
    ) -> Self {
        Self {
            supporter_id,
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

pub struct ProtagonistSupporter {
    pub supporter_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub country: String,
}

impl ProtagonistSupporter {
    pub fn new(supporter_id: i64, last_name: String, first_name: String, country: String) -> Self {
        Self {
            supporter_id,
            last_name,
            first_name,
            country,
        }
    }
}

pub struct ProtagonistSupporterRelation {
    pub protagonist_supporter_id: i64,
}

impl ProtagonistSupporterRelation {
    pub fn new(protagonist_supporter_id: i64) -> Self {
        Self {
            protagonist_supporter_id,
        }
    }
}
