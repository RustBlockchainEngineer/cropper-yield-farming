use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Transfer};

use crate::{
    instructions::*,
    utils::*
};

pub fn process_deposit(ctx: Context<Deposit>, _global_state_nonce: u8, _farm_nonce: u8, _farm_pool_lp_nonce: u8, _user_info_nonce: u8, _with_swap_action: u8, _amount: u64) -> ProgramResult {
    let cur_timestamp = ctx.accounts.clock.unix_timestamp as u64;
    ctx.accounts.farm.assert_allowed()?;
    assert_farm_period(cur_timestamp, ctx.accounts.farm.start_timestamp, ctx.accounts.farm.end_timestamp)?;

    let is_user_info_zero_account = is_zero_account(&ctx.accounts.user_info.to_account_info());
    if is_user_info_zero_account {
        ctx.accounts.user_info.wallet = ctx.accounts.depositor.key();
        ctx.accounts.user_info.farm_id = ctx.accounts.farm.key();
        ctx.accounts.user_info.deposit_balance = 0;
        ctx.accounts.user_info.reward_debt = 0;
        ctx.accounts.user_info.reward_debt_dual = 0;
    }
    let real_amount = get_real_amount_to_deposit(ctx.accounts.user_lp_token.amount, _with_swap_action, _amount)?;

    ctx.accounts.farm.update(&mut ctx.accounts.user_info, cur_timestamp)?;
    
    if real_amount > 0 {
        // transfer from user to pool
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_lp_token.to_account_info().clone(),
            to: ctx.accounts.pool_lp_token.to_account_info().clone(),
            authority: ctx.accounts.depositor.to_account_info().clone(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info().clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, real_amount)?;

        ctx.accounts.user_info.deposit_balance += real_amount;
        ctx.accounts.farm.pool_lp_balance += real_amount;
    }
    ctx.accounts.farm.update_debt(&mut ctx.accounts.user_info)?;
    Ok(())
}