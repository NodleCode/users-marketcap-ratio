use hyper::Client;
use hyper_tls::HttpsConnector;

mod coingecko;
mod subscan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // subscan ID, coingecko ID
    let networks = vec![("nodle", "nodle-network")];

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    for network in networks {
        let wallets = subscan::active_wallets(client.clone(), network.0).await?;
        let marketcap = coingecko::marketcap(client.clone(), network.1).await?;

        println!(
            "{}: {} wallets for a marketcap of ${}",
            network.0, wallets, marketcap
        );
    }

    Ok(())
}
