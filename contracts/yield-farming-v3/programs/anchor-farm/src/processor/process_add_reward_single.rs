use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Transfer};

use crate::{
    instructions::*,
    utils::*
};

pub fn process_add_reward_single(ctx: Context<AddRewardSingle>, _global_state_nonce: u8, _farm_nonce: u8, _farm_pool_lp_nonce: u8,  _farm_pool_reward_nonce: u8, _amount: u64) -> ProgramResult {
    assert_true(ctx.accounts.farm.owner == ctx.accounts.depositor.key())?;
    
    let cur_timestamp = ctx.accounts.clock.unix_timestamp as u64;

    if _amount > 0 {
        ctx.accounts.farm.update_share(cur_timestamp, ctx.accounts.pool_lp_token.amount, ctx.accounts.pool_reward_token.amount)?;

        // transfer from user to pool
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_reward_token.to_account_info().clone(),
            to: ctx.accounts.pool_reward_token.to_account_info().clone(),
            authority: ctx.accounts.depositor.to_account_info().clone(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info().clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, _amount)?;
    }
    ctx.accounts.farm.current_rewards = ctx.accounts.pool_reward_token.amount;
    
    Ok(())
}