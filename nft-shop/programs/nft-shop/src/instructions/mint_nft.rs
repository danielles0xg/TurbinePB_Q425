use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{
        mint_to,
        Mint,
        MintTo,
        Token,
        TokenAccount
    }
};
use anchor_spl::metadata::mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3Cpi,
        CreateMasterEditionV3CpiAccounts,
        CreateMasterEditionV3InstructionArgs,
        CreateMetadataAccountV3Cpi,
        CreateMetadataAccountV3CpiAccounts,
        CreateMetadataAccountV3InstructionArgs,
    },
    types::{
        Collection,
        Creator,
        DataV2,
    }
};

use crate::MintCount;


#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // Wallet account
    #[account(
        init,
        payer = owner,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>, // Mint account
    #[account(
        init,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub destination: Account<'info, TokenAccount>, // Associated token account
    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub master_edition: UncheckedAccount<'info>,
    #[account(
        seeds = [b"authority"],
        bump,
    )]
    /// CHECK: This account is used for signing purposes only, not initialized
    pub mint_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub collection_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        seeds = [b"mint_count"],
        bump,
    )]
    pub mint_count: Account<'info, MintCount>, // <-- infamous counter PDA
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
}

pub fn process_min_nft(ctx: Context<MintNft>) -> Result<()> {
    // Increment the mint counter
    ctx.accounts.mint_count.mint_count = ctx.accounts.mint_count.mint_count.checked_add(1)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let amount = ctx.accounts.mint_count.mint_count;
    msg!("Minting NFT #{}", amount);

    let metadata = &ctx.accounts.metadata.to_account_info();
    let master_edition = &ctx.accounts.master_edition.to_account_info();
    let mint = &ctx.accounts.mint.to_account_info();
    let authority = &ctx.accounts.mint_authority.to_account_info();
    let payer = &ctx.accounts.owner.to_account_info();
    let system_program = &ctx.accounts.system_program.to_account_info();
    let spl_token_program = &ctx.accounts.token_program.to_account_info();
    let spl_metadata_program = &ctx.accounts.token_metadata_program.to_account_info();

    let seeds = &[
        &b"authority"[..],
        &[ctx.bumps.mint_authority]
    ];
    let signer_seeds = &[&seeds[..]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    // by no it should be minted!

    let creator = vec![
        Creator {
            address: ctx.accounts.mint_authority.key(),
            verified: true,
            share: 100,
        },
    ];

    let metadata_account = CreateMetadataAccountV3Cpi::new(
        spl_metadata_program,
        CreateMetadataAccountV3CpiAccounts {
            metadata,
            mint,
            mint_authority: authority,
            payer,
            update_authority: (authority, true),
            system_program,
            rent: None,
        }, 
        CreateMetadataAccountV3InstructionArgs {
            data: DataV2 {
                name: "Turbin3".to_string(),
                symbol: format!("PBq425-{}", amount.to_string()),
                uri: "".to_string(),
                seller_fee_basis_points: 0,
                creators: Some(creator),
                collection: Some(Collection {
                    verified: false,
                    key: ctx.accounts.collection_mint.key(),
                }),
                uses: None
            },
            is_mutable: true,
            collection_details: None,
        }
    );
    metadata_account.invoke_signed(signer_seeds)?;

    let master_edition_account = CreateMasterEditionV3Cpi::new(
        spl_metadata_program,
        CreateMasterEditionV3CpiAccounts {
            edition: master_edition,
            update_authority: authority,
            mint_authority: authority,
            mint,
            payer,
            metadata,
            token_program: spl_token_program,
            system_program,
            rent: None,
        },
        CreateMasterEditionV3InstructionArgs {
            max_supply: Some(0),
        }
    );
    master_edition_account.invoke_signed(signer_seeds)?;
    

    let amount = ctx.accounts.mint_count.mint_count;
    msg!("Minting NFT #{}", amount);

    Ok(())
}