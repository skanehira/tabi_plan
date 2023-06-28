use crate::client::GoogleMapClient;
use crate::{config::AppState, error::AppError, input::spots::Input, output::spots::Output};
use axum::extract::State;
use axum::response::Json;
use once_cell::sync::Lazy;
use std::sync::Arc;

static PREFECTURES: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
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
    ]
});

pub async fn get_spots<G: GoogleMapClient>(
    State(state): State<Arc<AppState<G>>>,
    input: Json<Input>,
) -> Result<Json<Output>, AppError> {
    if !is_valid_prefecture(input.area.as_str()) {
        return Err(AppError {
            message: "該当の都道府県は存在しません".into(),
        });
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
    if message.contains("{\"places\": []}") {
        return Err(AppError {
            message: "該当の都道府県は存在しません".into(),
        });
    }
    let output: Output = serde_json::from_str(message.as_str())?;
    Ok(Json(output))
}

fn is_valid_prefecture(area: &str) -> bool {
    let area = area.replace(['都', '府', '県'], "");
    PREFECTURES.contains(&area.as_str())
}
