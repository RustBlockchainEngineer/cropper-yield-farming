use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Transfer};

use crate::{
    instructions::*,
    utils::*,
    constant::*
};

pub fn process_withdraw(ctx: Context<Withdraw>, _global_state_nonce: u8, _farm_nonce: u8, _farm_pool_lp_nonce: u8, _user_info_nonce: u8, _with_swap_action: u8, _amount: u64) -> ProgramResult {
    let cur_timestamp = ctx.accounts.clock.unix_timestamp as u64;
    ctx.accounts.farm.assert_allowed()?;
    assert_farm_started(cur_timestamp, ctx.accounts.farm.start_timestamp)?;

    let real_amount = get_real_amount_to_withdraw(ctx.accounts.user_info.deposit_balance, _with_swap_action, _amount)?;
    assert_true(real_amount > 0)?;
    
    ctx.accounts.farm.update(&mut ctx.accounts.user_info, cur_timestamp)?;

    if real_amount > 0 {
        // transfer from user to pool
        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_lp_token.to_account_info().clone(),
            to: ctx.accounts.user_lp_token.to_account_info().clone(),
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
        token::transfer(cpi_ctx, real_amount)?;

        ctx.accounts.user_info.deposit_balance -= real_amount;
        ctx.accounts.farm.pool_lp_balance -= real_amount;
    }

    ctx.accounts.farm.update_debt(&mut ctx.accounts.user_info)?;
    Ok(())
}