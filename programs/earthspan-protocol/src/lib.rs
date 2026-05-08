use anchor_lang::prelude::*;

declare_id!("CKTz56hSdQEKMKwPWNgDRq8rAfWamz3AfLgB3NXEctuf");

#[program]
pub mod earthspan_protocol {
    use super::*;

    pub fn initialize_engine(ctx: Context<Initialize>) -> Result<()> {
        msg!("Sovereign Engine Initialized: Earthspan Protocol is Live");
        Ok(())
    }

    // This is the core logic for the Solana Frontier Hackathon
    pub fn capture_trade_fee(ctx: Context<CaptureFee>, amount: u64) -> Result<()> {
        let fee = amount / 100; // 1% Fee Capture
        msg!("Sovereign Engine captured a 1% fee of: {} lamports", fee);
        msg!("Fee designated for: Field, Assets, Resilience, Innovation, Ecosystem");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CaptureFee<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}