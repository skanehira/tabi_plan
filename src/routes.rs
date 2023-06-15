use crate::{
    config::AppState,
    error::AppError,
    input::routes::Input,
    output::{map::Directions, routes::Output},
};
use axum::{extract::State, response::Json};
use reqwest::Url;
use std::sync::Arc;

pub async fn get_routes(
    State(state): State<Arc<AppState>>,
    input: Json<Input>,
) -> Result<Json<Output>, AppError> {
    let travel_mode = input.travel_mode.to_string();
    let url = Url::parse_with_params(
        &state.config.google_map.google_map_endpoint,
        &[
            ("origin", &input.origin),
            ("destination", &input.destination),
            ("waypoints", &input.waypoints.join("|")),
            ("mode", &travel_mode),
            ("key", &state.config.google_map.google_map_api_key),
        ],
    )?;

    let req = state.google_map_client.get(url);
    let dirs: Directions = req.send().await?.json().await?;

    let legs = &dirs.routes[0].legs;
    let origin = &legs[0].start_address;
    let destination = &legs[legs.len() - 1].end_address;
    let waypoints = legs[0..legs.len() - 1]
        .iter()
        .map(|leg| leg.end_address.clone())
        .collect::<Vec<_>>()
        .join("|");

    let url = format!(
        "https://www.google.com/maps/dir/?api=1&origin={}&destination={}&waypoints={}&travelmode={}",
        origin, destination, waypoints, travel_mode
    );

    let resp = Output {
        google_map_url: url,
    };
    Ok(Json(resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config, input::routes::TravelMode};
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        routing::{post, Router},
    };
    use chatgpt::prelude::ChatGPT;
    use clap::Parser as _;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_routes() -> anyhow::Result<()> {
        let config = config::AppConfig::try_parse()?;
        let token = config.open_chat.open_chat_api_key.clone();

        let state = config::AppState {
            config,
            chat_gpt_client: ChatGPT::new(token)?,
            google_map_client: reqwest::Client::new(),
        };
        let state = Arc::new(state);
        let app = Router::new()
            .route("/routes", post(get_routes))
            .with_state(Arc::clone(&state));

        let input = Input {
            origin: "大阪城".into(),
            destination: "USJ".into(),
            waypoints: vec!["道頓堀".into(), "通天閣".into(), "天保山".into()],
            travel_mode: TravelMode::Driving,
        };
        let json = serde_json::to_vec(&input)?;
        let body = Body::from(json);

        let req = Request::builder()
            .method(http::Method::POST)
            .uri("/routes")
            .body(body)?;

        let resp = app.oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::OK);

        Ok(())
    }
}
