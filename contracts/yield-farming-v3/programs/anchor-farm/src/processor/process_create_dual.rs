use anchor_lang::prelude::*;

use crate::{
    instructions::*,
    error::*,
    states::*
};

pub fn process_create_dual(ctx: Context<CreateDual>, _global_state_nonce:u8, _farm_nonce: u8, _farm_pool_reward_dual_nonce: u8, _start_timestamp: u64, _end_timestamp: u64) -> ProgramResult {
    ctx.accounts.farm.assert_allowed()?;
    assert_true(ctx.accounts.creator.key() == ctx.accounts.global_state.super_owner)?;

    ctx.accounts.farm.set_state(FarmState::DualYield);
    ctx.accounts.farm.reward_mint_address_dual = ctx.accounts.pool_reward_mint_dual.key();
    ctx.accounts.farm.pool_reward_token_account_dual = ctx.accounts.pool_reward_token_dual.key();
    ctx.accounts.farm.reward_per_share_net_dual = 0;
    ctx.accounts.farm.start_timestamp_dual = _start_timestamp;
    ctx.accounts.farm.end_timestamp_dual = _end_timestamp;
    ctx.accounts.farm.last_timestamp_dual = _start_timestamp;
    ctx.accounts.farm.current_rewards_dual = 0;
    ctx.accounts.farm.distributed_rewards_dual = 0;
    ctx.accounts.farm.harvested_rewards_dual = 0;
    Ok(())
}
pub fn assert_amm_pool_mint(amm_pool_mint: Pubkey, pool_lp_mint: Pubkey)->ProgramResult {
    if amm_pool_mint != pool_lp_mint {
        return Err(FarmError::WrongPoolMint.into());
    }
    Ok(())
}