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
    if !is_validation_prefecture(input.area.as_str()) {
        return Err(AppError(anyhow!("都道府県名が不正です")));
    }
    let mut conversation = state
        .chat_gpt_client
        .new_conversation_directed("あなたは日本旅行の素晴らしいプランナーです。");
    let content = format!(
        "都道府県である{}のおすすめの観光名所を合計{}個教えてください。レスポンスは次のJSON形式で返してください。JSON以外の説明は不要です。\
        {{\"places\": [\"\"]}}\
        存在しない都道府県の入力があった場合もしくは候補地がない場合は以下のJSONのみを返してください。JSON以外の説明文は不要です\
        {{\"places\": []}}",
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

fn is_validation_prefecture(area: &str) -> bool {
    let prefectures = vec![
        "北海道",
        "青森",
        "岩手",
        "宮城",
        "秋田",
        "山形",
        "福島",
        "茨城",
        "栃木",
        "群馬",
        "埼玉",
        "千葉",
        "東京",
        "神奈川",
        "新潟",
        "富山",
        "石川",
        "福井",
        "山梨",
        "長野",
        "岐阜",
        "静岡",
        "愛知",
        "三重",
        "滋賀",
        "京都",
        "大阪",
        "兵庫",
        "奈良",
        "和歌山",
        "鳥取",
        "島根",
        "岡山",
        "広島",
        "山口",
        "徳島",
        "香川",
        "愛媛",
        "高知",
        "福岡",
        "佐賀",
        "長崎",
        "熊本",
        "大分",
        "宮崎",
        "鹿児島",
        "沖縄",
    ];
    prefectures.contains(&area)
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
