use anchor_lang::prelude::*;

declare_id!("3iaCQt3p4JrS3SCNqwRwX8qirBYCdneczPDZYYsoCN7e");

#[program]
pub mod poi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
