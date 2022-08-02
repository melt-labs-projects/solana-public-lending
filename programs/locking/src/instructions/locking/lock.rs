use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};


use crate::states::{LockingAccount, CubeAccount, MembershipAccount};
use crate::instructions::utils;
use crate::events;

#[derive(Accounts)]
pub struct Lock<'info> {

    #[account(
        mut,
        seeds = [LockingAccount::NAME, locker.key().as_ref()],
        bump = locking_account.bump,
    )]
    pub locking_account: Account<'info, LockingAccount>,

    #[account(
        mut,
        seeds = [CubeAccount::NAME, cube_mint.key().as_ref()],
        bump = cube.bump,
        has_one = membership
    )]
    pub cube: Account<'info, CubeAccount>,

    #[account(mut)]
    pub membership: Account<'info, MembershipAccount>,

    pub cube_mint: Account<'info, Mint>,

    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = locker,
        token::mint = cube_mint,
        token::authority = locking_account, 
    )]
    pub locked_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub locker: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Lock<'info> {

    fn transfer_cube(&self) -> Result<()> {
        utils::transfer(
            self.token_program.to_account_info(),
            self.from_token_account.to_account_info(),
            self.locked_token_account.to_account_info(),
            self.locker.to_account_info(),
        )
    }

}

pub fn handler(ctx: Context<Lock>) -> Result<()> {

    let locking_account = &mut ctx.accounts.locking_account;
    locking_account.locked_token_account = ctx.accounts.locked_token_account.key();
    locking_account.locked_timestamp = utils::current_timestamp()?;
    locking_account.cube_mint = ctx.accounts.cube_mint.key();
    locking_account.membership = ctx.accounts.membership.key();

    let cube = &mut ctx.accounts.cube;
    cube.locker = ctx.accounts.locker.key();

    let membership = &mut ctx.accounts.membership;
    membership.locked_count += 1;

    ctx.accounts.transfer_cube()?;

    events::emit_locked_event(ctx.accounts.cube.mint)?;

    Ok(())
}
