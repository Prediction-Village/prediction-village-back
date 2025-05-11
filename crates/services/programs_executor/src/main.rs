use std::error::Error;

use game_state::{fetch_game_state, GameStatus};
use programs_executor::ProgramsExecutor;
mod programs_executor;
mod variables;
mod game_state;
mod utils;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    variables::check_env();

    let programs_executor = ProgramsExecutor::new().await?;

    loop {
        let game_state = fetch_game_state().await?;
        println!("Game state: {:?}", game_state);
        match game_state.status {
            GameStatus::InProgress => {
                println!("Generate income");
                programs_executor.generate_income()?;
            }
            GameStatus::Finished(_) => {
                println!("Restart game");
                programs_executor.restart_game()?;
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
