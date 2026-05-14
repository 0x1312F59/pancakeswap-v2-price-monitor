use anyhow::Result;
use ethers::prelude::*;
use ethers::utils::{format_units, parse_units};
use std::{env, sync::Arc, time::Duration};

// PancakeSwap V2 Router interface
abigen!(
    PancakeRouter,
    r#"[
        function getAmountsOut(uint256,address[]) external view returns (uint256[])
    ]"#
);

// Minimal ERC20 interface for fetching token symbol
abigen!(
    ERC20,
    r#"[
        function symbol() external view returns (string)
    ]"#
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // Load RPC endpoint from .env
    let rpc = env::var("RPC_WSS")?;

    // Input token address (example: USDT)
    let token_in: Address = env::var("TOKEN_IN")?.parse()?;

    // Amount to simulate in swap quote
    let amount_in: f64 = env::var("AMOUNT_IN")?.parse()?;

    // PancakeSwap V2 Router address (BSC Mainnet)
    let router_addr: Address = "0x10ED43C718714eb63d5aA57B78B54704E256024E".parse()?;

    // Create WebSocket provider
    let ws = Ws::connect(rpc).await?;
    let provider = Provider::new(ws).interval(Duration::from_millis(500));

    let client = Arc::new(provider);

    // Initialize ERC20 contract instance
    let token = ERC20::new(token_in, client.clone());

    // Fetch token ticker/symbol
    let symbol = token.symbol().call().await?;

    // Initialize PancakeSwap router contract
    let router = PancakeRouter::new(router_addr, client.clone());

    loop {
        match get_price(&router, token_in, amount_in).await {
            Ok(price) => {
                println!(
                    "[{}] 1 {} = {} USDT",
                    chrono::Local::now().format("%H:%M:%S"),
                    symbol,
                    price
                );
            }
            Err(err) => {
                eprintln!("Error: {:#?}", err);
            }
        }

        // Poll every second
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

/// Fetch token price using PancakeSwap V2 routing:
/// TOKEN -> WBNB -> USDT
async fn get_price(
    router: &PancakeRouter<Provider<Ws>>,
    token_in: Address,
    amount_in: f64,
) -> Result<String> {
    // Convert input amount into wei format
    let amount = parse_units(amount_in.to_string(), 18)?;

    // Wrapped BNB address
    let wbnb: Address = "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c".parse()?;

    // Binance-Peg USDT address
    let usdt: Address = "0x55d398326f99059fF775485246999027B3197955".parse()?;

    // Swap path for quote calculation
    let path = vec![token_in, wbnb, usdt];

    // Query expected output amounts
    let amounts = router.get_amounts_out(amount.into(), path).call().await?;

    let out = amounts.last().unwrap();

    // Convert USDT output from wei to readable format
    let formatted = format_units(*out, 18)?;

    let price: f64 = formatted.parse()?;

    // Limit output to 6 decimal places
    Ok(format!("{:.6}", price))
}
