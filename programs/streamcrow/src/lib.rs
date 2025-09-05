use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("ErZEgit4NAfSQTAoDeH2KrRR3AvtMAEt1nTXRdDA9KA6");

#[program]
pub mod streamcrow {
    use super::*;

    pub fn create_escrow(
        ctx: Context<CreateEscrow>,
        total_amount: u64,
        milestones: Vec<u64>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn submit_milestone(ctx: Context) -> Result<()> {
        Ok(())
    }

    pub fn submit_milestone(ctx: Context<ApproveMilestone>) -> Result<()> {
        Ok(())
    }

    pub fn dispute_milestone(ctx: Context<DisputeMilestone>) -> Result<()> {
        Ok(())
    }

    pub fn resolve_dispute(
        ctx: Context<ResolveDispute>,
        client_share: u64,
        provider_share: u64,
    ) -> Result<()> {
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub client: Pubkey,             // client wallet
    pub provider: Pubkey,           // provider wallet
    pub arbiter: Pubkey,            // arbiter wallet
    pub token_mint: Pubkey,         // token being used
    pub vault_account: Pubkey,      // PDA holding funds
    pub total_amount: u64,          // total escrow amount
    pub milestones: Vec<Milestone>, // milestones list
    pub current_index: u8,          // current milestone index
    pub is_active: u8,              // active or closed
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Milestone {
    pub amount: u64,        // total milestone amount locked
    pub is_submitted: bool, // provider submitted work
    pub is_approved: bool,  // client approved the work
    pub is_released: bool,  // amount ? released or !
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {}

#[derive(Accounts)]
pub struct SubmitMilestone<'info> {}

#[derive(Accounts)]
pub struct ApproveMilestone<'info> {}

#[derive(Accounts)]
pub struct DisputeMilestone<'info> {}

#[derive(Accounts)]
pub struct ResolveDispute<'info> {}
