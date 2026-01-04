use anchor_lang::prelude::*;

declare_id!("qAGHzQzfg1NqU4YLZDEUbquH4ZB4y6xz7zBHAtA6ibz");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn boaty_mc_boatfacee(ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
  signer: Signer<'info>,
  another_signer: Signer<'info>,
}
