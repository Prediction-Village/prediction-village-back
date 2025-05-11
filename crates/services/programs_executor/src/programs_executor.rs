use serde_json::Value;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use std::fs::File;
use std::path::Path;
use anyhow::Result;

use crate::utils::{get_game_pda, get_payer};
use crate::variables::{IDL_PATH, PROGRAM_ID, SOLANA_RPC_URL};

pub struct ProgramsExecutor {
    pub payer_bytes: [u8; 64],
    pub game_pda: Pubkey,
    pub rpc_client: RpcClient,
}

impl ProgramsExecutor {
    pub async fn new() -> Result<Self> {
        let payer =  get_payer().await?;
        let payer_bytes = payer.to_bytes();

        let game_pda = get_game_pda()?;

        let rpc_client = RpcClient::new(SOLANA_RPC_URL.clone());

        Ok(Self { payer_bytes, game_pda, rpc_client })
    }

    pub fn init_game(&self) -> Result<()> {
        let data = Self::load_discriminator("init_game")?;
        let accounts = vec![
            AccountMeta::new(self.game_pda, false),
            AccountMeta::new(self.payer().pubkey(), true),
            AccountMeta::new_readonly(anchor_client::solana_sdk::system_program::ID, false),
        ];  

        let init_game_ix = Instruction { program_id: *PROGRAM_ID, accounts, data };

        self.send_and_confirm_transaction(vec![init_game_ix]);

        Ok(())
    }

    pub fn restart_game(&self) -> Result<()> {
        let data = Self::load_discriminator("restart_game")?;
        let accounts = vec![
            AccountMeta::new(self.game_pda, false),
            AccountMeta::new(self.payer().pubkey(), true),
        ];

        let restart_game_ix = Instruction { program_id: *PROGRAM_ID, accounts, data };

        self.send_and_confirm_transaction(vec![restart_game_ix]);

        Ok(())
    }

    pub fn generate_income(&self) -> Result<()> {
        let data = Self::load_discriminator("generate_income")?;
        let accounts = vec![
            AccountMeta::new(self.game_pda, false),
            AccountMeta::new(self.payer().pubkey(), true),
        ];

        let generate_income_ix = Instruction { program_id: *PROGRAM_ID, accounts, data };

        self.send_and_confirm_transaction(vec![generate_income_ix]);

        Ok(())
    }

    pub fn payer(&self) -> Keypair {
        Keypair::from_bytes(&self.payer_bytes).unwrap()
    }

    pub fn load_discriminator(ix_name: &str) -> anyhow::Result<Vec<u8>> {
        let file = File::open(Path::new(&*IDL_PATH))?;
        let json: Value = serde_json::from_reader(&file)?;

        let instructions = json["instructions"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("No 'instructions' in IDL"))?;

        let instr = instructions
            .iter()
            .find(|ix| ix["name"] == ix_name)
            .ok_or_else(|| anyhow::anyhow!("Instruction '{}' not found in IDL", ix_name))?;

        let discriminator = instr["discriminator"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("No discriminator for instruction"))?;

        Ok(discriminator
            .iter()
            .map(|v| v.as_u64().unwrap() as u8)
            .collect())
    }

    pub fn send_and_confirm_transaction(&self, instructions: Vec<Instruction>) -> anyhow::Result<()> {
        let payer = self.payer();

        let block_hash = self.rpc_client.get_latest_blockhash()?;

        let tx = Transaction::new_signed_with_payer(
            &instructions,
            Some(&payer.pubkey()),
            &[&payer],
            block_hash,
        );

        let sig = self.rpc_client.send_and_confirm_transaction(&tx)?;
        println!("✅ Signature: {}", sig);

        Ok(())
    }
}