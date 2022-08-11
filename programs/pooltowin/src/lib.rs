use anchor_lang::prelude::*;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("FPpwkb2FsmhYnUXfbrxkE5HpwKZsbCcwExDUudrVdGdY");

#[derive(Debug)]
#[account]
pub struct Pool {
    token_mint: Pubkey,
    token_account: Pubkey,
    owner: Pubkey,
}

#[derive(Debug)]
#[account]
pub struct UserInfo {
    amount: u64,
}

pub fn init_token_mint_handler(mut ctx: Context<InitTokenMint>) -> Result<()> {
    let mut owner = &mut ctx.accounts.owner;
    let mut token_mint = &mut ctx.accounts.token_mint;

    Ok(())
}

pub fn init_user_info_handler(mut ctx: Context<InitUserInfo>) -> Result<()> {
    let mut owner = &mut ctx.accounts.owner;
    let mut user_info = &mut ctx.accounts.user_info;

    user_info.amount = 0;

    Ok(())
}

pub fn init_pool_handler(mut ctx: Context<InitPool>) -> Result<()> {
    let mut owner = &mut ctx.accounts.owner;
    let mut pool = &mut ctx.accounts.pool;
    let mut token_account = &mut ctx.accounts.token_account;
    let mut mint = &mut ctx.accounts.mint;

    pool.owner = owner.key();

    pool.token_mint = mint.key();

    pool.token_account = token_account.key();

    Ok(())
}

pub fn mint_token_handler(mut ctx: Context<MintToken>) -> Result<()> {
    let mut mint = &mut ctx.accounts.mint;
    let mut recipient = &mut ctx.accounts.recipient;
    let mut signer = &mut ctx.accounts.signer;

    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: mint.to_account_info(),
                authority: signer.to_account_info(),
                to: recipient.to_account_info(),
            },
        ),
        1000,
    )?;

    Ok(())
}

pub fn deposit_pool_handler(mut ctx: Context<DepositPool>, mut amount: u64) -> Result<()> {
    let mut signer = &mut ctx.accounts.signer;
    let mut user_token_account = &mut ctx.accounts.user_token_account;
    let mut pool_token_account = &mut ctx.accounts.pool_token_account;
    let mut user_info = &mut ctx.accounts.user_info;

    require!(amount > (0 as u64), ProgramError::E000);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: user_token_account.to_account_info(),
                authority: signer.to_account_info(),
                to: pool_token_account.to_account_info(),
            },
        ),
        amount,
    )?;

    user_info.amount += amount;

    Ok(())
}

pub fn withdraw_pool_handler(mut ctx: Context<WithdrawPool>, mut amount: u64) -> Result<()> {
    let mut signer = &mut ctx.accounts.signer;
    let mut user_token_account = &mut ctx.accounts.user_token_account;
    let mut pool_token_account = &mut ctx.accounts.pool_token_account;
    let mut user_info = &mut ctx.accounts.user_info;

    require!(amount > (0 as u64), ProgramError::E001);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: pool_token_account.to_account_info(),
                authority: signer.to_account_info(),
                to: user_token_account.to_account_info(),
            },
        ),
        amount,
    )?;

    user_info.amount -= amount;

    Ok(())
}

#[derive(Accounts)]
pub struct InitTokenMint<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = ["token-mint".as_bytes().as_ref(), owner.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = owner
    )]
    pub token_mint: Box<Account<'info, token::Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitUserInfo<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = ["user-info".as_bytes().as_ref(), owner.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<UserInfo>()
    )]
    pub user_info: Box<Account<'info, UserInfo>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = ["pool-account".as_bytes().as_ref(), owner.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Pool>()
    )]
    pub pool: Box<Account<'info, Pool>>,
    #[account(
        init,
        payer = owner,
        seeds = ["token-account".as_bytes().as_ref(), owner.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = owner
    )]
    pub token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub mint: Box<Account<'info, token::Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub recipient: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct DepositPool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub pool_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub user_info: Box<Account<'info, UserInfo>>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct WithdrawPool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub pool_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub user_info: Box<Account<'info, UserInfo>>,
    pub token_program: Program<'info, token::Token>,
}

#[program]
pub mod pooltowin {
    use super::*;

    pub fn init_token_mint(ctx: Context<InitTokenMint>) -> Result<()> {
        init_token_mint_handler(ctx)
    }

    pub fn init_user_info(ctx: Context<InitUserInfo>) -> Result<()> {
        init_user_info_handler(ctx)
    }

    pub fn init_pool(ctx: Context<InitPool>) -> Result<()> {
        init_pool_handler(ctx)
    }

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        mint_token_handler(ctx)
    }

    pub fn deposit_pool(ctx: Context<DepositPool>, amount: u64) -> Result<()> {
        deposit_pool_handler(ctx, amount)
    }

    pub fn withdraw_pool(ctx: Context<WithdrawPool>, amount: u64) -> Result<()> {
        withdraw_pool_handler(ctx, amount)
    }
}

#[error_code]
pub enum ProgramError {
    #[msg("Deposit amount should be more than 0")]
    E000,
    #[msg("Withdraw amount should be more than 0")]
    E001,
}
