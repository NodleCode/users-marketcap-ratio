# Crypto Intelligence: Computer the Daily Active Wallets to Marketcap Ratio for Polkadot Parachains
Data fetched via Subscan and Coingecko APIs.

## Example
```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/users-marketcap-ratio`
network name, daily active wallets, marketcap (usd), ratio (daily active wallets / marketcap)
nodle, 411, 8043737.610459265, 0.000051095649796629
polkadot, 35, 4779112675.358698, 0.000000007323535220347795
kusama, 10, 152467582.42419714, 0.00000006558771275180241
moonbeam, 147, 141020944.80079487, 0.0000010423983487534501
astar, 97, 224288484.878444, 0.000000432478734040093
acala, 39, 37234416.398917295, 0.0000010474180549029377
```

Output is a CSV. You may run analysis yourself.

## Adding network
Networks can be added by modifying the `main.rs` file by specifiying the network name, subscan API ID, and Coingecko ID.
