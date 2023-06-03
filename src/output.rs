pub mod map {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Directions {
        #[serde(rename = "geocoded_waypoints")]
        pub geocoded_waypoints: Vec<GeocodedWaypoint>,
        pub routes: Vec<Route>,
        pub status: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GeocodedWaypoint {
        #[serde(rename = "geocoder_status")]
        pub geocoder_status: String,
        #[serde(rename = "place_id")]
        pub place_id: String,
        pub types: Vec<String>,
        #[serde(rename = "partial_match")]
        pub partial_match: Option<bool>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Route {
        pub bounds: Bounds,
        pub copyrights: String,
        pub legs: Vec<Leg>,
        #[serde(rename = "overview_polyline")]
        pub overview_polyline: OverviewPolyline,
        pub summary: String,
        pub warnings: Vec<Value>,
        #[serde(rename = "waypoint_order")]
        pub waypoint_order: Vec<i64>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Bounds {
        pub northeast: Northeast,
        pub southwest: Southwest,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Northeast {
        pub lat: f64,
        pub lng: f64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Southwest {
        pub lat: f64,
        pub lng: f64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Leg {
        pub distance: Distance,
        pub duration: Duration,
        #[serde(rename = "end_address")]
        pub end_address: String,
        #[serde(rename = "end_location")]
        pub end_location: EndLocation,
        #[serde(rename = "start_address")]
        pub start_address: String,
        #[serde(rename = "start_location")]
        pub start_location: StartLocation,
        pub steps: Vec<Step>,
        #[serde(rename = "traffic_speed_entry")]
        pub traffic_speed_entry: Vec<Value>,
        #[serde(rename = "via_waypoint")]
        pub via_waypoint: Vec<Value>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Distance {
        pub text: String,
        pub value: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Duration {
        pub text: String,
        pub value: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EndLocation {
        pub lat: f64,
        pub lng: f64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StartLocation {
        pub lat: f64,
        pub lng: f64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Step {
        pub distance: Distance2,
        pub duration: Duration2,
        #[serde(rename = "end_location")]
        pub end_location: EndLocation2,
        #[serde(rename = "html_instructions")]
        pub html_instructions: String,
        pub polyline: Polyline,
        #[serde(rename = "start_location")]
        pub start_location: StartLocation2,
        #[serde(rename = "travel_mode")]
        pub travel_mode: String,
        pub maneuver: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Distance2 {
        pub text: String,
        pub value: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Duration2 {
        pub text: String,
        pub value: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EndLocation2 {
        pub lat: f64,
        pub lng: f64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Polyline {
        pub points: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StartLocation2 {
        pub lat: f64,
        pub lng: f64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OverviewPolyline {
        pub points: String,
    }
}
