use anchor_lang::prelude::*;

use crate::{
    instructions::*,
    constant::*,
    utils::*,
    error::*
};
use std::str::FromStr;

pub fn process_create_global_state(ctx: Context<SetGlobalState>, _global_state_nonce:u8,  _harvest_fee_numerator: u64, _harvest_fee_denominator: u64) -> ProgramResult {
    if is_zero_account(&ctx.accounts.global_state.to_account_info()) {
        ctx.accounts.global_state.super_owner = Pubkey::from_str(INITIAL_SUPER_OWNER).map_err(|_| FarmError::InvalidPubkey)?;
    }
    assert_owner(ctx.accounts.global_state.super_owner, ctx.accounts.super_owner.key())?;

    ctx.accounts.global_state.reward_multipler = REWARD_MULTIPLER;
    ctx.accounts.global_state.super_owner = *ctx.accounts.new_super_owner.key;
    ctx.accounts.global_state.fee_owner = *ctx.accounts.fee_owner.key;
    ctx.accounts.global_state.allowed_creator = *ctx.accounts.allowed_creator.key;
    ctx.accounts.global_state.amm_program_id = *ctx.accounts.amm_program_id.key;
    ctx.accounts.global_state.harvest_fee_numerator = _harvest_fee_numerator;
    ctx.accounts.global_state.harvest_fee_denominator = _harvest_fee_denominator;

    Ok(())
}