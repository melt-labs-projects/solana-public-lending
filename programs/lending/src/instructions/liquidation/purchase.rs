use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};
use locking::{
    program::Locking,
    states::{MembershipAccount, LockingAccount}
};

use crate::events;
use crate::instructions::{utils, logic};
use crate::errors::LendingError;
use crate::states::{
    AdminAccount, 
    LendingPool,
    BorrowingNode,
    CollateralAccount,
    OracleAccount,
    CollectionAccount,
    LSlice,
    CollateralList,
};

#[derive(Accounts)]
pub struct Purchase<'info> {

    pub admin: Account<'info, AdminAccount>,

    #[account(
        mut,
        has_one = treasury,
        has_one = reserve,
        has_one = base_oracle,
    )]
    pub pool: Box<Account<'info, LendingPool>>,

    #[account(mut)]
    pub treasury: Box<Account<'info, TokenAccount>>,
    
    #[account(mut)]
    pub reserve: Box<Account<'info, TokenAccount>>,

    pub base_oracle: Account<'info, OracleAccount>,

    #[account(
        mut,
        seeds = [BorrowingNode::NAME, pool.key().as_ref(), node.collection.as_ref()],
        bump = node.bump,
        has_one = collection
    )]
    pub node: Account<'info, BorrowingNode>,

    #[account(mut, has_one = collection_oracle)]
    pub collection: Account<'info, CollectionAccount>,

    pub collection_oracle: Account<'info, OracleAccount>,

    #[account(
        mut, 
        seeds = [CollateralAccount::NAME, node.collection.as_ref(), collateral_account.mint.as_ref()],
        bump = collateral_account.bump,
        has_one = collateral_token_account,
        close = receiver
    )]
    pub collateral_account: Box<Account<'info, CollateralAccount>>,

    #[account(mut)]
    pub collateral_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub collaterals: Box<Account<'info, CollateralList>>,
    
    pub signer: Signer<'info>,

    #[account(mut)]
    pub receiver: SystemAccount<'info>,

    #[account(mut)]
    pub payment_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub to_token_account: Box<Account<'info, TokenAccount>>,

    pub locking_program: Program<'info, Locking>,

    #[account(mut)]
    pub locking_account: Account<'info, LockingAccount>,

    pub membership_account: Account<'info, MembershipAccount>,

    pub token_program: Program<'info, Token>,

    // ==== Slices =====

    #[account(
        mut,
        seeds = [LSlice::NAME, node.collection.as_ref(), a_slice.number.to_le_bytes().as_ref()],
        bump = a_slice.bump
    )]
    pub a_slice: Box<Account<'info, LSlice>>,

    #[account(
        mut,
        seeds = [LSlice::NAME, node.collection.as_ref(), last_slice.number.to_le_bytes().as_ref()],
        bump = last_slice.bump
    )]
    pub last_slice: Box<Account<'info, LSlice>>,

}

impl<'info> Purchase<'info> {

    pub fn update_locking(&self) -> Result<()> {
        utils::update_locking(
            self.locking_program.to_account_info(),
            self.locking_account.to_account_info(),
            self.membership_account.to_account_info(),
        )
    }

    pub fn log_liquidation(&self) -> Result<()> {
        utils::log_liquidation(
            self.locking_program.to_account_info(),
            self.locking_account.to_account_info(),
            self.signer.to_account_info(),
        )
    }

    pub fn transfer_to_treasury(&self, amount: u64) -> Result<()> {
        utils::transfer(
            self.token_program.to_account_info(), 
            self.payment_token_account.to_account_info(), 
            self.treasury.to_account_info(), 
            self.signer.to_account_info(), 
            amount
        )
    }

    pub fn transfer_to_reserve(&self, amount: u64) -> Result<()> {
        utils::transfer(
            self.token_program.to_account_info(), 
            self.payment_token_account.to_account_info(), 
            self.reserve.to_account_info(), 
            self.signer.to_account_info(), 
            amount
        )
    }

    pub fn transfer_from_reserve_to_treasury(&self, amount: u64) -> Result<()> {
        utils::transfer_signed(
            self.token_program.to_account_info(), 
            self.reserve.to_account_info(), 
            self.treasury.to_account_info(), 
            self.admin.to_account_info(), 
            &self.admin,
            amount
        )
    }

    pub fn transfer_collateral(&self) -> Result<()> {
        utils::transfer_signed(
            self.token_program.to_account_info(), 
            self.collateral_token_account.to_account_info(), 
            self.to_token_account.to_account_info(), 
            self.admin.to_account_info(), 
            &self.admin,
            1
        )
    }

    pub fn close_collateral_token_account(&self) -> Result<()> {
        utils::close_token_account(
            self.token_program.to_account_info(), 
            self.collateral_token_account.to_account_info(), 
            self.receiver.to_account_info(), 
            self.admin.to_account_info(), 
            &self.admin,
        )
    }

}

pub fn handler(ctx: Context<Purchase>) -> Result<()> {

    // POTENTIAL BUG - need to make sure the pool + node is associated
    // with collaterals which are being liquidated. NO? this is enforced by the
    // slices. The slices are associated with the node and the node is associated with
    // the pool. If the collaterals is not in the slice, then it will fail.

    ctx.accounts.update_locking()?;
    ctx.accounts.locking_account.reload()?;

    let collaterals = &ctx.accounts.collaterals;

    // Make sure these collaterals have been liquidated
    require!(collaterals.is_pending_liquidation, LendingError::NotPurchasable);

    let (amount_for_treasury, amount_for_reserve) = logic::purchase::execute(
        &mut ctx.accounts.pool,
        &mut ctx.accounts.node,
        &mut ctx.accounts.collateral_account,
        &mut ctx.accounts.collaterals,
        &ctx.accounts.base_oracle,
        &ctx.accounts.collection_oracle,
        &ctx.accounts.locking_account
    )?;

    let collaterals_key = ctx.accounts.collaterals.key();
    logic::slice::remove(
        &mut ctx.accounts.collaterals,
        collaterals_key,
        &mut ctx.accounts.collection,
        &ctx.accounts.collateral_account,
        &mut ctx.accounts.a_slice,
        &mut ctx.accounts.last_slice,
    )?;

    ctx.accounts.transfer_to_treasury(amount_for_treasury)?;
    ctx.accounts.transfer_to_reserve(amount_for_reserve)?;

    // let amount_to_recover = logic::purchase::recover_tokens(&mut ctx.accounts.pool)?;
    // ctx.accounts.transfer_from_reserve_to_treasury(amount_to_recover)?;
    
    // Transfer collateral and close token account
    ctx.accounts.transfer_collateral()?;
    ctx.accounts.close_collateral_token_account()?;

    // Log the liquidation
    ctx.accounts.log_liquidation()?;

    events::emit_purchase_event(
        ctx.accounts.pool.key(),
        ctx.accounts.node.collection,
        ctx.accounts.signer.key(),
        ctx.accounts.collateral_account.mint,
        amount_for_treasury + amount_for_reserve
    )?;

    Ok(())
}