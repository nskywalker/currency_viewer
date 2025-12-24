extern crate reqwest;
use std::collections::{BTreeMap, HashMap};
use time;

use reqwest::Error;

#[derive(serde::Deserialize, Debug)]
pub struct CurrencyTable {
    pub amount: f32,
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f32>
}

pub struct CurrencyGetter {
    resource: String
}

impl CurrencyGetter {
    pub fn new() -> Self {
        CurrencyGetter {resource: "https://api.frankfurter.dev/v1/".to_string()}
    }

    async fn make_response(&self, site: &str) -> Result<String, Error>{
        match reqwest::get(format!("{}{}", self.resource, site)).await {
            Ok(res) => res.text().await,
            Err(e) => Err(e)
        }
    }

    pub async fn currencies(&self) -> BTreeMap<String, String> {
        match self.make_response("currencies").await {
            Ok(text) => match serde_json::from_str(text.as_str())  {
                Ok(v) => v,
                _ => BTreeMap::new()
            },
            _ => BTreeMap::new()
        }
    }

    pub async fn latest(&self, base: &str) -> Option<CurrencyTable> {
        match self.make_response(format!("latest?base={base}").as_str()).await {
            Ok(text) => match serde_json::from_str(text.as_str()) {
                Ok(v) => Some(v),
                _ => None
            },
            _ => None
        }
    }

    pub async fn currencies_at_date(&self, base: &str, date: time::Date) -> Option<CurrencyTable> {
        match self.make_response(format!("{}?base={base}", date.to_string()).as_str()).await {
            Ok(text) => match serde_json::from_str(text.as_str()) {
                Ok(v) => Some(v),
                _ => None
            },
            _ => None
        }
    }
}