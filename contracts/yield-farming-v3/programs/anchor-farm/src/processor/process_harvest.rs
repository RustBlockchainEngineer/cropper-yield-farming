use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{
    instructions::*,
    utils::*,
    states::*,
    constant::*
};

pub fn process_harvest(ctx: Context<Harvest>, _global_state_nonce: u8, _farm_nonce: u8, _user_info_nonce: u8, _reward_type: u8) -> ProgramResult {
    let cur_timestamp = ctx.accounts.clock.unix_timestamp as u64;
    let mut tag = FARM_POOL_REWARD_TAG;
    assert_true(ctx.accounts.user_reward_token.owner == ctx.accounts.harvester.key())?;
    assert_true(ctx.accounts.fee_reward_token.owner == ctx.accounts.global_state.fee_owner)?;
    if RewardType::is_single(_reward_type) {
        assert_true(ctx.accounts.user_reward_token.mint == ctx.accounts.farm.reward_mint_address)?;
        assert_true(ctx.accounts.fee_reward_token.mint == ctx.accounts.farm.reward_mint_address)?;
    }
    else if RewardType::is_dual(_reward_type) {
        assert_true(ctx.accounts.user_reward_token.mint == ctx.accounts.farm.reward_mint_address_dual)?;
        assert_true(ctx.accounts.fee_reward_token.mint == ctx.accounts.farm.reward_mint_address_dual)?;
        tag = DUAL_POOL_REWARD_TAG;
    }

    let farm_key = ctx.accounts.farm.key();
    let pool_reward_seeds = [tag, farm_key.as_ref()];
    assert_pda(&pool_reward_seeds, ctx.program_id, &ctx.accounts.pool_reward_token.key())?;

    ctx.accounts.farm.update(&mut ctx.accounts.user_info, cur_timestamp)?;

    harvest(
        &mut ctx.accounts.user_info, 
        &ctx.accounts.global_state, 
        _farm_nonce, 
        &ctx.accounts.token_program, 
        &ctx.accounts.pool_reward_token, 
        &ctx.accounts.fee_reward_token, 
        &ctx.accounts.user_reward_token, 
        &mut ctx.accounts.farm, 
        RewardType::is_dual(_reward_type)
    )?;

    Ok(())
}


pub fn harvest<'info>(
    user_info:&mut UserInfo, 
    global_state: &FarmProgram, 
    farm_nonce: u8,
    token_program: &Program<'info, Token>,
    pool_reward_token: &Account<'info, TokenAccount>,
    fee_reward_token: &Account<'info, TokenAccount>,
    user_reward_token: &Account<'info, TokenAccount>,
    farm: &mut ProgramAccount<'info, FarmPool>,
    is_dual: bool,
) -> ProgramResult{
    let mut pending = if is_dual {user_info.pending_rewards_dual} else {user_info.pending_rewards};
    if pool_reward_token.amount < pending {
        pending = pool_reward_token.amount;
    }

    if pending > 0 {
        let harvest_fee = farm.get_harvest_fee(pending, global_state)?;
        let user_pending = pending - harvest_fee;

        let cpi_program = token_program.to_account_info();

        let signer_seeds = &[
            FARM_TAG, 
            farm.seed_key.as_ref(),
            &[farm_nonce]
        ];
        let signer = &[&signer_seeds[..]];

        let cpi_accounts_fee = Transfer {
            from: pool_reward_token.to_account_info(),
            to: fee_reward_token.to_account_info(),
            authority: farm.to_account_info(),
        };
        let cpi_ctx_fee = CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts_fee, signer);
        token::transfer(cpi_ctx_fee, harvest_fee)?;

        let cpi_accounts_user = Transfer {
            from: pool_reward_token.to_account_info(),
            to: user_reward_token.to_account_info(),
            authority: farm.to_account_info(),
        };
        
        let cpi_ctx_user = CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts_user, signer);
        token::transfer(cpi_ctx_user, user_pending)?;

        if is_dual {
            farm.harvested_rewards_dual += pending;
            user_info.pending_rewards_dual = 0;
            user_info.reward_debt_dual = farm.get_new_reward_debt_dual(user_info)?;
        }
        else {
            farm.harvested_rewards += pending;
            user_info.pending_rewards = 0;
            user_info.reward_debt = farm.get_new_reward_debt(user_info)?;
        }
    }
    Ok(())
}