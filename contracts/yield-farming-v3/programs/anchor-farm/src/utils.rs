use anchor_lang::prelude::*;
use crate::{
    constant::*,
    error::*,
};
use std::convert::TryFrom;
use spl_math::{precise_number::PreciseNumber};
use std::str::FromStr;

pub trait ToPrecise {
    fn to_precise(&self)-> Result<PreciseNumber>;
}

pub trait ToU64U128 {
    fn to_u64(&self) -> Result<u64>;
    fn to_u128(&self) -> Result<u128>;
}

impl ToPrecise for u64 {
    fn to_precise(&self)-> Result<PreciseNumber> {
        Ok(PreciseNumber::new(*self as u128).ok_or(FarmError::PreciseError)?)
    }
}

impl ToPrecise for u128 {
    fn to_precise(&self)-> Result<PreciseNumber> {
        Ok(PreciseNumber::new(*self).ok_or(FarmError::PreciseError)?)
    }
}
impl ToU64U128 for PreciseNumber {
    fn to_u64(&self) -> Result<u64> {
        Ok(u64::try_from(self.to_imprecise().ok_or(FarmError::PreciseError)?).unwrap_or(0))
    }
    fn to_u128(&self) -> Result<u128> {
        Ok(self.to_imprecise().ok_or(FarmError::PreciseError)?)
    }
}

pub fn is_zero_account(account_info:&AccountInfo)->bool{
    let account_data: &[u8] = &account_info.data.borrow();
    let len = account_data.len();
    let mut is_zero = true;
    for i in 0..len-1 {
        if account_data[i] != 0 {
            is_zero = false;
        }
    }
    is_zero
}

pub fn assert_owner(cur_owner: Pubkey, given_owner: Pubkey) -> ProgramResult {
    if cur_owner != given_owner {
        return Err(FarmError::InvalidOwner.into());
    }
    Ok(())
}
pub fn check_locked_farm(token_a_mint:&Pubkey, token_b_mint:&Pubkey)->Result<bool> {
    let mut result = false;
    let sol_mint = Pubkey::from_str(SOL_MINT_ADDRESS).map_err(|_| FarmError::InvalidPubkey)?;
    let eth_mint = Pubkey::from_str(ETH_MINT_ADDRESS).map_err(|_| FarmError::InvalidPubkey)?;
    let crp_mint = Pubkey::from_str(CRP_MINT_ADDRESS).map_err(|_| FarmError::InvalidPubkey)?;
    let usdc_mint = Pubkey::from_str(USDC_MINT_ADDRESS).map_err(|_| FarmError::InvalidPubkey)?;
    let usdt_mint = Pubkey::from_str(USDT_MINT_ADDRESS).map_err(|_| FarmError::InvalidPubkey)?;

    if  (*token_a_mint == sol_mint && *token_b_mint == usdc_mint) ||
        (*token_a_mint == usdc_mint && *token_b_mint == sol_mint) || //SOL-USDC
        (*token_a_mint == sol_mint && *token_b_mint == usdt_mint) ||
        (*token_a_mint == usdt_mint && *token_b_mint == sol_mint) || //SOL-USDT
        (*token_a_mint == eth_mint && *token_b_mint == usdc_mint) ||
        (*token_a_mint == usdc_mint && *token_b_mint == eth_mint) || //ETH-USDC
        (*token_a_mint == eth_mint && *token_b_mint == usdt_mint) ||
        (*token_a_mint == usdt_mint && *token_b_mint == eth_mint) || //ETH-USDT
        (*token_a_mint == usdc_mint && *token_b_mint == crp_mint) ||
        (*token_a_mint == crp_mint && *token_b_mint == usdc_mint) || //CRP-USDC
        (*token_a_mint == usdt_mint && *token_b_mint == crp_mint) ||
        (*token_a_mint == crp_mint && *token_b_mint == usdt_mint) || //CRP-USDT
        (*token_a_mint == sol_mint && *token_b_mint == crp_mint) ||
        (*token_a_mint == crp_mint && *token_b_mint == sol_mint) ||  //SOL-CRP
        (*token_a_mint == eth_mint && *token_b_mint == crp_mint) ||
        (*token_a_mint == crp_mint && *token_b_mint == eth_mint) ||  //ETH-CRP
        (*token_a_mint == eth_mint && *token_b_mint == sol_mint) || 
        (*token_a_mint == sol_mint && *token_b_mint == eth_mint)     //SOL-ETH
    {
        result = true
    }
    Ok(result)
}

pub fn assert_locked_farm(token_a_mint:&Pubkey, token_b_mint:&Pubkey, creator: &Pubkey, allowed_creator: &Pubkey) -> ProgramResult {
    let is_locked_farm = check_locked_farm(token_a_mint, token_b_mint)?;
    if is_locked_farm {
        if creator == allowed_creator {
            return Err(FarmError::NotAllowed.into());
        }
    }
    Ok(())
}
pub fn check_allowed(token_a_mint:&Pubkey, token_b_mint:&Pubkey)->Result<bool> {
    let mut is_allowed = false;
    if  *token_a_mint == Pubkey::from_str(CRP_MINT_ADDRESS).map_err(|_| FarmError::InvalidPubkey)?  ||
        *token_b_mint == Pubkey::from_str(CRP_MINT_ADDRESS).map_err(|_| FarmError::InvalidPubkey)?  ||
        *token_a_mint == Pubkey::from_str(USDC_MINT_ADDRESS).map_err(|_| FarmError::InvalidPubkey)? || 
        *token_b_mint == Pubkey::from_str(USDC_MINT_ADDRESS).map_err(|_| FarmError::InvalidPubkey)? ||
        *token_a_mint == Pubkey::from_str(USDT_MINT_ADDRESS).map_err(|_| FarmError::InvalidPubkey)? || 
        *token_b_mint == Pubkey::from_str(USDT_MINT_ADDRESS).map_err(|_| FarmError::InvalidPubkey)?
    {
        is_allowed = true;
    }
    Ok(is_allowed)
}

pub fn assert_farm_period(cur_timestamp: u64, start_timestamp: u64, end_timestamp: u64) -> ProgramResult {
    // farm account - This farm was not started yet
    if cur_timestamp < start_timestamp {
        return Err(FarmError::NotStarted.into());
    }

    // farm account - The period of this farm was ended
    if cur_timestamp > end_timestamp {
        return Err(FarmError::FarmEnded.into());
    }
    Ok(())
}

pub fn assert_farm_started(cur_timestamp: u64, start_timestamp: u64) -> ProgramResult {
    // farm account - This farm was not started yet
    if cur_timestamp < start_timestamp {
        return Err(FarmError::NotStarted.into());
    }
    Ok(())
}
pub fn get_real_amount_to_deposit(user_wallet_amount: u64, with_swap_action: u8, amount: u64) -> Result<u64> {
    if user_wallet_amount < amount {
        return Err(FarmError::NotEnoughBalance.into());
    }
    let mut result = amount;
    if with_swap_action > 0 {
        result = user_wallet_amount - amount;
    }
    Ok(result)
}

pub fn get_real_amount_to_withdraw(user_deposit_balance: u64, with_swap_action: u8, amount: u64) -> Result<u64> {
    if user_deposit_balance == 0 {
        return Err(FarmError::NotEnoughBalance.into());
    }
    let mut result = amount;
    if with_swap_action > 0 {
        if user_deposit_balance < amount {
            result = user_deposit_balance;
        }
    }
    else if user_deposit_balance < result {
        return Err(FarmError::NotEnoughBalance.into());
    }
    
    Ok(result)
}


pub fn assert_true(flag: bool) -> ProgramResult {
    if !flag {
        return Err(FarmError::NotAllowed.into());
    }
    Ok(())
}
pub fn assert_pda(seeds:&[&[u8]], program_id: &Pubkey, goal_key: &Pubkey) -> ProgramResult {
    let (found_key, _bump) = Pubkey::find_program_address(seeds, program_id);
    if found_key != *goal_key {
        return Err(FarmError::InvalidProgramAddress.into());
    }
    Ok(())
}