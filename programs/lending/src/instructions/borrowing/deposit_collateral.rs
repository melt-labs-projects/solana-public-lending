use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};
use whitelisting::states::Whitelisted;

use crate::instructions::{utils, logic};
use crate::states::{
    AdminAccount, 
    BorrowingNode, 
    CollectionAccount, 
    CollateralAccount, 
    BorrowAccount,
    CollateralList,
};

#[derive(Accounts)]
pub struct DepositCollateral<'info> {

    pub admin: Account<'info, AdminAccount>,

    // Make sure the one collection is associated with this borrowing node.
    #[account(has_one = collection)]
    pub node: Box<Account<'info, BorrowingNode>>,

    #[account(mut)]
    pub collection: Box<Account<'info, CollectionAccount>>,

    #[account(mut)]
    pub signer: Signer<'info>,

    // We check the seeds here to make sure this borrow account is for this
    // particular borrowing node and does actually belong to the signer.
    //
    // Also make sure the collaterals account is the one associated with 
    // this borrow account.
    #[account(
        mut,
        seeds = [BorrowAccount::NAME, node.key().as_ref(), signer.key().as_ref()],
        bump = borrow_account.bump,
        has_one = collaterals
    )]
    pub borrow_account: Box<Account<'info, BorrowAccount>>,

    #[account(mut)]
    pub collaterals: Box<Account<'info, CollateralList>>,

    pub collateral_mint: Box<Account<'info, Mint>>,

    #[account(
        has_one = collection,
        constraint = whitelisted.mint == collateral_mint.key()
    )]
    pub whitelisted: Account<'info, Whitelisted>,

    // We make sure this collateral account is the one associated with the right collection
    // and token mint by checking the seeds.
    #[account(
        init,
        space = CollateralAccount::SIZE,
        payer = signer,
        seeds = [CollateralAccount::NAME, collection.key().as_ref(), collateral_mint.key().as_ref()],
        bump
    )]
    pub collateral_account: Account<'info, CollateralAccount>,

    // Create a token account which will be used to hold the collateral.
    #[account(
        init,
        payer = signer,
        token::mint = collateral_mint,
        token::authority = admin, 
    )]
    pub collateral_token_account: Account<'info, TokenAccount>,

    // The token account containing the NFT owned by the borrower.
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,

    pub token_program: Program<'info, Token>,

}

impl<'info> DepositCollateral<'info> {

    pub fn transfer_collateral(&self) -> Result<()> {
        utils::transfer(
            self.token_program.to_account_info(), 
            self.user_token_account.to_account_info(), 
            self.collateral_token_account.to_account_info(), 
            self.signer.to_account_info(), 
            1,
        )
    }

}

pub fn handler(ctx: Context<DepositCollateral>) -> Result<()> {

    // Initialise the collateral account fields
    ctx.accounts.collateral_account.bump = *ctx.bumps.get("collateral_account").unwrap();
    ctx.accounts.collateral_account.mint = ctx.accounts.collateral_mint.key();

    // Transfer collateral from borrower
    ctx.accounts.transfer_collateral()?;

    let collaterals_key = ctx.accounts.collaterals.key();
    logic::deposit_collateral::execute(
        &ctx.accounts.node,
        &mut ctx.accounts.collaterals,
        &mut ctx.accounts.collection,
        &mut ctx.accounts.collateral_account,
        &ctx.accounts.collateral_token_account.key(),
        collaterals_key,
        &ctx.accounts.collateral_token_account.mint,
        &ctx.accounts.signer.key()
    )?;

    Ok(())
}