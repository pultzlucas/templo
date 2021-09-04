use chrono::prelude::Utc;
use regex::Regex;

pub fn get_date_now_string() -> String {
    Regex::new(r"\..+")
        .unwrap()
        .replace(&Utc::now().to_string(), "")
        .trim()
        .to_string()
}