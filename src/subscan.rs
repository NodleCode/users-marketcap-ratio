use anyhow::{bail, Result};
use hyper::{client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;

mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DailyReqArgs {
        pub end: String,
        pub start: String,
        pub category: String,
        pub format: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DailyRes {
        pub code: u64,
        pub message: String,
        pub generated_at: u64,
        pub data: DailyResData,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DailyResData {
        pub list: Vec<DailyResDataList>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DailyResDataList {
        pub time_utc: String,
        pub total: String,
        pub transfer_amount: String,
    }
}

pub async fn active_wallets(
    client: Client<HttpsConnector<HttpConnector>>,
    network: &str,
) -> Result<u64> {
    let today = chrono::Utc::now();
    let args = types::DailyReqArgs {
        end: today.format("%Y-%m-%d").to_string(),
        start: today.format("%Y-%m-%d").to_string(),
        category: "ActiveAccount".to_string(),
        format: "day".to_string(),
    };
    let body = serde_json::to_string(&args).unwrap();

    let req = Request::builder()
        .method(Method::POST)
        .uri(format!(
            "https://{}.api.subscan.io/api/v2/scan/daily",
            network,
        ))
        .header("content-type", "application/json")
        .body(Body::from(body))?;

    let res = client.request(req).await.unwrap();
    if res.status() != 200 {
        bail!("subscan error: {}", res.status());
    }

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let res: types::DailyRes = serde_json::from_slice(&body).unwrap();

    Ok(res.data.list[0].total.parse::<u64>().unwrap())
}
