use std::{env, str::FromStr};

use solana_sdk::pubkey::Pubkey;



lazy_static::lazy_static! {
    pub static ref IDL_PATH: String = "../../../assets/idl/prediction_village.json".to_string();
    pub static ref KEY_PATH: String = env::var("KEY_PATH").and_then(|path| Ok(shellexpand::tilde(path.as_str()).to_string())).unwrap().to_string();
    pub static ref PROGRAM_ID: Pubkey = env::var("PROGRAM_ID").and_then(|id| Ok(Pubkey::from_str(id.as_str()).unwrap())).unwrap();
    pub static ref SOLANA_RPC_URL: String = env::var("SOLANA_RPC_URL").unwrap_or("https://api.devnet.solana.com".to_string());
    pub static ref SOLANA_RPC_WS_URL: String = env::var("SOLANA_RPC_URL").unwrap_or("ws://api.devnet.solana.com".to_string());
}

pub fn check_env() {
    dotenv::dotenv().ok();
    println!("env KEY_PATH = {}", *KEY_PATH);
    println!("env PROGRAM_ID = {}", *PROGRAM_ID);
    println!("env SOLANA_RPC_URL = {}", *SOLANA_RPC_URL);
    println!("env SOLANA_RPC_WS_URL = {}", *SOLANA_RPC_WS_URL);
}
