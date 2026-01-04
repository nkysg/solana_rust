use anchor_lang::prelude::*;

declare_id!("DMDsyETBnTcSJC2qawnsR2qDv9FCqzBRYxR2ShkVcYbb");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
