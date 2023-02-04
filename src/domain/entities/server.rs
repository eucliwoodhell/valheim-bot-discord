use chrono::{DateTime, Utc};

pub struct Server {
    id: i64,
    name: String,
    port: i64,
    world: String,
    state: bool,
    at_date: DateTime<Utc>,
}
