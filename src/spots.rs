use crate::{config::AppState, error::AppError};
use anyhow::anyhow;
use axum::extract::State;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub async fn get_spots(
    State(state): State<Arc<AppState>>,
    input: Json<Input>,
) -> Result<Json<Output>, AppError> {
    let mut conversation = state
        .chat_gpt_client
        .new_conversation_directed("あなたは日本旅行の素晴らしいプランナーです。");
    let content = format!(
        "都道府県である{}のおすすめの観光名所を合計{}個教えてください。レスポンスは次のJSON形式で返してください。JSON以外の説明は不要です。\n{{\
          \"places\": [\"\"\
          ]\
        }}\n存在しない都道府県の入力があった場合もしくは候補地がない場合は以下のJSONのみを返してください。JSON以外の説明文は不要です\n\n{{\
          \"places\": []\
        }}",
        input.area, input.candidate_count
    );
    let response = conversation.send_message(content).await?;
    let message = response.message().content.to_string();
    if message.contains("\n\n{\"places\": []}") {
        return Err(AppError(anyhow!("該当の都道府県は存在しません")));
    }
    let output: Output = serde_json::from_str(message.as_str())?;
    Ok(Json(output))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Input {
    pub area: String,
    pub candidate_count: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Output {
    pub places: Vec<String>,
}
