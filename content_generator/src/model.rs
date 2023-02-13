use chrono::naive::NaiveDate;
use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Page {
    js_libraries: bool,
    content: Content,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Content {
    sections: Vec<Section>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Section {
    title: String,
    #[serde(default = "today")]
    date: NaiveDate,
    paragraphs: Vec<String>,
}

fn today() -> NaiveDate {
    let now = Utc::now();
    NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).expect("must be a valid date")
}
