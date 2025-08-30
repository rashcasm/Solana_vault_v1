use anchor_lang::prelude::*;

declare_id!("GkDgiDHW6bqNBJp8q3GvkSzmsHgEMzcEZK4AWRVy16Zj");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
