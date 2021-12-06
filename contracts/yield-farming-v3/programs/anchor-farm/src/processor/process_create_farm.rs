use anchor_lang::prelude::*;

use crate::{
    instructions::*,
    constant::*,
    utils::*,
    error::*,
    states::*
};
use cropper_liquidity_pool::amm_stats::{SwapVersion};

pub fn process_create_farm(ctx: Context<CreateFarm>, _global_state_nonce:u8, _farm_nonce: u8, _farm_pool_lp_nonce: u8, _farm_pool_reward_nonce: u8, _start_timestamp: u64, _end_timestamp: u64) -> ProgramResult {
    return Ok(());
    let amm_swap = SwapVersion::unpack(&ctx.accounts.amm_swap.data.borrow())?;
    assert_amm_pool_mint(*amm_swap.pool_mint(), ctx.accounts.pool_lp_mint.key())?;
    
    assert_locked_farm(amm_swap.token_a_mint(), amm_swap.token_b_mint(), &ctx.accounts.creator.key(), &ctx.accounts.global_state.allowed_creator)?;

    ctx.accounts.new_farm.set_state(FarmState::SingleYield);

    ctx.accounts.new_farm.owner = ctx.accounts.creator.key();
    ctx.accounts.new_farm.set_version(VERSION);
    ctx.accounts.new_farm.seed_key = *ctx.accounts.farm_seed.key;
    ctx.accounts.new_farm.pool_lp_token_account = ctx.accounts.pool_lp_token.key();
    ctx.accounts.new_farm.pool_reward_token_account = ctx.accounts.pool_reward_token.key();
    ctx.accounts.new_farm.pool_mint_address = ctx.accounts.pool_lp_mint.key();
    ctx.accounts.new_farm.pool_lp_balance = 0;
    ctx.accounts.new_farm.reward_per_share_net = 0;
    ctx.accounts.new_farm.start_timestamp = _start_timestamp;
    ctx.accounts.new_farm.end_timestamp = _end_timestamp;
    ctx.accounts.new_farm.last_timestamp = _start_timestamp;
    ctx.accounts.new_farm.current_rewards = 0;
    ctx.accounts.new_farm.distributed_rewards = 0;
    ctx.accounts.new_farm.harvested_rewards = 0;
    
    Ok(())
}
pub fn assert_amm_pool_mint(amm_pool_mint: Pubkey, pool_lp_mint: Pubkey)->ProgramResult {
    if amm_pool_mint != pool_lp_mint {
        return Err(FarmError::WrongPoolMint.into());
    }
    Ok(())
}