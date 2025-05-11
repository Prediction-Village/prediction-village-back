use std::sync::Arc;

use anchor_client::{Client, Cluster, Program};
use solana_sdk::{pubkey::Pubkey, signature::{read_keypair_file, Keypair}, signer::Signer};

use crate::variables::{KEY_PATH, PROGRAM_ID, SOLANA_RPC_URL, SOLANA_RPC_WS_URL};

pub async fn get_payer() -> anyhow::Result<Keypair> {
    let handle = tokio::task::spawn_blocking(move || {
        read_keypair_file(&*KEY_PATH).map_err(|e| anyhow::anyhow!("Failed to read keypair file: {}", e))
    });

    let keypair = handle.await??;

    Ok(keypair)
}

pub fn get_game_pda() -> anyhow::Result<Pubkey> {
    let payer = read_keypair_file(&*KEY_PATH).map_err(|e| anyhow::anyhow!("Failed to read keypair file: {}", e))?;

    let (game_pda, _) = Pubkey::find_program_address(
        &[b"game", payer.pubkey().as_ref()],
        &PROGRAM_ID,
    );

    Ok(game_pda)
}
