use anchor_lang::prelude::*;

use crate::{
    instructions::*,
};

pub fn process_extend_dual(ctx: Context<ExtendDual>, _farm_nonce: u8, _end_timestamp: u64) -> ProgramResult {
    ctx.accounts.farm.assert_allowed()?;
    ctx.accounts.farm.end_timestamp_dual = _end_timestamp;
    
    Ok(())
}