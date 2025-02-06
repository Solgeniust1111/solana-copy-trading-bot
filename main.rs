use copy_trading_bot::common::utils::{
    create_nonblocking_rpc_client, create_rpc_client, import_env_var, import_wallet, AppState,
};
use copy_trading_bot::dex::raydium::{get_pool_state, get_pool_state_by_mint};
use copy_trading_bot::engine::swap::raydium_swap;
use copy_trading_bot::ray_parse::tx_parse::{self, tx_parse};
use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use solana_client::rpc_client::{self, RpcClient};
use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::commitment_config::{self, CommitmentConfig};
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::str::FromStr;
use tokio::time::Instant;
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
#[tokio::main]

async fn main() {
    dotenv().ok();

    let sol_address = env::var("SOL_PUBKEY").expect("SOL_PUBKEY not set");
    let rpc_https_url = env::var("RPC_ENDPOINT").expect("RPC_ENDPOINT not set");
    let rpc_client = RpcClient::new(rpc_https_url.clone());
    let unwanted_key = env::var("JUP_PUBKEY").expect("JUP_PUBKEY not set");
    let target = env::var("TARGET_PUBKEY").expect("TARGET_PUBKEY not set");
    let ws_url = "wss://atlas-mainnet.helius-rpc.com/?api-key=27fd6baa-75e9-4d39-9832-d5a43419ad78";
    let (ws_stream, _) = connect_async(ws_url)
        .await
        .expect("Failed to connect to WebSocket server");
    let (mut write, mut read) = ws_stream.split();
    // Subscribe to logs
    let subscription_message = serde_json::json!({});

    write
        .send(subscription_message.to_string().into())
        .await
        .expect("Failed to send subscription message");

    // Listen for messages
    while let Some(Ok(msg)) = read.next().await {
        if let WsMessage::Text(text) = msg {
            let json: Value = serde_json::from_str(&text).unwrap();

            let sig = json["params"]["result"]["signature"].to_string();

            if let Some(inner_instructions) =
                json["params"]["result"]["transaction"]["meta"]["innerInstructions"].as_array()
            {

                //filter raydium swap tx

                //fiter pumpfun swap tx
            }
        }
    }
}

// Listen all events with websocket

pub async fn swap_to_events(mint: String, amount_in: f64, dirs: String) {

    let start_time = Instant::now();
    let (pool_id, pool_state) = match get_pool_state_by_mint(rpc_client.clone(), &mint).await {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error fetching pool state: {}", err);
            return; // Propagates the error if needed
        }
    };
    println!("get pool state ellapsed: {:?}", start_time.elapsed());
    let state = AppState {
        rpc_client,
        rpc_nonblocking_client,
        wallet,
    };

    println!("amount_in: {:#?}", amount_in.clone());
    println!("start swap ellapsed: {:?}", start_time.elapsed());

    // contact me
    let res = jup_swap(
        amount_in.clone(),
        &dirs,
        mint
    )
    .await;
    println!("res: {:#?}", res);
}
