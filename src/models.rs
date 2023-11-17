use chrono::{DateTime, Local};
use serde::Deserialize;
use sqlx::{Decode, FromRow};

#[derive(Debug, FromRow, Decode)]
#[allow(non_snake_case)]
pub struct ShellyDataRow {
    pub entryID: i16,
    pub timestamp: Option<DateTime<Local>>,
    pub powerLevel: Option<i32>
}

#[derive(Deserialize, Debug)]
pub struct ShellyMeter {
    pub power: f64,
    pub overpower: f64,
    pub is_valid: bool,
    pub timestamp: i32,
    pub counters: Vec<f64>,
    pub total: i32
}
