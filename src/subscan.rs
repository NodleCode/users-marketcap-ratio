use anyhow::{bail, Result};
use hyper::{client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

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

static MUTEX: Lazy<Mutex<i64>> = Lazy::new(|| Mutex::new(0));

pub async fn active_wallets(
    client: Client<HttpsConnector<HttpConnector>>,
    network: &str,
) -> Result<u64> {
    // unfortunately subscan's API is very limited and I am not gonna
    // pay hundreds of dollars to run a simple tiny script, as such
    // we hack a rate limiting feature via the code below
    let mut last = MUTEX.lock().await;
    let now = chrono::Utc::now().timestamp();
    if now - *last < 1 {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    *last = now;
    drop(last);

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
