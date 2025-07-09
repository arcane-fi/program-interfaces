#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("FarmsPZpWu9i7Kky8tPN37rs2TpmMrAZrC7S7vJa91Hr");

/// CHECK: no checks are needed because this is just a cpi interface
#[allow(unexpected_cfgs)]
#[program]
pub mod kfarms_interface {
    use super::*;

    pub fn initialize_user(_ctx: Context<InitializeUser>) -> Result<()> {
        Ok(())
    }

    pub fn stake(_ctx: Context<Stake>, _amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn unstake(_ctx: Context<Unstake>, _staked_shares_scaled: u128) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_unstaked_deposits(_ctx: Context<WithdrawUnstakedDeposits>) -> Result<()> {
        Ok(())
    }

    pub fn harvest_reward(_ctx: Context<HarvestRewards>, _reward_index: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub owner: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub delegatee: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub user_state: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub farm_state: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    pub owner: Signer<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub user_state: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub farm_state: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub farm_vault: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub user_ata: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub token_mint: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub scope_prices: Option<AccountInfo<'info>>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub user_state: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub farm_state: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub scope_prices: Option<AccountInfo<'info>>,
}

#[derive(Accounts)]
pub struct WithdrawUnstakedDeposits<'info> {
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub user_state: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub farm_state: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub user_ata: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub farm_vault: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub farm_vaults_authority: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct HarvestRewards<'info> {
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub user_state: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub farm_state: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub global_config: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub reward_mint: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub user_reward_ata: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub rewards_vault: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    #[account(mut)]
    pub rewards_treasury_vault: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub farm_vaults_authority: AccountInfo<'info>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub scope_prices: Option<AccountInfo<'info>>,
    /// CHECK: no checks are needed because this is just a cpi interface
    pub token_program: AccountInfo<'info>,
}
