#![allow(non_snake_case)]
extern crate serde;
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GamesResponse {
    pub game_shipments_any: Option<Vec<Game>>,
    pub error: ErrorStruct,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub title: Option<String>,
    pub cover: Option<String>,
    pub preview: Option<String>,
    pub category_codes: Option<Vec<String>>,
    pub platforms: Option<Vec<String>>,
    pub code: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorStruct {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Genre {
    pub code: Option<String>,
    pub name: Option<String>,
    pub is_main: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub code: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputDevice {
    pub code: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameter {
    pub parameter: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Run {
    pub name: Option<String>,
    pub code: Option<String>,
    pub priority: Option<i32>,
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GamePlatform {
    pub code: Option<String>,
    pub name: Option<String>,
    pub link: Option<String>,
    pub img: Option<String>,
    pub badge: Option<String>,
    pub circle: Option<String>,
    pub icon68: Option<String>,
    pub iconNewCatalog: Option<String>,
    pub icon20: Option<String>,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardInfo {
    pub game_card_type: Option<String>,
    pub has_play_attempt: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChildGame {
    pub code: Option<String>,
    pub game_status_code: Option<String>,
    pub run: Option<Vec<Run>>,
    pub game_platforms: Option<Vec<GamePlatform>>,
    pub game_platform: Option<GamePlatform>,
    pub platform_code: Option<String>,
    pub card_info: Option<CardInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameFull {
    pub code: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub year: Option<i32>,
    pub plot_outline: Option<String>,
    pub writer: Option<String>,
    pub premiered: Option<String>,
    pub rating: Option<i32>,
    pub mpaa: Option<String>,
    pub demo_time: Option<i32>,
    pub rightholder: Option<String>,
    pub content: Option<Vec<Content>>,
    pub genres: Option<Vec<Genre>>,
    pub products: Option<Vec<Product>>,
    pub count_players: Option<i32>,
    pub input_devices: Option<Vec<InputDevice>>,
    pub categories: Option<Vec<Category>>,
    pub run: Option<Vec<Run>>,
    pub game_platforms: Option<Vec<GamePlatform>>,
    pub game_platform: Option<GamePlatform>,
    pub platform_code: Option<String>,
    pub game_status: Option<String>,
    pub child_game: Option<ChildGame>,
    pub child_games: Option<Vec<ChildGame>>,
    pub available_in_regions: Option<Vec<String>>,
    pub game_shipment_any_type: Option<String>,
    pub main_game_code: Option<String>,
    pub main_game_title: Option<String>,
    pub card_info: Option<CardInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GamesFullResponse {
    pub game_shipments_any: Option<Vec<GameFull>>,
    pub error: ErrorStruct,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameRequest {
    pub filter_categories_and_genres: Option<Vec<String>>,
    pub filter_codes: Option<Vec<String>>,
    pub filter_name: Option<String>,
    pub ga_clientId: Option<String>,
    pub lang: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryRequest {
    pub ga_clientId: Option<String>,
    pub lang: Option<String>,
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub code: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    pub r#type: Option<String>,
    pub links: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoriesResponse {
    pub categories: Option<Vec<Category>>,
    pub error: Option<ErrorStruct>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRunURLRequest {
    pub lang: Option<String>,
    pub token: Option<String>,
    pub code_game: Option<String>,
    pub play_config: Option<String>,
    pub ga_clientId: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRunURLResponse {
    pub error: Option<ErrorStruct>,
    pub key: Option<String>,
    pub active_to: Option<String>,
    pub play_url: Option<String>,
}
