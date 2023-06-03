use axum::response::Json;
use serde::Serialize;
use serde_json::{json, Value};

pub async fn handler(body_json: Json<Value>) -> Json<Value> {
    let area = body_json.0.get("area").unwrap().as_str().unwrap();
    let candidate_count = body_json
        .0
        .get("candidate_count")
        .unwrap()
        .as_str()
        .unwrap();
    let travel_mode = body_json.0.get("travel_mode").unwrap().as_str().unwrap();
    let _input = Input {
        area: area.to_string(),
        candidate_count: candidate_count.parse::<u8>().unwrap(),
        travel_mode: match travel_mode {
            "walking" => TravelMode::Walking,
            "driving" => TravelMode::Driving,
            _ => panic!("invalid travel_mode"),
        },
    };
    let response = dummy().await.unwrap();
    Json(json!({ "response": response }))
}

async fn dummy() -> Result<Output, anyhow::Error> {
    let dummy_response = Output {
        places: vec![Place {
            place: "dummy".to_string(),
        }],
        google_map_url: "dummy".to_string(),
    };
    Ok(dummy_response)
}

#[derive(Debug, Serialize)]
struct Input {
    area: String,
    candidate_count: u8,
    travel_mode: TravelMode,
}

#[derive(Debug, Serialize)]
struct Output {
    places: Vec<Place>,
    google_map_url: String,
}

#[derive(Debug, Serialize)]
struct Place {
    place: String,
}

#[derive(Debug, Serialize)]
enum TravelMode {
    Walking,
    Driving,
}
