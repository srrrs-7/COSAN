pub struct Protagonist {
    pub protagonist_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
}

pub struct Supporter {
    pub supporter_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
}

pub struct ProtagonistSupporter {
    pub supporter_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub country: String,
}

pub struct ProtagonistSupporterRelation {
    pub protagonist_supporter_id: i64,
}