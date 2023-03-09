use serde::{Deserialize, Serialize};

use super::core::Date;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub date: Date,
    pub hostname: Option<String>,
    pub ip: String,
    pub r#type: Type,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Type {
    InvalidPassword,
    InvalidUser,
    InvalidKeyExchange,
}

impl Entry {
    pub fn inval_password(words: Vec<&str>) -> Self {
        let r#type = Type::InvalidPassword;

        Self {
            date: Date::new(words[0], words[1], words[2]),
            hostname: Some(String::from(words[8])),
            ip: String::from(words[10]),
            r#type,
        }
    }

    pub fn inval_user(words: Vec<&str>) -> Self {
        let r#type = Type::InvalidUser;

        Self {
            date: Date::new(words[0], words[1], words[2]),
            hostname: Some(String::from(words[10])),
            ip: String::from(words[13]),
            r#type,
        }
    }

    pub fn inval_keyexchange(words: Vec<&str>) -> Self {
        let r#type = Type::InvalidKeyExchange;

        Self {
            date: Date::new(words[0], words[1], words[2]),
            hostname: None,
            ip: String::from(words[9]),
            r#type,
        }
    }
}

#[allow(dead_code)]
pub fn deserilize(directory: &str) -> Option<Vec<Entry>> {
    let mut output: Vec<Entry> = Vec::new();

    let vec = match crate::log::core::import(directory) {
        Some(a) => a,
        None => return None,
    };

    for x in vec {
        let words: Vec<&str> = x.split(' ').collect();

        if words.len() > 7 {
            match (words[5], words[6], words[8]) {
                ("Failed", "password", "invalid") => output.push(Entry::inval_user(words)),
                ("Failed", "password", _) => output.push(Entry::inval_password(words)),
                ("Unable", "to", _) => output.push(Entry::inval_keyexchange(words)),
                _ => {}
            }
        }
    }

    Some(output)
}
