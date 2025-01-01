use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct GetProtagonist {
    pub protagonist_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
}

#[derive(Debug, FromRow)]
pub struct CreateProtagonist {
    pub protagonist_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub login_id: String,
    pub password: String,
    pub email: String,
    pub country: String,
}

impl CreateProtagonist {
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
}

#[derive(Debug, FromRow)]
pub struct UpdateProtagonist {
    pub protagonist_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub login_id: String,
    pub password: String,
    pub email: String,
    pub country: String,
}

impl UpdateProtagonist {
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
}

#[derive(Debug, FromRow)]
pub struct GetSupporter {
    pub supporter_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
}

#[derive(Debug, FromRow)]
pub struct CreateSupporter {
    pub supporter_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub login_id: String,
    pub password: String,
    pub email: String,
    pub country: String,
}

impl CreateSupporter {
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
}

#[derive(Debug, FromRow)]
pub struct UpdateSupporter {
    pub supporter_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub login_id: String,
    pub password: String,
    pub email: String,
    pub country: String,
}

impl UpdateSupporter {
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
}

#[derive(Debug, FromRow)]
pub struct GetProtagonistSupporter {
    pub supporter_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub country: String,
}

#[derive(Debug, FromRow)]
pub struct CreateProtagonistSupporter {
    pub protagonist_supporter_id: i64,
    pub protagonist_id: i64,
    pub supporter_id: i64,
}
