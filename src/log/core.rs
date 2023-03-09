use serde::{Deserialize, Serialize};

pub fn import(target: &str) -> Option<Vec<String>> {
    match std::fs::read(target) {
        Err(e) => {
            println!("{e}\ndirectory: {target}");
            None
        }
        Ok(a) => Some(vecify(a)),
    }
}

fn vecify(input: Vec<u8>) -> Vec<String> {
    let input = String::from_utf8(input).unwrap();

    let mut master: Vec<String> = Vec::new();
    for x in input.split('\n') {
        master.push(x.to_string())
    }

    master
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    pub month: String,
    pub day: u32,
    pub time: String,
}
impl Date {
    pub fn new(month: &str, day: &str, time: &str) -> Self {
        Self {
            month: String::from(month),
            day: day.parse::<u32>().unwrap_or_default(),
            time: String::from(time),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub date: Date,
    pub hostname: String,
    pub ip: String,
}
