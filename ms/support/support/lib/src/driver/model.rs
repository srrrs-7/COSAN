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
    pub email: String,
    pub country: String,
}

#[derive(Debug, FromRow)]
pub struct UpdateProtagonist {
    pub protagonist_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
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
    pub email: String,
    pub country: String,
}

#[derive(Debug, FromRow)]
pub struct UpdateSupporter {
    pub supporter_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
}

#[derive(Debug, FromRow)]
pub struct GetProtagonistSupporter {
    pub supporter_id: i64,
    pub supporter_last_name: String,
    pub supporter_first_name: String,
    pub supporter_country: String,
}

#[derive(Debug, FromRow)]
pub struct CreateProtagonistSupporter {
    pub protagonist_id: i64,
    pub supporter_id: i64,
}
