use chrono::{DateTime, Utc};

pub struct Player {
    steam_id: String,
    name: String,
    last_date_connect: DateTime<Utc>,
}
