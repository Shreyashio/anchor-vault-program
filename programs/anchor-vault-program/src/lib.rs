use anchor_lang::prelude::*;

declare_id!("4gSxRtognF6bHPA5H6cDyvWYvC6PZd3HHLSq1bqpdmta");

#[program]
pub mod anchor_vault_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
