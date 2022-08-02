use anchor_lang::prelude::*;

declare_id!("GwUMfoPtUC4EgTtKePwuVmfifpU5zFGmR6mPPVAtthKx");

pub mod instructions;
pub mod states;

pub use instructions::*;

#[program]
pub mod whitelisting {
    use super::*;

    pub fn initialize_authority(ctx: Context<InitAuthority>) -> Result<()> {
        instructions::init_authority::handler(ctx)
    }

    pub fn whitelist(ctx: Context<Whitelist>) -> Result<()> {
        instructions::whitelist::handler(ctx)
    }

}

