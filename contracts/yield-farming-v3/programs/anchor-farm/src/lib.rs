use anchor_lang::prelude::*;

/// states
pub mod states;
///processor
pub mod processor;
/// error
pub mod error;
/// constant
pub mod constant;
/// instructions
pub mod instructions;
/// utils
pub mod utils;

use crate::{
    instructions::*,
    processor::*,
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_farm {
    use super::*;

    pub fn create_global_state(ctx: Context<SetGlobalState>, global_state_nonce:u8,  harvest_fee_numerator: u64, harvest_fee_denominator: u64) -> ProgramResult { 
        process_create_global_state(ctx, global_state_nonce, harvest_fee_numerator, harvest_fee_denominator) 
    }
    pub fn create_farm(ctx: Context<CreateFarm>, global_state_nonce:u8, farm_nonce: u8, farm_pool_lp_nonce: u8, farm_pool_reward_nonce: u8, start_timestamp: u64, end_timestamp: u64) -> ProgramResult { 
        process_create_farm(ctx, global_state_nonce, farm_nonce, farm_pool_lp_nonce, farm_pool_reward_nonce, start_timestamp, end_timestamp) 
    }
    pub fn create_dual(ctx: Context<CreateDual>, global_state_nonce:u8, farm_nonce: u8, farm_pool_reward_dual_nonce: u8, start_timestamp: u64, end_timestamp: u64) -> ProgramResult { 
        process_create_dual(ctx, global_state_nonce, farm_nonce, farm_pool_reward_dual_nonce, start_timestamp, end_timestamp) 
    }
    pub fn extend_farm(ctx: Context<ExtendFarm>, farm_nonce: u8, end_timestamp: u64) -> ProgramResult { 
        process_extend_farm(ctx, farm_nonce, end_timestamp) 
    }
    pub fn extend_dual(ctx: Context<ExtendDual>, farm_nonce: u8, end_timestamp: u64) -> ProgramResult { 
        process_extend_dual(ctx, farm_nonce, end_timestamp) 
    }
    pub fn deposit(ctx: Context<Deposit>, global_state_nonce: u8, farm_nonce: u8, farm_pool_lp_nonce: u8,  user_info_nonce: u8, with_swap_action: u8, amount: u64) -> ProgramResult { 
        process_deposit(ctx, global_state_nonce, farm_nonce, farm_pool_lp_nonce, user_info_nonce, with_swap_action, amount) 
    }
    pub fn withdraw(ctx: Context<Withdraw>, global_state_nonce: u8, farm_nonce: u8, farm_pool_lp_nonce: u8, user_info_nonce: u8, with_swap_action: u8, amount: u64) -> ProgramResult { 
        process_withdraw(ctx, global_state_nonce, farm_nonce, farm_pool_lp_nonce, user_info_nonce, with_swap_action, amount) 
    }
    pub fn add_reward_single(ctx: Context<AddRewardSingle>, global_state_nonce: u8, farm_nonce: u8,farm_pool_lp_nonce: u8,   farm_pool_reward_nonce: u8, amount: u64) -> ProgramResult { 
        process_add_reward_single(ctx, global_state_nonce, farm_nonce,farm_pool_lp_nonce,   farm_pool_reward_nonce, amount) 
    }
    pub fn add_reward_dual(ctx: Context<AddRewardDual>, global_state_nonce: u8, farm_nonce: u8,farm_pool_lp_nonce: u8,   farm_pool_reward_nonce: u8, amount: u64) -> ProgramResult { 
        process_add_reward_dual(ctx, global_state_nonce, farm_nonce,farm_pool_lp_nonce,   farm_pool_reward_nonce, amount) 
    }
    pub fn remove_reward_dual(ctx: Context<AddRewardDual>, global_state_nonce: u8, farm_nonce: u8,farm_pool_lp_nonce: u8,   farm_pool_reward_nonce: u8, amount: u64) -> ProgramResult { 
        process_remove_reward_dual(ctx, global_state_nonce, farm_nonce,farm_pool_lp_nonce,   farm_pool_reward_nonce, amount) 
    }
    pub fn harvest(ctx: Context<Harvest>, global_state_nonce: u8, farm_nonce: u8, user_info_nonce: u8, reward_type: u8) -> ProgramResult { 
        process_harvest(ctx, global_state_nonce, farm_nonce, user_info_nonce, reward_type) 
    }
}
