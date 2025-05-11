use std::sync::{Arc, Mutex};

use anchor_lang::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::{read_keypair_file, Keypair}, signer::Signer};

use crate::{utils::get_game_pda, variables::{KEY_PATH, PROGRAM_ID, SOLANA_RPC_URL}}; 


#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct Game {
    pub light_forces: Village,
    pub dark_forces: Village,
    pub status: GameStatus,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct Village {
    pub town_hall: TownHall,
    pub gold_mines: Vec<u8>, 
    pub barracks: Vec<u8>,
    pub laboratories: Vec<u8>,
    pub warriors: Vec<Warrior>,
    pub resources: Resources,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct TownHall {
    pub level: u8,
    pub health: u32,
    pub damage: u32,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct Warrior {
    pub level: u8,
    pub health: u32,
    pub damage: u32,
    pub position: Position,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}


#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct Resources {
    pub gold: u32,
    pub gold_income: u32,
    pub technologies: u32,
    pub technologies_income: u32,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, PartialEq, Eq)]
pub enum GameStatus {
    InProgress,
    Finished(u8),
}

pub async fn fetch_game_state() -> anyhow::Result<Game> {
    let game_pda = get_game_pda()?;
    let rpc_client = RpcClient::new(SOLANA_RPC_URL.clone());

    let game_data = rpc_client.get_account_data(&game_pda)?;
    let game: Game = Game::deserialize(&mut game_data.as_slice())?;

    Ok(game)
}
