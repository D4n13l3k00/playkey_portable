use crate::structs;
use reqwest::blocking as rq;

pub fn get_categories(token: String) -> structs::CategoriesResponse {
    let request_obj = structs::CategoryRequest {
        ga_clientId: Some("ga doesn't have clientId".to_string()),
        lang: Some("ru".to_string()),
        token: Some(token),
    };
    let client = rq::Client::new();
    let json = serde_json::to_string(&request_obj).unwrap();
    let res = client
        .post("https://api.playkey.net/rest/PlaykeyAPI.svc/games/categories")
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .unwrap();
    let text: String = res.text().unwrap();
    let resp: structs::CategoriesResponse = serde_json::from_str(text.as_str()).unwrap();
    return resp;
}

pub fn get_favorite_games(token: String) -> structs::GamesResponse {
    return get_games_by_category(token, "FavouriteGames".to_string());
}

pub fn get_games_by_category(token: String, filter: String) -> structs::GamesResponse {
    let request_obj = structs::GameRequest {
        filter_categories_and_genres: Some(vec![filter]),
        filter_codes: None,
        filter_name: Some("".to_string()),
        ga_clientId: Some("ga doesn't have clientId".to_string()),
        lang: Some("ru".to_string()),
        limit: Some(50),
        offset: Some(0),
        token: Some(token),
    };
    let client = rq::Client::new();
    let json = serde_json::to_string(&request_obj).unwrap();
    let res = client
        .post("https://api.playkey.net/rest/PlaykeyAPI.svc/gameshipments/all/catalog-category/auth")
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .unwrap();
    let text: String = res.text().unwrap();
    let resp = serde_json::from_str::<structs::GamesResponse>(text.as_str()).unwrap();
    return resp;
}

pub fn get_games_by_name(token: String, name: String) -> structs::GamesResponse {
    let request_obj = structs::GameRequest {
        filter_categories_and_genres: None,
        filter_codes: None,
        filter_name: Some(name),
        ga_clientId: Some("ga doesn't have clientId".to_string()),
        lang: Some("ru".to_string()),
        limit: Some(50),
        offset: Some(0),
        token: Some(token),
    };
    let client = rq::Client::new();
    let json = serde_json::to_string(&request_obj).unwrap();
    let res = client
        .post("https://api.playkey.net/rest/PlaykeyAPI.svc/gameshipments/all/catalog-category/auth")
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .unwrap();
    let text: String = res.text().unwrap();
    let resp = serde_json::from_str::<structs::GamesResponse>(text.as_str()).unwrap();
    return resp;
}

pub fn get_game_by_code(token: String, code: String) -> structs::GameFull {
    let request_obj = structs::GameRequest {
        filter_categories_and_genres: None,
        filter_codes: Some(vec![code]),
        filter_name: None,
        ga_clientId: Some("ga doesn't have clientId".to_string()),
        lang: Some("ru".to_string()),
        limit: Some(50),
        offset: Some(0),
        token: Some(token),
    };
    let client = rq::Client::new();
    let json = serde_json::to_string(&request_obj).unwrap();
    let res = client
        .post("https://api.playkey.net/rest/PlaykeyAPI.svc/gameshipments/all/auth")
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .unwrap();
    let text: String = res.text().unwrap();
    let resp = serde_json::from_str::<structs::GamesFullResponse>(text.as_str()).unwrap().game_shipments_any.unwrap()[0].clone();
    return resp;
}

pub fn get_run_url(token: String, code: String, play_conf: String) -> String {
    let request_obj = structs::GetRunURLRequest {
        ga_clientId: Some("ga doesn't have clientId".to_string()),
        lang: Some("ru".to_string()),
        token: Some(token),
        code_game: Some(code),
        play_config: Some(play_conf),
    };
    let client = rq::Client::new();
    let json = serde_json::to_string(&request_obj).unwrap();
    let res = client
        .post("https://api.playkey.net/rest/PlaykeyAPI.svc/games/play")
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .unwrap();
    let text: String = res.text().unwrap();
    let resp = serde_json::from_str::<structs::GetRunURLResponse>(text.as_str()).unwrap().play_url.unwrap();
    return resp;
}
