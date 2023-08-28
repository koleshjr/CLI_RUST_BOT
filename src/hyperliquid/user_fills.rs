use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::helpers::get_current_time_in_milliseconds;

#[derive(Deserialize, Debug)]
pub struct FillsResponse {
    pub data : UserFills,
}

#[derive(Deserialize, Debug)]

pub struct UserFills {
    pub closedpnl: String,
    pub coin: String,
    pub crossed: bool,
    pub dir: String,
    pub hash: String,   
    pub oid: u64,
    pub px: String,
    pub side: String,
    pub startposition: String,
    pub sz: String,
    pub time: u64,
}

#[derive(Serialize, Debug)]
struct RequestBody {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
    starttime: u128,
    endtime: u128,
}

pub async fn get_user_fills() -> Result<FillsResponse, Box<dyn std::error::Error>> {
    let client = Client::new();
    let request_body = RequestBody {
        request_type: "userFunding".to_string(),
        user: String::from("users onchain address"),
        starttime: get_current_time_in_milliseconds() - 1000000000,
        endtime: get_current_time_in_milliseconds(),
    };
    let json_body = serde_json::to_string(&request_body).expect("Failed to serialize the request body");
    let resp = client
        .post("https://api.hyperliquid.xyz/info")
        .body(json_body)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<FillsResponse>()
        .await?;
    Ok(resp)
}


