use anchor_lang::prelude::*;

declare_id!("2zu8SFickvWcfMWLVGAWi8nmXbCYpJ53rfcqpN2sk2Ci");

#[program]
pub mod goblin_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
