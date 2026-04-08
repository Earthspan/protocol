use anchor_lang::prelude::*;

declare_id!("BeBdUnX84ibxLAdhsz9snDTnmm54CBViBpcHtE4PeqHY");

#[program]
pub mod upra_protocol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
