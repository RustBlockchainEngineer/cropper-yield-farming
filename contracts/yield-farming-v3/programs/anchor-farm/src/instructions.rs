use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount,Mint};

use crate::{
    states::*,
    constant::*,
};


#[derive(Accounts)]
#[instruction(global_state_nonce:u8, harvest_fee_numerator: u64, harvest_fee_denominator: u64)]
pub struct SetGlobalState <'info>{
    pub super_owner:  Signer<'info>,

    #[account(
    init_if_needed,
    seeds = [GLOBAL_STATE_TAG],
    bump = global_state_nonce,
    payer = super_owner,
    )]
    pub global_state:ProgramAccount<'info, FarmProgram>,

    pub new_super_owner: AccountInfo<'info>,
    pub fee_owner: AccountInfo<'info>,
    pub allowed_creator: AccountInfo<'info>,
    pub amm_program_id: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(global_state_nonce:u8, farm_nonce: u8, farm_pool_lp_nonce: u8, farm_pool_reward_nonce: u8, start_timestamp: u64, end_timestamp: u64)]
pub struct CreateFarm <'info>{
    pub creator:  Signer<'info>,

    #[account(
    seeds = [GLOBAL_STATE_TAG],
    bump = global_state_nonce,
    constraint = global_state.amm_program_id == *amm_swap.owner,
    )]
    pub global_state:ProgramAccount<'info, FarmProgram>,

    #[account(
        init,
        seeds = [FARM_TAG, farm_seed.key.as_ref()],
        bump = farm_nonce,
        payer = creator,
        constraint = end_timestamp > start_timestamp
        )]
    pub new_farm: ProgramAccount<'info, FarmPool>,
    pub farm_seed: AccountInfo<'info>,

    #[account(
        constraint = !pool_lp_mint.freeze_authority.is_some(),
    )]
    pub pool_lp_mint: Account<'info, Mint>,
    #[account(
        constraint = !pool_reward_mint.freeze_authority.is_some()
    )]
    pub pool_reward_mint: Account<'info, Mint>,
    #[account(init,
        token::mint = pool_lp_mint,
        token::authority = new_farm,
        seeds = [FARM_POOL_LP_TAG, new_farm.key().as_ref()],
        bump = farm_pool_lp_nonce,
        payer = creator)]
    pub pool_lp_token: Account<'info, TokenAccount>,
    #[account(init,
        token::mint = pool_reward_mint,
        token::authority = new_farm,
        seeds = [FARM_POOL_REWARD_TAG, new_farm.key().as_ref()],
        bump = farm_pool_reward_nonce,
        payer = creator)]
    pub pool_reward_token: Account<'info, TokenAccount>,
    pub amm_swap: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(global_state_nonce:u8, farm_nonce: u8, farm_pool_reward_dual_nonce: u8, start_timestamp: u64, end_timestamp: u64)]
pub struct CreateDual <'info>{
    #[account(
        constraint = global_state.super_owner == creator.key()
    )]
    pub creator:  Signer<'info>,

    #[account(
    seeds = [GLOBAL_STATE_TAG],
    bump = global_state_nonce,
    )]
    pub global_state:ProgramAccount<'info, FarmProgram>,

    #[account(mut,
        seeds = [FARM_TAG, farm_seed.key.as_ref()],
        bump = farm_nonce,
        constraint = end_timestamp > start_timestamp
        )]
    pub farm: ProgramAccount<'info, FarmPool>,
    pub farm_seed: AccountInfo<'info>,

    #[account(
        constraint = !pool_reward_mint_dual.freeze_authority.is_some()
    )]
    pub pool_reward_mint_dual: Account<'info, Mint>,
    
    #[account(init,
        token::mint = pool_reward_mint_dual,
        token::authority = farm,
        seeds = [DUAL_POOL_REWARD_TAG, farm.key().as_ref()],
        bump = farm_pool_reward_dual_nonce,
        payer = creator)]
    pub pool_reward_token_dual: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(farm_nonce: u8, end_timestamp: u64)]
pub struct ExtendFarm <'info>{
    #[account(
        constraint = creator.key() == farm.owner, 
        constraint = end_timestamp > farm.end_timestamp
    )]
    pub creator:  Signer<'info>,

    #[account(mut,
        seeds = [FARM_TAG, farm_seed.key.as_ref()],
        bump = farm_nonce,
        )]
    pub farm: ProgramAccount<'info, FarmPool>,
    pub farm_seed: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(farm_nonce: u8, end_timestamp: u64)]
pub struct ExtendDual<'info>{
    #[account(
        constraint = creator.key() == farm.owner
    )]
    pub creator:  Signer<'info>,

    #[account(mut,
        seeds = [FARM_TAG, farm_seed.key.as_ref()],
        bump = farm_nonce,
        constraint = end_timestamp > farm.end_timestamp_dual
        )]
    pub farm: ProgramAccount<'info, FarmPool>,
    pub farm_seed: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(global_state_nonce: u8, farm_nonce: u8, farm_pool_lp_nonce: u8, user_info_nonce: u8, with_swap_action: u8, amount: u64)]
pub struct Deposit <'info>{
    pub depositor:  Signer<'info>,

    #[account(
        seeds = [GLOBAL_STATE_TAG],
        bump = global_state_nonce,
    )]
    pub global_state:ProgramAccount<'info, FarmProgram>,
    
    #[account(mut,
        seeds = [FARM_TAG, farm_seed.key.as_ref()],
        bump = farm_nonce,
        )]
    pub farm: ProgramAccount<'info, FarmPool>, 
    pub farm_seed: AccountInfo<'info>,

    #[account(
        init_if_needed,
        seeds = [USER_INFO_TAG, farm.key().as_ref(), depositor.key().as_ref()],
        bump = user_info_nonce,
        payer = depositor,
        )]
    pub user_info: ProgramAccount<'info, UserInfo>,
    
    #[account(mut,
        seeds = [FARM_POOL_LP_TAG, farm.key().as_ref()],
        bump = farm_pool_lp_nonce,
    )]
    pub pool_lp_token: Account<'info, TokenAccount>,

    #[account(mut,
        constraint = user_lp_token.mint == farm.pool_mint_address,
        constraint = user_lp_token.owner == depositor.key(),
    )]
    pub user_lp_token: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(global_state_nonce: u8, farm_nonce: u8, farm_pool_lp_nonce: u8, user_info_nonce: u8, with_swap_action: u8, amount: u64)]
pub struct Withdraw <'info>{
    pub withdrawer:  Signer<'info>,

    #[account(
        seeds = [GLOBAL_STATE_TAG],
        bump = global_state_nonce,
    )]
    pub global_state:ProgramAccount<'info, FarmProgram>,
    
    #[account(mut,
        seeds = [FARM_TAG, farm_seed.key.as_ref()],
        bump = farm_nonce,
        )]
    pub farm: ProgramAccount<'info, FarmPool>, 
    pub farm_seed: AccountInfo<'info>,

    #[account(
        seeds = [USER_INFO_TAG, farm.key().as_ref(), withdrawer.key().as_ref()],
        bump = user_info_nonce,
        )]
    pub user_info: ProgramAccount<'info, UserInfo>,
    
    #[account(mut,
        seeds = [FARM_POOL_LP_TAG, farm.key().as_ref()],
        bump = farm_pool_lp_nonce,
    )]
    pub pool_lp_token: Account<'info, TokenAccount>,

    #[account(mut,
        constraint = user_lp_token.mint == farm.pool_mint_address,
        constraint = user_lp_token.owner == withdrawer.key(),
    )]
    pub user_lp_token: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(global_state_nonce: u8, farm_nonce: u8,farm_pool_lp_nonce: u8,  farm_pool_reward_nonce: u8, amount: u64)]
pub struct AddRewardSingle <'info>{
    pub depositor:  Signer<'info>,

    #[account(
        seeds = [GLOBAL_STATE_TAG],
        bump = global_state_nonce,
    )]
    pub global_state:ProgramAccount<'info, FarmProgram>,
    
    #[account(mut,
        seeds = [FARM_TAG, farm_seed.key.as_ref()],
        bump = farm_nonce,
        
        )]
    pub farm: ProgramAccount<'info, FarmPool>, 
    pub farm_seed: AccountInfo<'info>,

    #[account(mut,
        seeds = [FARM_POOL_LP_TAG, farm.key().as_ref()],
        bump = farm_pool_lp_nonce,
    )]
    pub pool_lp_token: Account<'info, TokenAccount>,

    #[account(mut,
        seeds = [FARM_POOL_REWARD_TAG, farm.key().as_ref()],
        bump = farm_pool_reward_nonce,
    )]
    pub pool_reward_token: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_reward_token: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(global_state_nonce: u8, farm_nonce: u8, farm_pool_lp_nonce: u8,  farm_pool_reward_nonce: u8, amount: u64)]
pub struct AddRewardDual <'info>{
    pub depositor:  Signer<'info>,

    #[account(
        seeds = [GLOBAL_STATE_TAG],
        bump = global_state_nonce,
    )]
    pub global_state:ProgramAccount<'info, FarmProgram>,
    
    #[account(mut,
        seeds = [FARM_TAG, farm_seed.key.as_ref()],
        bump = farm_nonce,
        )]
    pub farm: ProgramAccount<'info, FarmPool>, 
    pub farm_seed: AccountInfo<'info>,

    #[account(mut,
        seeds = [FARM_POOL_LP_TAG, farm.key().as_ref()],
        bump = farm_pool_lp_nonce,
    )]
    pub pool_lp_token: Account<'info, TokenAccount>,

    #[account(mut,
        seeds = [DUAL_POOL_REWARD_TAG, farm.key().as_ref()],
        bump = farm_pool_reward_nonce,
    )]
    pub pool_reward_token_dual: Account<'info, TokenAccount>,
    
    #[account(mut,
        constraint = user_reward_token_dual.mint == farm.reward_mint_address_dual,
        constraint = user_reward_token_dual.owner == depositor.key(),
    )]
    pub user_reward_token_dual: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}


#[derive(Accounts)]
#[instruction(global_state_nonce: u8, farm_nonce: u8, user_info_nonce: u8, reward_type: u8)]
pub struct Harvest <'info>{
    pub harvester:  Signer<'info>,

    #[account(
        seeds = [GLOBAL_STATE_TAG],
        bump = global_state_nonce,
    )]
    pub global_state:ProgramAccount<'info, FarmProgram>,
    
    #[account(mut,
        seeds = [FARM_TAG, farm_seed.key.as_ref()],
        bump = farm_nonce,
        )]
    pub farm: ProgramAccount<'info, FarmPool>, 
    pub farm_seed: AccountInfo<'info>,

    #[account(mut,
        seeds = [USER_INFO_TAG, farm.key().as_ref(), harvester.key().as_ref()],
        bump = user_info_nonce,
        )]
    pub user_info: ProgramAccount<'info, UserInfo>,
    
    #[account(mut)]
    pub pool_reward_token: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_reward_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub fee_reward_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}
