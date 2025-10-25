use anchor_lang::prelude::*;

declare_id!("81HTrHxu1MsEpbG13tBgdfEkr1MZiiZQWXGViFCP25d8");

pub mod instructions;
use instructions::*;

#[program]
pub mod nft_shop {
    use super::*;

    pub fn init_shop(ctx: Context<MintCollection>, name: String, symbol: String, uri: String) -> Result<()> {
        process_init_shop(ctx, name, symbol, uri)
    }

    pub fn init_mint_count(ctx: Context<InitMintCount>) -> Result<()> {
        ctx.accounts.mint_count.mint_count = 0;
        Ok(())
    }

    pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {
        process_min_nft(ctx)
    }

}

#[derive(Accounts)]
pub struct InitMintCount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + MintCount::INIT_SPACE,
        seeds = [b"mint_count"],
        bump,
    )]
    pub mint_count: Account<'info, MintCount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct MintCount {
    pub mint_count: u64 // <- the infamous counter
}

