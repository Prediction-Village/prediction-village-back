use std::error::Error;

use programs_executor::ProgramsExecutor;
mod programs_executor;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let programs_executor = ProgramsExecutor::new()?;
    programs_executor.init_game()?;

    loop {
        programs_executor.generate_income()?;
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
