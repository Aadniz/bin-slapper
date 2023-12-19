use chrono::prelude::*;
use serde::Serialize;
use std::time::SystemTime;

use super::{utils};

#[derive(Serialize, Debug)]
pub struct DateTimeComponents {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}
impl DateTimeComponents {
    pub fn new() -> Self {
        let now : DateTime<Utc> = chrono::Utc::now();
        Self {
            year: now.year(),
            month: now.month(),
            day: now.day(),
            hour: now.hour(),
            minute: now.minute(),
            second: now.second(),
        }
    }
}


#[derive(Serialize, Debug)]
pub struct Sorts {
    pub name: String,
    pub style_version: String,
    pub main_js_version: String,
    pub date_time: DateTimeComponents,
    pub start_load_time: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<(u16, String)>,
}

impl Sorts {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            style_version: utils::get_file_modification_time("/static/css/style.css"),
            main_js_version: utils::get_file_modification_time("/static/js/main.js"),
            date_time: DateTimeComponents::new(),
            start_load_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f64(),
            status_info: None
        }
    }

    pub fn status(mut self, status_code: u16, msg: String) -> Self {
        self.status_info = Some((status_code, msg));
        self
    }
}