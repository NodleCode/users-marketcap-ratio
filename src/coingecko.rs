use anyhow::{bail, Result};
use hyper::{client::HttpConnector, Client, Uri};
use hyper_tls::HttpsConnector;

mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct MarketRes {
        pub prices: Vec<(f64, f64)>,
        pub market_caps: Vec<(f64, f64)>,
        pub total_volumes: Vec<(f64, f64)>,
    }
}

pub async fn marketcap(
    client: Client<HttpsConnector<HttpConnector>>,
    network: &str,
) -> Result<f64> {
    let uri: Uri = format!("https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency=usd&days=0&interval=daily", network).parse()?;
    let req = client.get(uri).await?;

    let body = hyper::body::to_bytes(req.into_body()).await?;
    if body.is_empty() {
        bail!("empty body");
    }

    let res: types::MarketRes = serde_json::from_slice(&body)?;
    Ok(res.market_caps[0].1)
}
