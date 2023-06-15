use crate::{error::AppError, input::routes::Input, output::map::Directions};
use reqwest::Url;

#[async_trait::async_trait]
pub trait GoogleMapClient: Sync + Send {
    async fn routes(&self, input: Input) -> Result<Directions, AppError>;
}

pub struct GoogleMapClientImpl {
    client: reqwest::Client,
    key: String,
}

impl GoogleMapClientImpl {
    pub fn new(client: reqwest::Client, key: String) -> Self {
        Self { client, key }
    }
}

#[async_trait::async_trait]
impl GoogleMapClient for GoogleMapClientImpl {
    async fn routes(&self, input: Input) -> Result<Directions, AppError> {
        let url = Url::parse_with_params(
            "https://maps.googleapis.com/maps/api/directions/json",
            &[
                ("origin", &input.origin),
                ("destination", &input.destination),
                ("waypoints", &input.waypoints.join("|")),
                ("mode", &input.travel_mode.to_string()),
                ("key", &self.key),
            ],
        )?;
        let req = self.client.get(url);
        let dirs: Directions = req.send().await?.json().await?;
        Ok(dirs)
    }
}
