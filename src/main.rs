use hyper::Client;
use hyper_tls::HttpsConnector;

mod subscan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let networks = vec!["nodle"];

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    for network in networks {
        let stats = subscan::daily_stats(client.clone(), network).await?;
        println!("--- stats for {}\n{:?}", network, stats);
    }

    Ok(())
}
