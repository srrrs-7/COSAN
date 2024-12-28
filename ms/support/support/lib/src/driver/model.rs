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
    pub status: String,
}

#[derive(Debug, FromRow)]
pub struct UpdateProtagonist {
    pub status: String,
}

#[derive(Debug, FromRow)]
pub struct DeleteProtagonist {
    pub status: String,
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
    pub status: String,
}

#[derive(Debug, FromRow)]
pub struct UpdateSupporter {
    pub status: String,
}

#[derive(Debug, FromRow)]
pub struct DeleteSupporter {
    pub status: String,
}

#[derive(Debug, FromRow)]
pub struct GetProtagonistSupporter {
    pub protagonist_id: i64,
    pub protagonist_name: String,
    pub protagonist_country: String,
    pub supporter_id: i64,
    pub supporter_name: String,
    pub supporter_country: String,
}

#[derive(Debug, FromRow)]
pub struct CreateProtagonistSupporter {
    pub status: String,
}

#[derive(Debug, FromRow)]
pub struct UpdateProtagonistSupporter {
    pub status: String,
}

#[derive(Debug, FromRow)]
pub struct DeleteProtagonistSupporter {
    pub status: String,
}
