use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::states::{Whitelisted, AdminAccount};


#[derive(Accounts)]
pub struct Whitelist<'info> {

    #[account(has_one = authority)]
    pub admin: Account<'info, AdminAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub mint: Account<'info, Mint>,

    /// CHECK: it's fine
    pub collection: UncheckedAccount<'info>,

    #[account(
        init,
        space = Whitelisted::SIZE,
        payer = authority,
        seeds = [Whitelisted::NAME, collection.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub whitelisted: Account<'info, Whitelisted>,

    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<Whitelist>) -> Result<()> {
    ctx.accounts.whitelisted.mint = ctx.accounts.mint.key();
    ctx.accounts.whitelisted.collection = ctx.accounts.collection.key();
    ctx.accounts.whitelisted.bump = *ctx.bumps.get("whitelisted").unwrap();
    Ok(())
}