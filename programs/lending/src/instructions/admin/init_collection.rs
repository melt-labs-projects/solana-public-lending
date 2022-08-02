use anchor_lang::prelude::*;

use crate::states::{AdminAccount, CollectionAccount, OracleAccount};

#[derive(Accounts)]
pub struct InitCollection<'info> {

    // Make sure the current authority signed
    #[account(has_one = authority)]
    pub admin: Account<'info, AdminAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        space = CollectionAccount::SIZE,
        payer = authority,
    )]
    pub collection: Account<'info, CollectionAccount>,

    pub collection_oracle: Account<'info, OracleAccount>,

    pub system_program: Program<'info, System>,

}

#[allow(unused_variables)]
pub fn handler(ctx: Context<InitCollection>, max_collaterals: u64) -> Result<()> {
    let collection = &mut ctx.accounts.collection;
    collection.collection_oracle = ctx.accounts.collection_oracle.key();
    collection.max_collaterals = max_collaterals;
    Ok(())
}