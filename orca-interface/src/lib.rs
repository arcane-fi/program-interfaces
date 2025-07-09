#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

#[program]
pub mod orca_interface {
    use super::*;

    #[allow(unused_variables)]
    pub fn swap_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapV2<'info>>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn increase_liquidity_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, ModifyLiquidityV2<'info>>,
        liquidity_amount: u128,
        token_max_a: u64,
        token_max_b: u64,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn decrease_liquidity_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, ModifyLiquidityV2<'info>>,
        liquidity_amount: u128,
        token_min_a: u64,
        token_min_b: u64,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn collect_fees_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, CollectFeesV2<'info>>,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn update_fees_and_rewards(ctx: Context<UpdateFeesAndRewards>) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn open_position_with_token_extensions(
        ctx: Context<OpenPositionWithTokenExtensions>,
        tick_lower_index: i32,
        tick_upper_index: i32,
        with_token_metadata: bool,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn close_position_with_token_extensions(
        ctx: Context<ClosePositionWithTokenExtensions>,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SwapV2<'info> {
    /// CHECK: validated in cpi
    pub token_program_a: AccountInfo<'info>,
    /// CHECK: validated in cpi
    pub token_program_b: AccountInfo<'info>,
    /// CHECK: validated in cpi
    pub memo_program: AccountInfo<'info>,
    /// CHECK: validated in cpi
    pub token_authority: AccountInfo<'info>,
    /// CHECK: validated in cpi
    #[account(mut)]
    pub whirlpool: AccountInfo<'info>,

    /// CHECK: validated in cpi
    pub token_mint_a: AccountInfo<'info>,
    /// CHECK: validated in cpi
    pub token_mint_b: AccountInfo<'info>,
    /// CHECK: validated in cpi
    #[account(mut)]
    pub token_owner_account_a: AccountInfo<'info>,
    /// CHECK: validated in cpi
    #[account(mut)]
    pub token_vault_a: AccountInfo<'info>,

    /// CHECK: validated in cpi
    #[account(mut)]
    pub token_owner_account_b: AccountInfo<'info>,
    /// CHECK: validated in cpi
    #[account(mut)]
    pub token_vault_b: AccountInfo<'info>,
    /// CHECK: validated in cpi
    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_0: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_1: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_2: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Oracle is currently unused and will be enabled on subsequent updates
    pub oracle: UncheckedAccount<'info>,
    // remaining accounts
    // - accounts for transfer hook program of token_mint_a
    // - accounts for transfer hook program of token_mint_b
    // - supplemental TickArray accounts
}

#[derive(Accounts)]
pub struct ModifyLiquidityV2<'info> {
    #[account(mut)]
    /// CHECK:
    pub whirlpool: UncheckedAccount<'info>,

    /// CHECK:
    pub token_program_a: UncheckedAccount<'info>,

    /// CHECK:
    pub token_program_b: UncheckedAccount<'info>,

    /// CHECK:
    pub memo_program: UncheckedAccount<'info>,

    /// CHECK:
    pub position_authority: Signer<'info>,

    #[account(mut)]
    /// CHECK:
    pub position: UncheckedAccount<'info>,

    /// CHECK:
    pub position_token_account: UncheckedAccount<'info>,

    /// CHECK:
    pub token_mint_a: UncheckedAccount<'info>,

    /// CHECK:
    pub token_mint_b: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub token_owner_account_a: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub token_owner_account_b: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub token_vault_a: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub token_vault_b: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub tick_array_lower: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub tick_array_upper: UncheckedAccount<'info>,
    // remaining_accounts
    // - accounts for transfer hook program of token_mint_a
    // - accounts for transfer hook program of token_mint_b
}

#[derive(Accounts)]
pub struct CollectFeesV2<'info> {
    /// CHECK:
    pub whirlpool: UncheckedAccount<'info>,

    pub position_authority: Signer<'info>,

    #[account(mut)]
    /// CHECK:
    pub position: UncheckedAccount<'info>,
    /// CHECK:
    pub position_token_account: UncheckedAccount<'info>,

    /// CHECK:
    pub token_mint_a: UncheckedAccount<'info>,
    /// CHECK:
    pub token_mint_b: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub token_owner_account_a: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK:
    pub token_vault_a: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub token_owner_account_b: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK:
    pub token_vault_b: UncheckedAccount<'info>,

    /// CHECK:
    pub token_program_a: UncheckedAccount<'info>,

    /// CHECK:
    pub token_program_b: UncheckedAccount<'info>,
    /// CHECK:
    pub memo_program: UncheckedAccount<'info>,
    // remaining accounts
    // - accounts for transfer hook program of token_mint_a
    // - accounts for transfer hook program of token_mint_b
}

#[derive(Accounts)]
pub struct UpdateFeesAndRewards<'info> {
    #[account(mut)]
    /// CHECK:
    pub whirlpool: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub position: UncheckedAccount<'info>,

    /// CHECK:
    pub tick_array_lower: UncheckedAccount<'info>,

    /// CHECK:
    pub tick_array_upper: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct OpenPositionWithTokenExtensions<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    /// CHECK:
    pub owner: UncheckedAccount<'info>,

    /// CHECK:
    pub position: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub position_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub position_token_account: UncheckedAccount<'info>,

    /// CHECK:
    pub whirlpool: UncheckedAccount<'info>,

    /// CHECK:
    pub token_2022_program: UncheckedAccount<'info>,

    /// CHECK:
    pub system_program: UncheckedAccount<'info>,

    /// CHECK:
    pub associated_token_program: UncheckedAccount<'info>,

    /// CHECK:
    pub metadata_update_auth: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct ClosePositionWithTokenExtensions<'info> {
    pub position_authority: Signer<'info>,

    /// CHECK
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub position: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub position_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub position_token_account: UncheckedAccount<'info>,

    /// CHECK:
    pub token_2022_program: UncheckedAccount<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AccountsType {
    TransferHookA,
    TransferHookB,
    TransferHookReward,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RemainingAccountsSlice {
    pub accounts_type: AccountsType,
    pub length: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}
