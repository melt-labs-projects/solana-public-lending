use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};


use crate::states::{LockingAccount, CubeAccount, MembershipAccount};
use crate::instructions::utils;
use crate::events;

#[derive(Accounts)]
pub struct Unlock<'info> {

    #[account(
        mut,
        seeds = [LockingAccount::NAME, locker.key().as_ref()],
        bump = locking_account.bump,
        has_one = locked_token_account,
    )]
    pub locking_account: Account<'info, LockingAccount>,

    #[account(
        mut,
        seeds = [CubeAccount::NAME, locking_account.cube_mint.as_ref()],
        bump = cube.bump,
        has_one = membership,
        has_one = locker
    )]
    pub cube: Account<'info, CubeAccount>,

    #[account(mut)]
    pub membership: Account<'info, MembershipAccount>,

    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub locked_token_account: Account<'info, TokenAccount>,

    pub locker: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

impl<'info> Unlock<'info> {

    fn transfer_cube(&self) -> Result<()> {
        utils::transfer_signed(
            self.token_program.to_account_info(),
            self.locked_token_account.to_account_info(),
            self.to_token_account.to_account_info(),
            self.locking_account.to_account_info(),
            &self.locking_account,
            &self.locker.key()
        )
    }

}
    
pub fn handler(ctx: Context<Unlock>) -> Result<()> {

    ctx.accounts.transfer_cube()?;

    let cube = &mut ctx.accounts.cube;
    cube.locker = Pubkey::default();

    let membership = &mut ctx.accounts.membership;
    membership.locked_count -= 1;

    let locking = &mut ctx.accounts.locking_account;
    locking.reset();

    events::emit_unlocked_event(ctx.accounts.cube.mint)?;

    Ok(())
}
