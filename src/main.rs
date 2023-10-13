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
    network: &str,
    subscan_id: &str,
    coingecko_id: &str,
) -> Result<NetworkAnalysis> {
    let (active_wallets, marketcap) = futures::try_join!(
        subscan::active_wallets(client.clone(), subscan_id),
        coingecko::marketcap(client.clone(), coingecko_id)
    )?;

    let ratio = active_wallets as f64 / marketcap;

    Ok(NetworkAnalysis {
        network: network.to_string(),
        active_wallets,
        marketcap,
        ratio,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // name, subscan ID, coingecko ID
    let networks = vec![
        ("nodle", "nodle", "nodle-network"),
        ("polkadot", "polkadot", "polkadot"),
        ("kusama", "kusama", "kusama"),
        ("moonbeam", "moonbeam", "moonbeam"),
        ("astar", "astar", "astar"),
        ("acala", "acala", "acala"),
        ("kilt", "spiritnet", "kilt-protocol"),
        ("centrifuge", "centrifuge", "centrifuge"),
        ("hydradx", "hydradx", "hydradx"),
    ];

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let pending = networks
        .iter()
        .map(|network| analyze_network(client.clone(), network.0, network.1, network.2));

    let results = futures::future::join_all(pending).await;

    println!("network name, daily active wallets, marketcap (usd), ratio (daily active wallets / marketcap)");
    for result in results {
        let analysis = result?;
        println!(
            "{}, {}, {}, {}",
            analysis.network, analysis.active_wallets, analysis.marketcap, analysis.ratio,
        );
    }

    Ok(())
}
