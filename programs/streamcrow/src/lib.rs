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
pub struct CreateEscrow<'info> {
    #[account(mut)]
    pub client: Signer<'info>,

    #[account(
        init,
        payer = client,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", client.key().as_ref(), provider.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = client,
        token::mint = token_mint,
        token::authority = escrow,
        seeds = [b"vault", escrow.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,
    pub provider: Account<'info, Token>,
    pub token_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = token_mint,
        token::authority = client
    )]
    pub client_token_account: Account<'info, TokenAccount>,
    pub token_program: Account<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SubmitMilestone<'info> {}

#[derive(Accounts)]
pub struct ApproveMilestone<'info> {}

#[derive(Accounts)]
pub struct DisputeMilestone<'info> {}

#[derive(Accounts)]
pub struct ResolveDispute<'info> {}

#[error_code]
pub enum StreamCrowError {
    #[msg("Invalid milestone count (must be 1-20)")]
    InvalidMilestoneCount,
    
    #[msg("Milestone data arrays don't match milestone count")]
    MismatchedMilestoneData,
    
    #[msg("Total milestone amounts don't match escrow amount")]
    AmountMismatch,
    
    #[msg("Escrow is not in active status")]
    EscrowNotActive,
    
    #[msg("Invalid milestone ID")]
    InvalidMilestoneId,
    
    #[msg("Milestones must be completed in order")]
    MilestonesOutOfOrder,
    
    #[msg("Milestone has not been submitted for approval")]
    MilestoneNotSubmitted,
    
    #[msg("Milestone is not in a state that can be disputed")]
    MilestoneNotDisputable,
    
    #[msg("Escrow is not in disputed status")]
    EscrowNotDisputed,
    
    #[msg("Milestone is not currently disputed")]
    MilestoneNotDisputed,
    
    #[msg("Dispute reason is too long (max 500 characters)")]
    DisputeReasonTooLong,
    
    #[msg("Invalid refund percentage (must be 0-100)")]
    InvalidRefundPercentage,
    
    #[msg("Emergency timeout period has not been reached (90 days)")]
    EmergencyTimeoutNotReached,
}