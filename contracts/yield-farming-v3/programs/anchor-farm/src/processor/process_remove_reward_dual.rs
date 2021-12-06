use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Transfer};

use crate::{
    instructions::*,
    constant::*,
    utils::*
};

pub fn process_remove_reward_dual(ctx: Context<AddRewardDual>, _global_state_nonce: u8, _farm_nonce: u8, _farm_pool_lp_nonce: u8,  _farm_pool_reward_nonce: u8, _amount: u64) -> ProgramResult {
    assert_true(ctx.accounts.global_state.super_owner == ctx.accounts.depositor.key())?;

    let cur_timestamp = ctx.accounts.clock.unix_timestamp as u64;
    ctx.accounts.farm.assert_dual_yield()?;
    
    // limitation of remove
    // add code here
    let removal_amount = ctx.accounts.farm.removal_reward_dual_amount(ctx.accounts.pool_reward_token_dual.amount, _amount)?;
    
    assert_true(removal_amount > 0)?;

    if removal_amount > 0 {
        ctx.accounts.farm.update_share_dual(cur_timestamp, ctx.accounts.pool_lp_token.amount, ctx.accounts.pool_reward_token_dual.amount)?;

        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_reward_token_dual.to_account_info().clone(),
            to: ctx.accounts.user_reward_token_dual.to_account_info().clone(),
            authority: ctx.accounts.farm.to_account_info().clone(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info().clone();
        let signer_seeds = &[
            FARM_TAG, 
            ctx.accounts.farm.seed_key.as_ref(),
            &[_farm_nonce]
        ];
        let signer = &[&signer_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, removal_amount)?;
        
        ctx.accounts.farm.current_rewards_dual = ctx.accounts.pool_reward_token_dual.amount;
    }
    Ok(())
}