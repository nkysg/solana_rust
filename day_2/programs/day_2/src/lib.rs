use anchor_lang::prelude::*;

declare_id!("GNi4qBFawVQe3VkcRqNxSf59Sbx8yLKY9gHWXbf6iCyh");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
	msg!("You said {:?}", message);
	msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
	msg!("You array {:?}",arr);
	Ok(())
	}
}

#[derive(Accounts)]
pub struct Initialize {}
