use crate::{
    client::GoogleMapClient, config::AppState, error::AppError, input::routes::Input,
    output::routes::Output,
};
use axum::{extract::State, response::Json};
use reqwest::Url;
use std::sync::Arc;

pub async fn get_routes<G: GoogleMapClient>(
    State(state): State<Arc<AppState<G>>>,
    input: Json<Input>,
) -> Result<Json<Output>, AppError> {
    let travel_mode = input.travel_mode.to_string();
    let dirs = state.google_map_client.routes(input.0).await?;

    let err = Err(AppError {
        message: "routes were not found".into(),
    });

    if dirs.routes.is_empty() {
        eprintln!("route is empty");
        return err;
    }
    if dirs.routes[0].legs.is_empty() {
        eprintln!("legs is empty");
        return err;
    }

    let legs = &dirs.routes[0].legs;
    let origin = &legs[0].start_address;
    let destination = &legs[legs.len() - 1].end_address;
    let waypoints = legs[0..legs.len() - 1]
        .iter()
        .map(|leg| leg.end_address.clone())
        .collect::<Vec<_>>();

    let mut queries = vec![
        ("api", "1"),
        ("origin", origin),
        ("destination", destination),
        ("mode", &travel_mode),
    ];

    let has_waypoints = !waypoints.is_empty();
    let waypoints = waypoints.join("|");
    if has_waypoints {
        queries.push(("waypoints", &waypoints));
    }

    let url = Url::parse_with_params("https://www.google.com/maps/dir", &queries)?.to_string();

    let resp = Output {
        google_map_url: url,
    };
    Ok(Json(resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::GoogleMapClient, config, input::routes::TravelMode, output::map::Directions,
    };
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        routing::{post, Router},
    };
    use chatgpt::prelude::ChatGPT;
    use clap::Parser as _;
    use tower::ServiceExt;

    struct MockGoogleMapClient {
        directions: Option<Directions>,
        error: Option<AppError>,
    }

    #[async_trait::async_trait]
    impl GoogleMapClient for MockGoogleMapClient {
        async fn routes(&self, _: Input) -> Result<Directions, AppError> {
            match (&self.directions, &self.error) {
                (Some(directions), None) => Ok(directions.clone()),
                (None, Some(_)) => Err(AppError {
                    message: "no any directions".into(),
                }),
                _ => unreachable!(),
            }
        }
    }
    async fn run_test(input: Input) -> anyhow::Result<StatusCode> {
        let config = config::AppConfig::try_parse()?;
        let token = config.open_chat.open_chat_api_key.clone();
        let bytes = include_bytes!("fixtures/directions.json");
        let directions: Directions = serde_json::from_slice(bytes)?;
        let client = MockGoogleMapClient {
            directions: Some(directions),
            error: None,
        };
        let state = config::AppState {
            config,
            chat_gpt_client: ChatGPT::new(token)?,
            google_map_client: client,
        };

        let state = Arc::new(state);
        let app = Router::new()
            .route("/routes", post(get_routes))
            .with_state(Arc::clone(&state));
        let json = serde_json::to_vec(&input)?;
        let body = Body::from(json);

        let req = Request::builder()
            .method(http::Method::POST)
            .uri("/routes")
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(body)?;

        let resp = app.oneshot(req).await?;
        Ok(resp.status())
    }

    #[tokio::test]
    async fn test_get_routes_returns_all_routes() -> anyhow::Result<()> {
        let input = Input {
            origin: "大阪城".into(),
            destination: "USJ".into(),
            waypoints: vec!["道頓堀".into(), "通天閣".into(), "天保山".into()],
            travel_mode: TravelMode::Driving,
        };
        let result = run_test(input).await?;
        assert_eq!(result, StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_routes_returns_ok_when_request_no_waypoints() -> anyhow::Result<()> {
        let input = Input {
            origin: "大阪城".into(),
            destination: "USJ".into(),
            waypoints: vec![],
            travel_mode: TravelMode::Driving,
        };
        let result = run_test(input).await?;
        assert_eq!(result, StatusCode::OK);
        Ok(())
    }
}
