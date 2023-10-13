use anyhow::Result;
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;

mod coingecko;
mod subscan;

struct NetworkAnalysis {
    network: String,
    active_wallets: u64,
    marketcap: f64,
    ratio: f64,
}

async fn analyze_network(
    client: Client<HttpsConnector<HttpConnector>>,
    subscan_id: &str,
    coingecko_id: &str,
) -> Result<NetworkAnalysis> {
    let (active_wallets, marketcap) = futures::try_join!(
        subscan::active_wallets(client.clone(), subscan_id),
        coingecko::marketcap(client.clone(), coingecko_id)
    )?;

    let ratio = marketcap / active_wallets as f64;

    Ok(NetworkAnalysis {
        network: subscan_id.to_string(),
        active_wallets,
        marketcap,
        ratio,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // subscan ID, coingecko ID
    let networks = vec![
        ("nodle", "nodle-network"),
        ("polkadot", "polkadot"),
        ("kusama", "kusama"),
        ("moonbeam", "moonbeam"),
        ("astar", "astar"),
        ("acala", "acala"),
    ];

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let pending = networks
        .iter()
        .map(|network| analyze_network(client.clone(), network.0, network.1));

    let results = futures::future::join_all(pending).await;

    for result in results {
        let analysis = result?;
        println!(
            "{}: {} daily active wallets for a marketcap of ${} -> ratio {}",
            analysis.network, analysis.active_wallets, analysis.marketcap, analysis.ratio,
        );
    }

    Ok(())
}
