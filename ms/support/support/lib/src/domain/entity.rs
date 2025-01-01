pub struct Protagonist {
    pub protagonist_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
}

impl Protagonist {
    pub fn new(
        protagonist_id: i64,
        last_name: String,
        first_name: String,
        email: String,
        country: String,
    ) -> Self {
        Self {
            protagonist_id,
            last_name,
            first_name,
            email,
            country,
        }
    }
}

pub struct Supporter {
    pub supporter_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub country: String,
}

impl Supporter {
    pub fn new(
        supporter_id: i64,
        last_name: String,
        first_name: String,
        email: String,
        country: String,
    ) -> Self {
        Self {
            supporter_id,
            last_name,
            first_name,
            email,
            country,
        }
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
