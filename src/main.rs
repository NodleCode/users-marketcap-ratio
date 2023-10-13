use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DailyReqArgs {
    end: String,
    start: String,
    category: String,
    format: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DailyRes {
    code: u64,
    message: String,
    generated_at: u64,
    data: DailyResData,
}

#[derive(Debug, Serialize, Deserialize)]
struct DailyResData {
    list: Vec<DailyResDataList>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DailyResDataList {
    time_utc: String,
    total: String,
    transfer_amount: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let endpoints = vec!["nodle"];

    let today = chrono::Utc::now();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let args = DailyReqArgs {
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
            endpoints[0]
        ))
        .header("content-type", "application/json")
        .body(Body::from(body))?;

    let res = client.request(req).await.unwrap();

    if res.status() != 200 {
        println!("error: {}", res.status());
        return Ok(());
    }

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let res: DailyRes = serde_json::from_slice(&body).unwrap();

    println!("{:?}", res);

    Ok(())
}
