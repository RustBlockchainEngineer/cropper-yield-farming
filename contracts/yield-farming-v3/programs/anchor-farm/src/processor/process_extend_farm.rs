use anchor_lang::prelude::*;

use crate::{
    instructions::*,
};

pub fn process_extend_farm(ctx: Context<ExtendFarm>, _farm_nonce: u8, _end_timestamp: u64) -> ProgramResult {
    ctx.accounts.farm.assert_allowed()?;
    ctx.accounts.farm.end_timestamp = _end_timestamp;
    
    Ok(())
}