use anchor_lang::prelude::*;

declare_id!("4JM47H9t82cK6qtSdZLmnWwbTss6JPH7csJBZz5Qe68o");

pub mod instructions;
pub mod states;
pub mod errors;
pub mod events;

pub use instructions::*;
pub use states::*;

#[program]
pub mod lending {
    use super::*;

    pub fn initialize_authority(ctx: Context<InitAuthority>) -> Result<()> {
        instructions::admin::init_authority::handler(ctx)
    }

    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        instructions::admin::update_authority::handler(ctx, new_authority)
    }

    pub fn initialize_oracle(ctx: Context<InitOracle>, decimals: u64, price: u64) -> Result<()> {
        instructions::admin::init_oracle::handler(ctx, decimals, price)
    }

    pub fn update_oracle(ctx: Context<UpdateOracle>, price: u64) -> Result<()> {
        instructions::admin::update_oracle::handler(ctx, price)
    }

    pub fn initialize_collection(ctx: Context<InitCollection>, max_collaterals: u64) -> Result<()> {
        instructions::admin::init_collection::handler(ctx, max_collaterals)
    }

    pub fn update_collection(ctx: Context<UpdateCollection>, max_collaterals: u64) -> Result<()> {
        instructions::admin::update_collection::handler(ctx, max_collaterals)
    }

    pub fn initialize_n_slope_model(ctx: Context<InitNSlopeModel>, slopes: Vec<[u64; 3]>, _length: u64) -> Result<()> {
        instructions::admin::init_n_slope_model::handler(ctx, slopes)
    }

    pub fn initialize_lending_pool(ctx: Context<InitPool>, params: PoolParams) -> Result<()> {
        instructions::admin::init_pool::handler(ctx, params)
    }

    pub fn update_lending_pool(ctx: Context<UpdatePool>, params: PoolParams) -> Result<()> {
        instructions::admin::update_pool::handler(ctx, params)
    }

    pub fn initialize_borrowing_node(ctx: Context<InitNode>, params: NodeParams) -> Result<()> {
        instructions::admin::init_node::handler(ctx, params)
    }

    pub fn update_borrowing_node(ctx: Context<UpdateNode>, params: NodeParams) -> Result<()> {
        instructions::admin::update_node::handler(ctx, params)
    }

    pub fn initialize_slice(ctx: Context<InitSlice>, slice_number: u64) -> Result<()> {
        instructions::admin::init_slice::handler(ctx, slice_number)
    }

    // ==== Lending ====

    pub fn deposit_tokens(ctx: Context<DepositTokens>, amount: u64) -> Result<()> {
        instructions::lending::deposit_tokens::handler(ctx, amount)
    }

    pub fn withdraw_tokens(ctx: Context<WithdrawTokens>, amount: u64) -> Result<()> {
        instructions::lending::withdraw_tokens::handler(ctx, amount)
    }

    // ==== Borrowing ====

    pub fn initialize_borrow_account(ctx: Context<InitBorrowAccount>, max_collaterals: u64) -> Result<()> {
        instructions::borrowing::init_borrow_account::handler(ctx, max_collaterals)
    }

    pub fn close_borrow_account(ctx: Context<CloseBorrowAccount>) -> Result<()> {
        instructions::borrowing::close_borrow_account::handler(ctx)
    }

    pub fn deposit_collateral(ctx: Context<DepositCollateral>) -> Result<()> {
        instructions::borrowing::deposit_collateral::handler(ctx)
    }

    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>) -> Result<()> {
        instructions::borrowing::withdraw_collateral::handler(ctx)
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        instructions::borrowing::borrow::handler(ctx, amount)
    }

    pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
        instructions::borrowing::repay::handler(ctx, amount)
    }

    // ==== Liquidation =====

    pub fn liquidate(ctx: Context<Liquidate>) -> Result<()> {
        instructions::liquidation::liquidate::handler(ctx)
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        instructions::liquidation::purchase::handler(ctx)
    }

}

