use anchor_client::solana_sdk::signer::keypair::read_keypair_file;
use anchor_client::{Client, Cluster};
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use anyhow::Result;
use std::rc::Rc;

lazy_static::lazy_static! {
    static ref IDL_PATH: String = "../../../assets/idl/prediction_village.json".to_string();
    static ref KEY_PATH: String = shellexpand::tilde("~/.config/solana/id.json").to_string();
    static ref PROGRAM_ID: Pubkey = Pubkey::from_str("7DmaWof2zqAwJXnBWyFrpQa4dXUkGVaB5WqSj5QpobaK").unwrap();
}

pub struct ProgramsExecutor {
    pub payer_bytes: [u8; 64],
    pub game_pda: Pubkey,
}

impl ProgramsExecutor {
    pub fn new() -> Result<Self> {
        let payer = read_keypair_file(&*KEY_PATH).map_err(|e| anyhow::anyhow!("Failed to read keypair file: {}", e))?;
        let payer_bytes = payer.to_bytes();

        let (game_pda, _) = Pubkey::find_program_address(
            &[b"game", payer.pubkey().as_ref()],
            &PROGRAM_ID,
        );

        Ok(Self { payer_bytes, game_pda })
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

    pub fn send_and_confirm_transaction(&self, instructions: Vec<Instruction>) {
        let payer = self.payer();
        let payer_clonned = self.payer();

        tokio::task::spawn_blocking(move || {
            let client = Client::new(
                Cluster::Devnet,
                Rc::new(payer)
            );
    
            let program = client.program(*PROGRAM_ID).unwrap();
            let blockhash = program.rpc().get_latest_blockhash().unwrap();

            let tx = Transaction::new_signed_with_payer(
                &instructions,
                Some(&program.payer()),
                &[&payer_clonned],
                blockhash,
            );

            let sig = program.rpc().send_and_confirm_transaction(&tx).unwrap();
            println!("✅ Signature: {}", sig);
        });
    }
}