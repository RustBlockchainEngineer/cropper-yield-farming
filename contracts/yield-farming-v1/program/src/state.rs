//! State transition types
//! State stores account data and manage version upgrade

#![allow(clippy::too_many_arguments)]
use {
    crate::{
        error::FarmError,
        constant::*,
    },
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        pubkey::{Pubkey},
        program_error::ProgramError,
    },
    spl_math::{precise_number::PreciseNumber},
    std::convert::TryFrom,
};


/// Farm Pool struct
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct FarmProgram {
    /// program version
    pub version: u8,
    
    /// super owner of this program. this owner can change program state
    pub super_owner: Pubkey,

    /// fee owner wallet address
    pub fee_owner: Pubkey,

    /// allowed creator - This is allowed wallet address to create specified farms
    /// Specified farms are SOL-USDC, SOL-USDT, ETH-USDC, ETH-USDT, CRP-USDC, CRP-USDT, CRP-SOL, CRP-ETH
    pub allowed_creator: Pubkey,

    /// AMM program id
    pub amm_program_id: Pubkey,
    
    /// farm fee for not CRP token pairing farm
    pub farm_fee: u64,

    /// harvest fee numerator
    pub harvest_fee_numerator: u64,

    /// harvest fee denominator
    pub harvest_fee_denominator: u64,

    /// reward multipler
    pub reward_multipler: u64,
    
}

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

    /// This represents the total reward amount what a farmer can receive for unit lp
    pub reward_per_share_net: u128,

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
    pub fn pending_rewards(&self, user_info:&mut UserInfo) -> Result<u64, ProgramError> {
        let deposit_balance = PreciseNumber::new(user_info.deposit_balance as u128).ok_or(FarmError::PreciseError)?;
        let reward_per_share_net = PreciseNumber::new(self.reward_per_share_net as u128).ok_or(FarmError::PreciseError)?;
        let reward_multipler = PreciseNumber::new(REWARD_MULTIPLER as u128).ok_or(FarmError::PreciseError)?;
        let reward_debt = PreciseNumber::new(user_info.reward_debt as u128).ok_or(FarmError::PreciseError)?;

        let result = deposit_balance.checked_mul(&reward_per_share_net).ok_or(FarmError::PreciseError)?
                    .checked_div(&reward_multipler).ok_or(FarmError::PreciseError)?
                    .checked_sub(&reward_debt).ok_or(FarmError::PreciseError)?;

        Ok(u64::try_from(result.to_imprecise().ok_or(FarmError::PreciseError)?).unwrap_or(0))
    }

    /// get total reward amount for a user so far
    pub fn get_new_reward_debt(&self, user_info:&UserInfo) -> Result<u64, ProgramError>{
        let deposit_balance = PreciseNumber::new(user_info.deposit_balance as u128).ok_or(FarmError::PreciseError)?;
        let reward_per_share_net = PreciseNumber::new(self.reward_per_share_net as u128).ok_or(FarmError::PreciseError)?;
        let reward_multipler = PreciseNumber::new(REWARD_MULTIPLER as u128).ok_or(FarmError::PreciseError)?;

        let result = deposit_balance.checked_mul(&reward_per_share_net).ok_or(FarmError::PreciseError)?
                    .checked_div(&reward_multipler).ok_or(FarmError::PreciseError)?;
                    
        Ok(u64::try_from(result.to_imprecise().ok_or(FarmError::PreciseError)?).unwrap_or(0))
    }
    /// get harvest fee
    pub fn get_harvest_fee(&self, pending:u64, program_data:&FarmProgram) -> Result<u64, ProgramError>{
        
        let harvest_fee_numerator = PreciseNumber::new(program_data.harvest_fee_numerator as u128).ok_or(FarmError::PreciseError)?;
        let harvest_fee_denominator = PreciseNumber::new(program_data.harvest_fee_denominator as u128).ok_or(FarmError::PreciseError)?;
        let pending = PreciseNumber::new(pending as u128).ok_or(FarmError::PreciseError)?;

        let result = pending.checked_mul(&harvest_fee_numerator).ok_or(FarmError::PreciseError)?
                    .checked_div(&harvest_fee_denominator).ok_or(FarmError::PreciseError)?;
                    
        Ok(u64::try_from(result.to_imprecise().ok_or(FarmError::PreciseError)?).unwrap_or(0))
    }
    pub fn update_share(&mut self, cur_timestamp:u64, _lp_balance:u64) -> Result<(), ProgramError>{
        let mut _calc_timestamp = cur_timestamp;
        if cur_timestamp > self.end_timestamp {
            _calc_timestamp = self.end_timestamp;
        }

        let duration = PreciseNumber::new((_calc_timestamp - self.last_timestamp) as u128).ok_or(FarmError::PreciseError)?;
        let reward_per_timestamp = PreciseNumber::new(self.reward_per_timestamp as u128).ok_or(FarmError::PreciseError)?;
        let reward_multipler = PreciseNumber::new(REWARD_MULTIPLER as u128).ok_or(FarmError::PreciseError)?;
        let reward_per_share_net = PreciseNumber::new(self.reward_per_share_net as u128).ok_or(FarmError::PreciseError)?;
        let lp_balance = PreciseNumber::new(_lp_balance as u128).ok_or(FarmError::PreciseError)?;

        let reward = duration.checked_mul(&reward_per_timestamp).ok_or(FarmError::PreciseError)?;
        let updated_share = reward_multipler.checked_mul(&reward).ok_or(FarmError::PreciseError)?
                            .checked_div(&lp_balance).ok_or(FarmError::PreciseError)?
                            .checked_add(&reward_per_share_net).ok_or(FarmError::PreciseError)?;

        self.reward_per_share_net = updated_share.to_imprecise().ok_or(FarmError::PreciseError)?;

        Ok(())
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
