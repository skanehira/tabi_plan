use axum::response::Json;
use serde::{Deserialize, Serialize};

pub async fn handler(_input: Json<Input>) -> Json<Output> {
    let resp = Output {
        places: vec![Place {
            place: "< place >".to_string(),
        }],
        google_map_url: "< url >".to_string(),
    };
    Json(resp)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Input {
    pub area: String,
    pub candidate_count: u8,
    pub travel_mode: TravelMode,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Output {
    pub places: Vec<Place>,
    pub google_map_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Place {
    pub place: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TravelMode {
    #[serde(rename = "walking")]
    Walking,
    #[serde(rename = "bicycling")]
    Driving,
}
