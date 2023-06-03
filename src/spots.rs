use axum::response::Json;
use chatgpt::prelude::ChatGPT;
use serde::{Deserialize, Serialize};
use std::env;

pub async fn handler(input: Json<Input>) -> Json<Output> {
    let open_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
    let resp = get_spots(&open_api_key, &input).await.unwrap();
    Json(resp)
}

async fn get_spots(open_api_key: &str, input: &Input) -> Result<Output, anyhow::Error> {
    let client = ChatGPT::new(open_api_key)?;
    let mut conversation =
        client.new_conversation_directed("あなたは日本旅行の素晴らしいプランナーです。");
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
    if message.contains("存在") {
        return Ok(Output { places: vec![] });
    }
    let output: Output = serde_json::from_str(message.as_str())?;
    Ok(Output {
        places: output.places,
    })
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
