use anchor_lang::prelude::*;

use crate::states::{AdminAccount, CollectionAccount, LSlice};

#[derive(Accounts)]
#[instruction(slice_number: u64)]
pub struct InitSlice<'info> {

    // Make sure the authority signed
    #[account(has_one = authority)]
    pub admin: Account<'info, AdminAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub collection: Account<'info, CollectionAccount>,

    #[account(
        init,
        space = LSlice::SIZE,
        payer = authority,
        seeds = [LSlice::NAME, collection.key().as_ref(), slice_number.to_le_bytes().as_ref()],
        bump
    )]
    pub slice: Account<'info, LSlice>,

    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<InitSlice>, slice_number: u64) -> Result<()> {
    ctx.accounts.slice.bump = *ctx.bumps.get("slice").unwrap();
    ctx.accounts.slice.number = slice_number;
    Ok(())
}