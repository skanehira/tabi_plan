use clap::Parser;

#[derive(Parser, Debug)]
pub struct AppConfig {
    #[command(flatten)]
    basic: BasicAuth,

    #[command(flatten)]
    open_chat: OpenChatConfig,

    #[command(flatten)]
    google_map: GoogleMapConfig,
}

// TODO: 履歴を持つ場合はユーザ情報はDBで管理する
#[derive(Clone, Debug, Parser)]
pub struct BasicAuth {
    #[arg(long, env = "BASIC_USER")]
    pub user_name: String,
    #[arg(long, env = "BASIC_PASSWORD")]
    pub password: String,
}

#[derive(Clone, Debug, Parser)]
pub struct OpenChatConfig {
    #[arg(long, env = "OPEN_CHAT_ENDPOINT")]
    pub open_chat_endpoint: String,
    #[arg(long, env = "OPEN_CHAT_API_KEY")]
    pub open_chat_api_key: String,
}

#[derive(Clone, Debug, Parser)]
pub struct GoogleMapConfig {
    #[arg(long, env = "GOOGLE_MAP_ENDPOINT")]
    pub google_map_endpoint: String,
    #[arg(long, env = "GOOGLE_MAP_API_KEY")]
    pub google_map_api_key: String,
}
