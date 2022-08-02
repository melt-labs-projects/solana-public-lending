use anchor_lang::prelude::*;

use crate::states::{AdminAccount, CollectionAccount, OracleAccount};

#[derive(Accounts)]
pub struct UpdateCollection<'info> {

    // Make sure the authority signed
    #[account(has_one = authority)]
    pub admin: Account<'info, AdminAccount>,

    pub authority: Signer<'info>,

    #[account(mut)]
    pub collection: Box<Account<'info, CollectionAccount>>,

    pub collection_oracle: Account<'info, OracleAccount>,

}

pub fn handler(ctx: Context<UpdateCollection>, max_collaterals: u64) -> Result<()> {
    let collection = &mut ctx.accounts.collection;
    collection.collection_oracle = ctx.accounts.collection_oracle.key();
    collection.max_collaterals = max_collaterals;
    Ok(())
}