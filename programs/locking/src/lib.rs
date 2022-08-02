use anchor_lang::prelude::*;

declare_id!("5DkgLNzMMMdxoEUrS2ZwVWxcsW7E5LAsCLsp25WX6jyE");

pub mod instructions;
pub mod states;
pub mod errors;
pub mod events;

use instructions::*;
use states::*;

#[program]
pub mod locking {
    use super::*;

    pub fn initialize_authority(ctx: Context<InitAuthority>) -> Result<()> {
        instructions::init_authority::handler(ctx)
    }

    pub fn update_authority(ctx: Context<UpdateAuthority>, authority: Pubkey) -> Result<()> {
        instructions::update_authority::handler(ctx, authority)
    }

    pub fn initialize_membership(ctx: Context<InitMembership>, membership_number: u64, perks: LiquidationPerks) -> Result<()> {
        instructions::init_membership::handler(ctx, membership_number, perks)
    }

    pub fn update_membership(ctx: Context<UpdateMembership>, perks: LiquidationPerks) -> Result<()> {
        instructions::update_membership::handler(ctx, perks)
    }

    pub fn initialize_cube(ctx: Context<InitCube>) -> Result<()> {
        instructions::init_cube::handler(ctx)
    }

    // ==== LOCKING ====

    pub fn initialize_locking_account(ctx: Context<InitLockAccount>) -> Result<()> {
        instructions::locking::init_locking_account::handler(ctx)
    }

    pub fn lock(ctx: Context<Lock>) -> Result<()> {
        instructions::locking::lock::handler(ctx)
    }

    pub fn unlock(ctx: Context<Unlock>) -> Result<()> {
        instructions::locking::unlock::handler(ctx)
    }

    pub fn close_locking_account(ctx: Context<CloseLockAccount>) -> Result<()> {
        instructions::locking::close_locking_account::handler(ctx)
    }

     // ==== LIQUIDATIONS ====

     pub fn liquidation_update(ctx: Context<LiquidationUpdate>) -> Result<()> {
        instructions::liquidation::update::handler(ctx)
    }

    pub fn liquidation_log(ctx: Context<LiquidationLog>) -> Result<()> {
        instructions::liquidation::log::handler(ctx)
    }

}