use chrono::{naive::NaiveDate};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize)]
pub struct HLOC {
    pub high: f32,
    pub low: f32,
    pub open: f32,
    pub close: f32,
    #[serde(deserialize_with = "from_date")]
    pub date: NaiveDate
}

fn from_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer).unwrap();
    Ok(NaiveDate::from_str(s).unwrap())
}
