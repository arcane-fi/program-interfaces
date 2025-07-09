#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK");

#[program]
pub mod raydium_clmm_interface {
    use super::*;

    #[allow(unused_variables)]
    pub fn swap_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapSingleV2<'info>>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn increase_liquidity_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, IncreaseLiquidityV2<'info>>,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
        base_flag: Option<bool>,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn decrease_liquidity_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, DecreaseLiquidityV2<'info>>,
        liquidity: u128,
        amount_0_min: u64,
        amount_1_min: u64,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn open_position_with_token22_nft<'info>(
        ctx: Context<'_, '_, '_, 'info, OpenPositionWithToken22Nft<'info>>,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
        tick_lower_index: i32,
        tick_upper_index: i32,
        tick_array_lower_start_index: i32,
        tick_array_upper_start_index: i32,
        with_metadata: bool,
        base_flag: Option<bool>,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SwapSingleV2<'info> {
    /// CHECK: validated by raydium
    pub payer: AccountInfo<'info>,

    /// CHECK: validated by raydium
    pub amm_config: AccountInfo<'info>,

    /// CHECK: validated by raydium
    #[account(mut)]
    pub pool_state: AccountInfo<'info>,

    /// CHECK: validated by raydium
    #[account(mut)]
    pub input_token_account: AccountInfo<'info>,

    /// CHECK: validated by raydium
    #[account(mut)]
    pub output_token_account: AccountInfo<'info>,

    /// CHECK: validated by raydium
    #[account(mut)]
    pub input_vault: AccountInfo<'info>,

    /// CHECK: validated by raydium
    #[account(mut)]
    pub output_vault: AccountInfo<'info>,

    /// CHECK: validated by raydium
    #[account(mut)]
    pub observation_state: AccountInfo<'info>,

    /// CHECK: validated by raydium
    pub token_program: AccountInfo<'info>,

    /// CHECK: validated by raydium
    pub token_program_2022: AccountInfo<'info>,

    /// CHECK: validated by raydium
    pub memo_program: AccountInfo<'info>,

    /// CHECK: validated by raydium
    pub input_vault_mint: AccountInfo<'info>,

    /// CHECK: validated by raydium
    pub output_vault_mint: AccountInfo<'info>,
    // remaining accounts
    // tickarray_bitmap_extension: must add account if need regardless the sequence
    // tick_array_account_1
    // tick_array_account_2
    // tick_array_account_...
}

#[derive(Accounts)]
pub struct IncreaseLiquidityV2<'info> {
    /// CHECK:
    pub nft_owner: Signer<'info>,

    /// CHECK:
    pub nft_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub pool_state: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub protocol_position: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub personal_position: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub tick_array_lower: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub tick_array_upper: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub token_account_0: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub token_account_1: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub token_vault_0: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub token_vault_1: UncheckedAccount<'info>,

    /// CHECK:
    pub token_program: UncheckedAccount<'info>,

    /// CHECK:
    pub token_program_2022: UncheckedAccount<'info>,

    /// CHECK:
    pub vault_0_mint: UncheckedAccount<'info>,

    /// CHECK:
    pub vault_1_mint: UncheckedAccount<'info>,
    // remaining accounts
}

#[derive(Accounts)]
pub struct DecreaseLiquidityV2<'info> {
    pub nft_owner: Signer<'info>,

    /// CHECK:
    pub nft_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub personal_position: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub pool_state: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub protocol_position: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub token_vault_0: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub token_vault_1: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub tick_array_lower: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub tick_array_upper: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub recipient_token_account_0: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub recipient_token_account_1: UncheckedAccount<'info>,

    /// CHECK:
    pub token_program: UncheckedAccount<'info>,

    /// CHECK:
    pub token_program_2022: UncheckedAccount<'info>,

    /// CHECK:
    pub memo_program: UncheckedAccount<'info>,

    /// CHECK:
    pub vault_0_mint: UncheckedAccount<'info>,

    /// CHECK:
    pub vault_1_mint: UncheckedAccount<'info>,
    // remaining accounts
}

#[derive(Accounts)]
pub struct OpenPositionWithToken22Nft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK:
    pub position_nft_owner: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub position_nft_mint: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub position_nft_account: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub pool_state: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub protocol_position: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub tick_array_lower: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub tick_array_upper: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub personal_position: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub token_account_0: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub token_account_1: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub token_vault_0: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub token_vault_1: UncheckedAccount<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,

    /// CHECK:
    pub token_program: UncheckedAccount<'info>,

    /// CHECK:
    pub associated_token_program: UncheckedAccount<'info>,

    /// CHECK:
    pub token_program_2022: UncheckedAccount<'info>,

    /// CHECK:
    pub vault_0_mint: UncheckedAccount<'info>,

    /// CHECK:
    pub vault_1_mint: UncheckedAccount<'info>,
    // remaining accounts
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    /// CHECK:
    #[account(mut)]
    pub nft_owner: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub position_nft_mint: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub position_nft_account: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub personal_position: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}
