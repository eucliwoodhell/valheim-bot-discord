pub struct Valheim {
    server_address: String,
    port: i64,
    domain: String,
    password: String,
    type_server: i64,
    valheim_plus: i64,
}

impl Clone for Valheim {
    fn clone(&self) -> Self {
        Valheim {
            server_address: self.server_address.to_string(),
            port: self.port,
            domain: self.domain.to_string(),
            password: self.password.to_string(),
            type_server: self.type_server,
            valheim_plus: self.valheim_plus,
        }
    }
}

pub struct ValheimList(Vec<Valheim>);

impl TryFrom<Vec<Valheim>> for ValheimList {
    type Error = ();

    fn try_from(value: Vec<Valheim>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            let mut lists: Vec<Valheim> = vec![];
            for n in value {
                match Valheim::try_from(Valheim::from(n)) {
                    Ok(list) => lists.push(list),
                    _ => return Err(()),
                }
            }
            Ok(Self(lists))
        }
    }
}

pub struct ServerAddress(String);

impl TryFrom<String> for ServerAddress {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

pub struct Port(i64);

impl TryFrom<i64> for Port {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value.is_positive() {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

pub struct ValheimPlus(i64);

impl TryFrom<i64> for ValheimPlus {
    type Error = ();
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value >= 0 as i64 && value <= 1 as i64 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

pub enum TypeServer {
    Dedicate,
    Virtual,
}

impl TryFrom<String> for TypeServer {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Dedicate" => Ok(Self::Dedicate),
            "Virtual" => Ok(Self::Virtual),
            _ => Err(()),
        }
    }
}
