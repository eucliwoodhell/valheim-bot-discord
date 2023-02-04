use chrono::DateTime;

pub struct LogServe {
    pub at_date: String,
    pub content: String,
    pub server_address: String,
}

pub struct Content(String);

impl TryFrom<String> for Content {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

pub struct ServerAddress(String);

impl TryFrom<String> for ServerAddress {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 0 {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

pub struct AtDate(String);

impl TryFrom<String> for AtDate {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if DateTime::parse_from_rfc3339(&value).is_ok() {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}
