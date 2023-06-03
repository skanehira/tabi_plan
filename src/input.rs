use serde::{Deserialize, Serialize};

pub mod spots {
    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Input {
        pub area: String,
        pub candidate_count: u8,
    }
}

pub mod routes {
    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Input {
        pub origin: String,
        pub destination: String,
        pub waypoints: Vec<String>,
        pub travel_mode: TravelMode,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub enum TravelMode {
        #[serde(rename = "walking")]
        Walking,
        #[serde(rename = "driving")]
        Driving,
    }

    impl ToString for TravelMode {
        fn to_string(&self) -> String {
            match self {
                TravelMode::Walking => "walking".to_string(),
                TravelMode::Driving => "driving".to_string(),
            }
        }
    }
}
