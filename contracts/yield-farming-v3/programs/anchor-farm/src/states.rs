use anchor_lang::prelude::*;

use crate::{
    constant::*,
    error::*,
    utils::*
};

#[account]
#[derive(Default)]
pub struct FarmProgram {
    /// super owner of this program. this owner can change program state
    pub super_owner: Pubkey,

    /// fee owner wallet address
    pub fee_owner: Pubkey,

    /// allowed creator - This is allowed wallet address to create specified farms
    /// Specified farms are SOL-USDC, SOL-USDT, ETH-USDC, ETH-USDT, CRP-USDC, CRP-USDT, CRP-SOL, CRP-ETH
    pub allowed_creator: Pubkey,

    /// AMM program id
    pub amm_program_id: Pubkey,
    
    /// harvest fee numerator
    pub harvest_fee_numerator: u64,

    /// harvest fee denominator
    pub harvest_fee_denominator: u64,

    /// reward multipler
    pub reward_multipler: u64,

    /// reserve
    pub reserve1: Pubkey,
    pub reserve2: Pubkey,
    pub reserve3: Pubkey,
}
#[account]
#[derive(Default)]
pub struct FarmPool {
    /// owner wallet address of this farm
    pub owner: Pubkey,

    /// allowed flag for the additional fee to create farm
    pub state: u8,

    /// pool version
    pub version: u8,
    
    /// seed_key is used to authorize this farm pool
    pub seed_key: Pubkey,

    /// This account stores lp token
    pub pool_lp_token_account: Pubkey,

    /// current pool's lp total balance
    pub pool_lp_balance: u64,

    /// This account stores reward token
    pub pool_reward_token_account: Pubkey,

    /// lp token's mint address
    pub pool_mint_address: Pubkey,

    /// reward token's mint address
    pub reward_mint_address: Pubkey,

    /// This represents the total reward amount what a farmer can receive for unit lp
    pub reward_per_share_net: u128,

    /// latest reward time
    pub last_timestamp: u64,

    /// current real reward amount
    pub current_rewards: u64,

    /// distributed reward amount
    pub distributed_rewards: u64,

    /// harvested reward amount
    pub harvested_rewards: u64,

    /// start time of this farm
    pub start_timestamp: u64,

    /// end time of this farm
    pub end_timestamp: u64,

    /// This account stores reward token
    pub pool_reward_token_account_dual: Pubkey,

    /// reward token's mint address
    pub reward_mint_address_dual: Pubkey,

    /// This represents the total reward amount what a farmer can receive for unit lp
    pub reward_per_share_net_dual: u128,

    /// latest reward time
    pub last_timestamp_dual: u64,

    /// current real reward amount
    pub current_rewards_dual: u64,

    /// distributed reward amount
    pub distributed_rewards_dual: u64,

    /// harvested reward amount
    pub harvested_rewards_dual: u64,

    /// start time of this farm
    pub start_timestamp_dual: u64,

    /// end time of this farm
    pub end_timestamp_dual: u64,

    /// reserve
    pub reserve1: Pubkey,
    pub reserve2: Pubkey,
    pub reserve3: Pubkey,
    pub reserve4: Pubkey,
}

impl FarmPool {
    pub fn pending_rewards(&self, user_info:&UserInfo) -> Result<u64> {
        let deposit_balance = user_info.deposit_balance.to_precise()?;
        let reward_per_share_net = self.reward_per_share_net.to_precise()?;
        let reward_multipler = REWARD_MULTIPLER.to_precise()?;
        let reward_debt = user_info.reward_debt.to_precise()?;

        let mut pending = deposit_balance.checked_mul(&reward_per_share_net).ok_or(FarmError::PreciseError)?
                    .checked_div(&reward_multipler).ok_or(FarmError::PreciseError)?;

        if reward_debt.to_imprecise().ok_or(FarmError::PreciseError)? > 0 {
            pending = pending.checked_sub(&reward_debt).ok_or(FarmError::PreciseError)?;
        }
        Ok(pending.to_u64()?)
    }
    /// get total reward amount for a user so far
    pub fn get_new_reward_debt(&self, user_info:&UserInfo) -> Result<u64>{
        let deposit_balance = user_info.deposit_balance.to_precise()?;
        let reward_per_share_net = self.reward_per_share_net.to_precise()?;
        let reward_multipler = REWARD_MULTIPLER.to_precise()?;

        let result = deposit_balance.checked_mul(&reward_per_share_net).ok_or(FarmError::PreciseError)?
                    .checked_div(&reward_multipler).ok_or(FarmError::PreciseError)?;
                    
        Ok(result.to_u64()?)
    }
    /// get harvest fee
    pub fn get_harvest_fee(&self, pending:u64, program_data:&FarmProgram) -> Result<u64>{
        let harvest_fee_numerator = program_data.harvest_fee_numerator.to_precise()?;
        let harvest_fee_denominator = program_data.harvest_fee_denominator.to_precise()?;
        let _pending = pending.to_precise()?;

        let result = _pending.checked_mul(&harvest_fee_numerator).ok_or(FarmError::PreciseError)?
                    .checked_div(&harvest_fee_denominator).ok_or(FarmError::PreciseError)?;
                    
        Ok(result.to_u64()?)
    }
    pub fn pending_rewards_dual(&self, user_info:&mut UserInfo) -> Result<u64> {
        self.assert_dual_yield()?;

        let deposit_balance = user_info.deposit_balance.to_precise()?;
        let reward_per_share_net = self.reward_per_share_net_dual.to_precise()?;
        let reward_multipler = REWARD_MULTIPLER.to_precise()?;
        let reward_debt = user_info.reward_debt_dual.to_precise()?;

        let mut pending = deposit_balance.checked_mul(&reward_per_share_net).ok_or(FarmError::PreciseError)?
                    .checked_div(&reward_multipler).ok_or(FarmError::PreciseError)?;

        if reward_debt.to_imprecise().ok_or(FarmError::PreciseError)? > 0 {
            pending = pending.checked_sub(&reward_debt).ok_or(FarmError::PreciseError)?;
        }
        Ok(pending.to_u64()?)
    }
    /// get total reward amount for a user so far
    pub fn get_new_reward_debt_dual(&self, user_info:&UserInfo) -> Result<u64>{
        let deposit_balance = user_info.deposit_balance.to_precise()?;
        let reward_per_share_net = self.reward_per_share_net_dual.to_precise()?;
        let reward_multipler = REWARD_MULTIPLER.to_precise()?;

        let result = deposit_balance.checked_mul(&reward_per_share_net).ok_or(FarmError::PreciseError)?
                    .checked_div(&reward_multipler).ok_or(FarmError::PreciseError)?;
                    
        Ok(result.to_u64()?)
    }
    pub fn get_version(&self)->u8 {
        self.version
    }
    pub fn set_version(&mut self, ver: u8) {
        self.version = ver;
    }
    pub fn is_allowed(&self) -> bool {
        self.state != FarmState::tou8(FarmState::NotAllowed)
    }
    pub fn set_state(&mut self, new_state: FarmState) {
        self.state = FarmState::tou8(new_state);
    }
    pub fn get_state(&self) -> FarmState {
        FarmState::fromu8(self.state)
    }
    pub fn update_share(&mut self, cur_timestamp: u64, param_lp_balance: u64, param_reward_balance: u64) -> ProgramResult{
        let mut _calc_timestamp = cur_timestamp;
        let end_timestamp = self.end_timestamp;
        if cur_timestamp > end_timestamp {
            _calc_timestamp = end_timestamp;
        }
        let last_timestamp = self.last_timestamp;
        
        self.last_timestamp = _calc_timestamp;

        if param_lp_balance == 0 {
            msg!("Error: param_lp_balance == 0");
            return Ok(());
        }
        
        if self.distributed_rewards < self.harvested_rewards {
            msg!("Error: self.distributed_rewards < self.harvested_rewards");
            return Err(FarmError::RewardOverflow.into());
        }
        if param_reward_balance < self.distributed_rewards - self.harvested_rewards {
            msg!("Error: param_reward_balance < self.distributed_rewards - self.harvested_rewards");
            return Err(FarmError::RewardOverflow.into());
        }
        if end_timestamp < last_timestamp {
            msg!("Error: self.end_timestamp < last_timestamp");
            return Err(FarmError::TimeOverflow.into());
        }

        let remained_farm_duration = (end_timestamp - last_timestamp).to_precise()?;
        let reward_balance = (param_reward_balance - self.distributed_rewards + self.harvested_rewards).to_precise()?;

        let reward_per_timestamp = reward_balance
                                    .checked_div(&remained_farm_duration).ok_or(FarmError::PreciseError)?;

        let pending_duration = (_calc_timestamp - last_timestamp).to_precise()?;
        let reward_multipler = REWARD_MULTIPLER.to_precise()?;
        let reward_per_share_net = self.reward_per_share_net.to_precise()?;
        let lp_balance = param_lp_balance.to_precise()?;

        let mut reward = pending_duration.checked_mul(&reward_per_timestamp).ok_or(FarmError::PreciseError)?;

        if reward.to_imprecise().ok_or(FarmError::PreciseError)? > reward_balance.to_imprecise().ok_or(FarmError::PreciseError)? {
            reward = reward_balance;
        }
        
        self.distributed_rewards += reward.to_u64()?;

        let updated_share = reward_multipler.checked_mul(&reward).ok_or(FarmError::PreciseError)?
                            .checked_div(&lp_balance).ok_or(FarmError::PreciseError)?
                            .checked_add(&reward_per_share_net).ok_or(FarmError::PreciseError)?;

        self.reward_per_share_net = updated_share.to_imprecise().ok_or(FarmError::PreciseError)?;
        Ok(())
    }
    pub fn update_share_dual(&mut self, cur_timestamp: u64, param_lp_balance: u64, param_reward_balance: u64) -> ProgramResult{
        self.assert_dual_yield()?;

        let mut _calc_timestamp = cur_timestamp;
        let end_timestamp = self.end_timestamp_dual;
        if cur_timestamp > end_timestamp {
            _calc_timestamp = end_timestamp;
        }
        let last_timestamp = self.last_timestamp_dual;
        
        self.last_timestamp_dual = _calc_timestamp;

        if param_lp_balance == 0 {
            msg!("Error: param_lp_balance == 0");
            return Ok(());
        }

        if self.distributed_rewards_dual < self.harvested_rewards_dual {
            msg!("Error: self.distributed_rewards < self.harvested_rewards");
            return Err(FarmError::RewardOverflow.into());
        }
        if param_reward_balance < self.distributed_rewards_dual - self.harvested_rewards_dual {
            msg!("Error: param_reward_balance < self.distributed_rewards - self.harvested_rewards");
            return Err(FarmError::RewardOverflow.into());
        }
        if end_timestamp < last_timestamp {
            msg!("Error: self.end_timestamp < last_timestamp");
            return Err(FarmError::TimeOverflow.into());
        }

        let remained_farm_duration = (end_timestamp - last_timestamp).to_precise()?;
        let reward_balance = (param_reward_balance - self.distributed_rewards_dual + self.harvested_rewards_dual).to_precise()?;

        let reward_per_timestamp = reward_balance
                                    .checked_div(&remained_farm_duration).ok_or(FarmError::PreciseError)?;

        let pending_duration = (_calc_timestamp - last_timestamp).to_precise()?;
        let reward_multipler = REWARD_MULTIPLER.to_precise()?;
        let reward_per_share_net = self.reward_per_share_net_dual.to_precise()?;
        let lp_balance = param_lp_balance.to_precise()?;

        let mut reward = pending_duration.checked_mul(&reward_per_timestamp).ok_or(FarmError::PreciseError)?;

        if reward.to_imprecise().ok_or(FarmError::PreciseError)? > reward_balance.to_imprecise().ok_or(FarmError::PreciseError)? {
            reward = reward_balance;
        }
        
        self.distributed_rewards_dual += reward.to_u64()?;

        let updated_share = reward_multipler.checked_mul(&reward).ok_or(FarmError::PreciseError)?
                            .checked_div(&lp_balance).ok_or(FarmError::PreciseError)?
                            .checked_add(&reward_per_share_net).ok_or(FarmError::PreciseError)?;
        self.reward_per_share_net_dual = updated_share.to_imprecise().ok_or(FarmError::PreciseError)?;

        Ok(())
    }
        
    pub fn assert_dual_yield(&self) -> ProgramResult {
        if self.get_state() != FarmState::DualYield {
            return Err(FarmError::NotAllowed.into());
        }
        Ok(())
    }

    pub fn assert_single_yield(&self) -> ProgramResult {
        if self.get_state() != FarmState::SingleYield {
            return Err(FarmError::NotAllowed.into());
        }
        Ok(())
    }
    pub fn assert_allowed(&self) -> ProgramResult {
        if !self.is_allowed() {
            return Err(FarmError::NotAllowed.into());
        }
        Ok(())
    }
    pub fn assert_not_allowed(&self) -> ProgramResult {
        if self.is_allowed() {
            return Err(FarmError::NotAllowed.into());
        }
        Ok(())
    }
    pub fn removal_reward_dual_amount(&self, pool_amount: u64,amount: u64) -> Result<u64> {
        if pool_amount <= self.distributed_rewards_dual - self.harvested_rewards_dual {
            return Err(FarmError::NotEnoughBalance.into());
        }
        let removal_reward_amount = pool_amount - (self.distributed_rewards_dual - self.harvested_rewards_dual);
        if removal_reward_amount >= amount {
            return Ok(amount);
        }
        Ok(removal_reward_amount)
    }
    pub fn update<'info>(&mut self, user_info:&mut ProgramAccount<'info, UserInfo>, cur_timestamp: u64) -> ProgramResult {
        self.update_share(cur_timestamp, self.pool_lp_balance, self.current_rewards)?;
        self.update_share_dual(cur_timestamp, self.pool_lp_balance, self.current_rewards_dual)?;

        if user_info.deposit_balance > 0 {
            let pending = self.pending_rewards(user_info)?;
            let pending_dual = self.pending_rewards_dual(user_info)?;
            user_info.pending_rewards += pending;
            user_info.pending_rewards_dual += pending_dual;
        }
        Ok(())
    }
    

}

/// User information struct
#[account]
#[derive(Default)]
pub struct UserInfo {
    pub wallet: Pubkey,
    pub farm_id: Pubkey,
    pub deposit_balance: u64,

    pub pending_rewards: u64,
    pub reward_debt: u64,

    pub pending_rewards_dual: u64,
    pub reward_debt_dual: u64,

    /// reserve
    pub reserve1: Pubkey,
    pub reserve2: Pubkey,
    pub reserve3: Pubkey,
}

/// Define valid farm states.
#[derive(Clone, PartialEq)]
pub enum FarmState {
    NotAllowed,
    SingleYield,
    DualYield
}
impl FarmState {
    pub fn not_allowed() -> Self {
        FarmState::NotAllowed
    }
    pub fn single() -> Self {
        FarmState::SingleYield
    }
    pub fn dual() -> Self {
        FarmState::DualYield
    }
    pub fn tou8(state:FarmState) -> u8 {
        state as u8
    }
    pub fn fromu8(num: u8) -> FarmState {
        match num {
            0 => {FarmState::NotAllowed}
            1 => {FarmState::SingleYield}
            2 => {FarmState::DualYield}
            _ => {FarmState::NotAllowed}
        }
    }
}


/// Define valid farm states.
#[derive(Clone, PartialEq)]
pub enum RewardType {
    SingleReward,
    DualReward
}
impl RewardType {
    pub fn single() -> Self {
        RewardType::SingleReward
    }
    pub fn dual() -> Self {
        RewardType::DualReward
    }
    pub fn tou8(state:RewardType) -> u8 {
        state as u8
    }
    pub fn fromu8(num: u8) -> RewardType {
        match num {
            0 => {RewardType::SingleReward}
            1 => {RewardType::DualReward}
            _ => {RewardType::SingleReward}
        }
    }
    pub fn is_single(num: u8) -> bool {
        (RewardType::SingleReward as u8) == num
    }
    pub fn is_dual(num: u8) -> bool {
        (RewardType::DualReward as u8) == num
    }
}
