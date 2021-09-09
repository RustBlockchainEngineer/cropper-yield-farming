//! State transition types
//! State stores account data and manage version upgrade

#![allow(clippy::too_many_arguments)]
use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        pubkey::{Pubkey},
    },
};
use crate::constant::{
    REWARD_MULTIPLER,
};
/// Farm Pool struct
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct FarmPool {
    /// allowed flag for the additional fee to create farm
    pub is_allowed: u8,
    
    /// nonce is used to authorize this farm pool
    pub nonce: u8,

    /// This account stores lp token
    pub pool_lp_token_account: Pubkey,

    /// This account stores reward token
    pub pool_reward_token_account: Pubkey,

    /// lp token's mint address
    pub pool_mint_address: Pubkey,

    /// reward token's mint address
    pub reward_mint_address: Pubkey,

    /// spl-token program id
    pub token_program_id: Pubkey,
    
    /// owner wallet address of this farm
    pub owner: Pubkey,

    /// fee owner wallet address to receive harvest fees
    /// This wallet account should have all token accounts
    pub fee_owner: Pubkey,

    /// This represents the total reward amount what a farmer can receive for unit lp
    pub reward_per_share_net: u64,

    /// latest reward time
    pub last_timestamp: u64,

    /// reward per second
    pub reward_per_timestamp: u64,

    /// start time of this farm
    pub start_timestamp: u64,

    /// end time of this farm
    pub end_timestamp: u64,

}
impl FarmPool {
    /// get current pending reward amount for a user
    pub fn pending_rewards(&self, user_info:&mut UserInfo) -> u64{
        let reward_per_share_net: u64 = self.reward_per_share_net;
        return user_info.deposit_balance * reward_per_share_net / REWARD_MULTIPLER - user_info.reward_debt;
    }

    /// get total reward amount for a user so far
    pub fn get_new_reward_debt(&self, user_info:&UserInfo) -> u64{
        return user_info.deposit_balance * self.reward_per_share_net / REWARD_MULTIPLER;
    }
    
}

/// User information struct
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct UserInfo {
    
    /// user's wallet address
    pub wallet: Pubkey,

    /// farm account address what this user deposited
    pub farm_id: Pubkey,

    /// current deposited balance
    pub deposit_balance: u64,

    /// reward debt so far
    pub reward_debt: u64,
}
