use crate::domain::log::LogServe;

pub trait LogRepository {
    fn get(&self, at_date: String) -> Option<LogServe>;
    fn list(&self) -> Vec<LogServe>;
    fn save(&self, log: &LogServe) -> bool;
    fn delete(&self, at_date: String) -> bool;
}
