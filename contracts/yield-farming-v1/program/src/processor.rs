//! Program state processor
//! In here, All instructions are processed by Processor

use {
    crate::{
        error::FarmPoolError,
        instruction::{FarmPoolInstruction},
        state::{FarmPool,UserInfo},
        constant::*,
    },
    borsh::{BorshDeserialize, BorshSerialize},
    num_traits::FromPrimitive,
    solana_program::{
        account_info::{
            next_account_info,
            AccountInfo,
        },
        borsh::try_from_slice_unchecked,
        decode_error::DecodeError,
        entrypoint::ProgramResult,
        msg,
        program::{ invoke_signed},
        program_error::PrintProgramError,
        program_error::ProgramError,
        pubkey::Pubkey,
        clock::Clock,
        sysvar::Sysvar,
        program_pack::Pack,
    },
    spl_token::state::{Mint, Account}, 
};
use std::str::FromStr;

// useful amm program's state
use cropper_liquidity_pool::amm_stats::{SwapVersion};

/// Program state handler.
/// Main logic of this program
pub struct Processor {}
impl Processor {  
    /// All instructions start from here and are processed by their type.
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = FarmPoolInstruction::try_from_slice(input)?;

        // determine instruction type
        match instruction {
            FarmPoolInstruction::Initialize{
                nonce,
                start_timestamp,
                end_timestamp
            } => {
                // Instruction: Initialize
                Self::process_initialize(program_id, accounts, nonce, start_timestamp, end_timestamp)
            }
            FarmPoolInstruction::Deposit(amount) => {
                // Instruction: Deposit
                Self::process_deposit(program_id, accounts, amount)
            }
            FarmPoolInstruction::Withdraw(amount) => {
                // Instruction: Withdraw
                Self::process_withdraw(program_id, accounts, amount)
            }
            FarmPoolInstruction::AddReward(amount) => {
                // Instruction: AddReward
                Self::process_add_reward(program_id, accounts, amount)
            }
            FarmPoolInstruction::PayFarmFee(amount) => {
                // Instruction: PayFarmFee
                Self::process_pay_farm_fee(program_id, accounts, amount)
            }
        }
    }

    /// process `Initialize` instruction.
    pub fn process_initialize(
        program_id: &Pubkey,        // this program id
        accounts: &[AccountInfo],   // all account informations
        nonce: u8,                  // nonce for authorizing
        start_timestamp: u64,       // start time of this farm
        end_timestamp: u64,         // end time of this farm
    ) -> ProgramResult {
        // start initializeing this farm pool ...

        // get all account informations from accounts array by using iterator
        let account_info_iter = &mut accounts.iter();
        
        // farm pool account info to create newly
        let farm_id_info = next_account_info(account_info_iter)?;

        // authority of farm pool account
        let authority_info = next_account_info(account_info_iter)?;

        // creator wallet account information
        let creator_info = next_account_info(account_info_iter)?;

        // lp token account information to store lp token in the pool
        let pool_lp_token_account_info = next_account_info(account_info_iter)?;

        // reward token account information to store reward token in the pool
        let pool_reward_token_account_info = next_account_info(account_info_iter)?;

        // lp token's mint account information
        let pool_mint_info = next_account_info(account_info_iter)?;

        // reward token's mint account information
        let reward_mint_info = next_account_info(account_info_iter)?;

        // amm account information what have lp token mint, token_a mint, token_b mint
        let amm_id_info = next_account_info(account_info_iter)?;

        // check if this farm account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != Self::authority_id(program_id, farm_id_info.key, nonce)? {
            return Err(FarmPoolError::InvalidProgramAddress.into());
        }

        // check if farm creator is signer of this transaction
        // if not, returns SignatureMissing error
        if !creator_info.is_signer {
            return Err(FarmPoolError::SignatureMissing.into());
        }

        // check if given farm was initialized already
        if !Self::is_zero_account(farm_id_info) {
            return Err(FarmPoolError::AlreadyInUse.into());
        }

        // check if end time is later than start time
        if end_timestamp <= start_timestamp {
            return Err(FarmPoolError::WrongPeriod.into());
        }

        let token_program_pubkey = Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap();

        // token account - check if owner is saved token program
        if  *pool_lp_token_account_info.owner != token_program_pubkey ||
            *pool_reward_token_account_info.owner != token_program_pubkey {
                return Err(FarmPoolError::InvalidOwner.into());
        }

        let pool_lp_token_data = Account::unpack_from_slice(&pool_lp_token_account_info.data.borrow())?;
        let pool_reward_token_data = Account::unpack_from_slice(&pool_reward_token_account_info.data.borrow())?;

        // token account - check if user token's owner is depositor
        if  pool_lp_token_data.owner != *authority_info.key ||
            pool_reward_token_data.owner != *authority_info.key {
            return Err(FarmPoolError::InvalidOwner.into());
        }

        // token account - check if token mint is correct
        if  pool_lp_token_data.mint != *pool_mint_info.key ||
            pool_reward_token_data.mint != *reward_mint_info.key {
            return Err(FarmPoolError::WrongPoolMint.into());
        }

        // borrow farm account data to initialize (mutable)
        let mut farm_pool = try_from_slice_unchecked::<FarmPool>(&farm_id_info.data.borrow())?;

        let amm_program_id = Pubkey::from_str(AMM_PROGRAM_ID).unwrap();

        // check if given amm id is for correct amm program id
        if *amm_id_info.owner != amm_program_id {
            return Err(FarmPoolError::InvalidProgramAddress.into());
        }
        // borrow amm account data to check token's mint address with inputed one (immutable)
        let amm_swap = SwapVersion::unpack(&amm_id_info.data.borrow())?;
        
        // check if lp token mint address is same with amm pool's lp token mint address
        // if not, returns WrongPoolMint error
        if *amm_swap.pool_mint() != *pool_mint_info.key {
            return Err(FarmPoolError::WrongPoolMint.into());
        }

        // check if token pairing is CRP Pair
        
        // CRP token pairing flag
        let mut crp_token_pairing = 0;

        // CRP token mint address
        let crp_pubkey = Pubkey::from_str(CRP_MINT_ADDRESS).unwrap();

        // other token mint address to check token pairing
        let mut other_pubkey = *amm_swap.token_a_mint();

        if *amm_swap.token_a_mint() == crp_pubkey {
            // this is crp token pair
            crp_token_pairing = 1;
            other_pubkey = *amm_swap.token_b_mint();
        }
        
        if *amm_swap.token_b_mint() == crp_pubkey {
            // this is crp token pair
            crp_token_pairing = 1;
        }

        // check if this creator can create "locked farms" specified by site owner
        if crp_token_pairing == 1 {
            if  other_pubkey == Pubkey::from_str(USDC_MINT_ADDRESS).unwrap() ||
                other_pubkey == Pubkey::from_str(USDT_MINT_ADDRESS).unwrap() ||
                other_pubkey == Pubkey::from_str(SOL_MINT_ADDRESS).unwrap() ||
                other_pubkey == Pubkey::from_str(ETH_MINT_ADDRESS).unwrap() {

                    // check if creator is allowed creator
                    // if not returns WrongCreator error
                    if *creator_info.key != Pubkey::from_str(ALLOWED_CREATOR).unwrap() {
                        return Err(FarmPoolError::WrongCreator.into());
                    }
                }
        }

        // Initialize farm account data
        
        // if not CRP token pairing,this farm is not allowed until creator pays farm fee
        farm_pool.is_allowed = crp_token_pairing;

        // owner of this farm - creator
        farm_pool.owner = *creator_info.key;

        // initialize fee owner with predefined wallet address
        farm_pool.fee_owner = Pubkey::from_str(FEE_OWNER).unwrap();

        // initialize lp token account to store lp token
        farm_pool.pool_lp_token_account = *pool_lp_token_account_info.key;

        // initialize reward token account to store reward token
        farm_pool.pool_reward_token_account = *pool_reward_token_account_info.key;

        // store nonce to authorize this farm account
        farm_pool.nonce = nonce;

        // store lp token mint address
        farm_pool.pool_mint_address = *pool_mint_info.key;

        // store spl-token program address
        farm_pool.token_program_id = token_program_pubkey;

        // store reward token mint address
        farm_pool.reward_mint_address = *reward_mint_info.key;

        // initialize total reward for unit lp so far
        farm_pool.reward_per_share_net = 0;

        // initialize lastest reward time
        farm_pool.last_timestamp = start_timestamp;

        // store reward per second
        farm_pool.reward_per_timestamp = 0;

        // store start time of this farm
        farm_pool.start_timestamp = start_timestamp;

        // store end time of this farm
        farm_pool.end_timestamp = end_timestamp;
        
        // serialize/store this initialized farm again
        farm_pool
            .serialize(&mut *farm_id_info.data.borrow_mut())
            .map_err(|e| e.into())
    } 

    /// process deposit instruction
    /// this function performs stake lp token, harvest reward token
    pub fn process_deposit(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        // get account informations
        let account_info_iter = &mut accounts.iter();

        // farm account information to stake/harvest
        let farm_id_info = next_account_info(account_info_iter)?;

        // authority information of this farm account
        let authority_info = next_account_info(account_info_iter)?;

        // depositor's wallet account information
        let depositor_info = next_account_info(account_info_iter)?;

        // depositor's user account information to include deposited balance, reward debt
        let user_info_account_info = next_account_info(account_info_iter)?;

        // lp token account information in the depositor's wallet
        let user_lp_token_account_info = next_account_info(account_info_iter)?;

        // lp token account information in the farm pool
        let pool_lp_token_account_info = next_account_info(account_info_iter)?;

        // reward token account information in the depositor's wallet
        let user_reward_token_account_info = next_account_info(account_info_iter)?;

        // reward token account information in the farm pool
        let pool_reward_token_account_info = next_account_info(account_info_iter)?;

        // lp token mint account information in the farm pool
        let pool_lp_mint_info = next_account_info(account_info_iter)?;

        // fee owner wallet account information to collect fees such as harvest fee
        let fee_owner_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        // clock account information to use timestamp
        let clock_sysvar_info = next_account_info(account_info_iter)?;

        // get clock from clock sysvar account information
        let clock = &Clock::from_account_info(clock_sysvar_info)?;

        // get current timestamp(second)
        let cur_timestamp: u64 = clock.unix_timestamp as u64;

        // borrow farm pool account data
        let mut farm_pool = try_from_slice_unchecked::<FarmPool>(&farm_id_info.data.borrow())?;
        
        // borrow user info for this pool
        let mut user_info = try_from_slice_unchecked::<UserInfo>(&user_info_account_info.data.borrow())?;

        //singers - check if depositor is signer
        if !depositor_info.is_signer {
            return Err(FarmPoolError::InvalidSigner.into());
        }

        // farm account - check if the given program address and farm account are correct
        if *authority_info.key != Self::authority_id(program_id, farm_id_info.key, farm_pool.nonce)? {
            return Err(FarmPoolError::InvalidProgramAddress.into());
        }

        // farm account - check if this farm was allowed already
        if farm_pool.is_allowed == 0 {
            return Err(FarmPoolError::NotAllowed.into());
        }

        // farm account - check fee owner
        if farm_pool.fee_owner != *fee_owner_info.key {
            return Err(FarmPoolError::InvalidFeeAccount.into());
        }

        // farm account - This farm was not started yet
        if cur_timestamp < farm_pool.start_timestamp {
            return Err(FarmPoolError::NotStarted.into());
        }

        // farm account - The period of this farm was ended
        if cur_timestamp > farm_pool.end_timestamp {
            return Err(FarmPoolError::FarmEnded.into());
        }

        // user info account - check if user info account's owner is program id
        if user_info_account_info.owner != program_id {
            return Err(FarmPoolError::InvalidOwner.into());
        }

        // user info account - check if this depositor is new user
        if Self::is_zero_account(user_info_account_info) {
            // save user's wallet address
            user_info.wallet = *depositor_info.key;

            // save user's farm account address
            user_info.farm_id = *farm_id_info.key;
        }

        // user info account - check if this is for given farm account
        if user_info.farm_id != *farm_id_info.key {
            return Err(FarmPoolError::InvalidOwner.into());
        }

        // user info account - check if this user info is for depositor
        if user_info.wallet != *depositor_info.key {
            return Err(FarmPoolError::InvalidOwner.into());
        }

        // token account - check if owner is saved token program
        if  *user_lp_token_account_info.owner != farm_pool.token_program_id ||
            *pool_lp_token_account_info.owner != farm_pool.token_program_id ||
            *user_reward_token_account_info.owner != farm_pool.token_program_id ||
            *pool_reward_token_account_info.owner != farm_pool.token_program_id {
                return Err(FarmPoolError::InvalidOwner.into());
        }

        // token account - check if pool lp token account & pool reward token account is for given farm account
        if  farm_pool.pool_lp_token_account != *pool_lp_token_account_info.key ||
            farm_pool.pool_reward_token_account != *pool_reward_token_account_info.key{
                return Err(FarmPoolError::InvalidOwner.into());
        }

        let user_lp_token_data = Account::unpack_from_slice(&user_lp_token_account_info.data.borrow())?;
        let pool_lp_token_data = Account::unpack_from_slice(&pool_lp_token_account_info.data.borrow())?;
        let user_reward_token_data = Account::unpack_from_slice(&user_reward_token_account_info.data.borrow())?;
        let pool_reward_token_data = Account::unpack_from_slice(&pool_reward_token_account_info.data.borrow())?;

        // token account - check if user token's owner is depositor
        if  user_lp_token_data.owner != *depositor_info.key ||
            user_reward_token_data.owner != *depositor_info.key ||
            pool_lp_token_data.owner != *authority_info.key ||
            pool_reward_token_data.owner != *authority_info.key {
            return Err(FarmPoolError::InvalidOwner.into());
        }

        // token account - check if user has enough token amount
        if user_lp_token_data.amount < amount {
            return Err(FarmPoolError::NotEnoughBalance.into());
        }

        // pool mint - check if pool mint is current program's mint address
        if *pool_lp_mint_info.key != farm_pool.pool_mint_address {
            return Err(FarmPoolError::WrongPoolMint.into());
        }

        // token program - check if given token program is correct
        if *token_program_info.key != farm_pool.token_program_id {
            return Err(FarmPoolError::InvalidProgramAddress.into());
        }

        // borrow lp token mint account data
        let pool_mint = Mint::unpack_from_slice(&pool_lp_mint_info.data.borrow())?; 

        //update this pool with up-to-date, distribute reward token 
        Self::update_pool(
            &mut farm_pool,
            cur_timestamp,
            pool_mint.supply,
        );

        // harvest user's pending rewards
        if user_info.deposit_balance > 0 {

            // pending amount
            let pending: u64 = farm_pool.pending_rewards(&mut user_info);
            
            if pending > 0 {
                // harvest fee for the pending reward
                let harvest_fee = pending * HARVEST_FEE_NUMERATOR / HARVEST_FEE_DENOMINATOR; 
                
                // transfer harvest fee to fee owner
                Self::token_transfer(
                    farm_id_info.key,
                    token_program_info.clone(), 
                    pool_reward_token_account_info.clone(), 
                    fee_owner_info.clone(), 
                    authority_info.clone(), 
                    farm_pool.nonce, 
                    harvest_fee
                )?;

                // user's real pending amount
                let _pending = pending - harvest_fee;

                // transfer pending reward amount from reward pool to user reward token account
                Self::token_transfer(
                    farm_id_info.key,
                    token_program_info.clone(), 
                    pool_reward_token_account_info.clone(), 
                    user_reward_token_account_info.clone(), 
                    authority_info.clone(), 
                    farm_pool.nonce, 
                    _pending
                )?;
            }
        }

        // deposit (stake lp token)
        if amount > 0 {
            // transfer lp token amount from user's lp token account to pool's lp token pool
            Self::token_transfer(
                farm_id_info.key,
                token_program_info.clone(), 
                user_lp_token_account_info.clone(), 
                pool_lp_token_account_info.clone(), 
                depositor_info.clone(), 
                farm_pool.nonce, 
                amount
            )?;

            // update user's deposited balance
            user_info.deposit_balance += amount;
        }
        
        // update user's reward debt
        user_info.reward_debt = farm_pool.get_new_reward_debt(&user_info);

        // save user's new info to network
        user_info
            .serialize(&mut *user_info_account_info.data.borrow_mut())?;

        // save new farm account data to network
        farm_pool
            .serialize(&mut *farm_id_info.data.borrow_mut())
            .map_err(|e| e.into())
        
    }

    /// process withdraw
    pub fn process_withdraw(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {

        // get account informations
        let account_info_iter = &mut accounts.iter();

        // farm account information to unstake/harvest
        let farm_id_info = next_account_info(account_info_iter)?;

        // authority information of this farm account
        let authority_info = next_account_info(account_info_iter)?;

        // withdrawer's wallet account information
        let withdrawer_info = next_account_info(account_info_iter)?;

        // withdrawer's user account information to include deposited balance, reward debt
        let user_info_account_info = next_account_info(account_info_iter)?;

        // lp token account information in the withdrawer's wallet
        let user_lp_token_account_info = next_account_info(account_info_iter)?;

        // lp token account information in the farm pool
        let pool_lp_token_account_info = next_account_info(account_info_iter)?;

        // reward token account information in the withdrawer's wallet
        let user_reward_token_account_info = next_account_info(account_info_iter)?;

        // reward token account information in the farm pool
        let pool_reward_token_account_info = next_account_info(account_info_iter)?;

        // lp token mint account information in the farm pool
        let pool_lp_mint_info = next_account_info(account_info_iter)?;

        // fee owner wallet account information to collect fees such as harvest fee
        let fee_owner_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        // clock account information to use timestamp
        let clock_sysvar_info = next_account_info(account_info_iter)?;

        // get clock from clock sysvar account information
        let clock = &Clock::from_account_info(clock_sysvar_info)?;

        // get current timestamp(second)
        let cur_timestamp: u64 = clock.unix_timestamp as u64;

        // borrow farm pool account data
        let mut farm_pool = try_from_slice_unchecked::<FarmPool>(&farm_id_info.data.borrow())?;

        // borrow user info for this pool
        let mut user_info = try_from_slice_unchecked::<UserInfo>(&user_info_account_info.data.borrow())?;

        //singers - check if depositor is signer
        if !withdrawer_info.is_signer {
            return Err(FarmPoolError::InvalidSigner.into());
        }

        // farm account - check if the given program address and farm account are correct
        if *authority_info.key != Self::authority_id(program_id, farm_id_info.key, farm_pool.nonce)? {
            return Err(FarmPoolError::InvalidProgramAddress.into());
        }

        // farm account - check if this farm was allowed already
        if farm_pool.is_allowed == 0 {
            return Err(FarmPoolError::NotAllowed.into());
        }

        // farm account - check fee owner
        if farm_pool.fee_owner != *fee_owner_info.key {
            return Err(FarmPoolError::InvalidFeeAccount.into());
        }

        // farm account - This farm was not started yet
        if cur_timestamp < farm_pool.start_timestamp {
            return Err(FarmPoolError::NotStarted.into());
        }

        // user info account - check if user info account's owner is program id
        if user_info_account_info.owner != program_id {
            return Err(FarmPoolError::InvalidOwner.into());
        }

        // user info account - check if this is for given farm account
        if user_info.farm_id != *farm_id_info.key {
            return Err(FarmPoolError::InvalidOwner.into());
        }

        // user info account - check if this user info is for depositor
        if user_info.wallet != *withdrawer_info.key {
            return Err(FarmPoolError::InvalidOwner.into());
        }

        // token account - check if owner is saved token program
        if  *user_lp_token_account_info.owner != farm_pool.token_program_id ||
            *pool_lp_token_account_info.owner != farm_pool.token_program_id ||
            *user_reward_token_account_info.owner != farm_pool.token_program_id ||
            *pool_reward_token_account_info.owner != farm_pool.token_program_id {
                return Err(FarmPoolError::InvalidOwner.into());
        }

        // token account - check if pool lp token account & pool reward token account is for given farm account
        if  farm_pool.pool_lp_token_account != *pool_lp_token_account_info.key ||
            farm_pool.pool_reward_token_account != *pool_reward_token_account_info.key{
                return Err(FarmPoolError::InvalidOwner.into());
        }

        let user_lp_token_data = Account::unpack_from_slice(&user_lp_token_account_info.data.borrow())?;
        let pool_lp_token_data = Account::unpack_from_slice(&pool_lp_token_account_info.data.borrow())?;
        let user_reward_token_data = Account::unpack_from_slice(&user_reward_token_account_info.data.borrow())?;
        let pool_reward_token_data = Account::unpack_from_slice(&pool_reward_token_account_info.data.borrow())?;

        // token account - check if user token's owner is depositor
        if  user_lp_token_data.owner != *withdrawer_info.key ||
            user_reward_token_data.owner != *withdrawer_info.key ||
            pool_lp_token_data.owner != *authority_info.key ||
            pool_reward_token_data.owner != *authority_info.key {
            return Err(FarmPoolError::InvalidOwner.into());
        }

        // token account - check if user has enough token amount
        if pool_lp_token_data.amount < amount {
            return Err(FarmPoolError::NotEnoughBalance.into());
        }

        // pool mint - check if pool mint is current program's mint address
        if *pool_lp_mint_info.key != farm_pool.pool_mint_address {
            return Err(FarmPoolError::WrongPoolMint.into());
        }

        // token program - check if given token program is correct
        if *token_program_info.key != farm_pool.token_program_id {
            return Err(FarmPoolError::InvalidProgramAddress.into());
        }
        
        // if amount > deposited balance, amount is deposited balance
        let mut _amount = amount;
        if user_info.deposit_balance < amount {
            _amount = user_info.deposit_balance;
        }

        // if deposited balance is zero, can't withdraw and returns ZeroDepositBalance error
        if user_info.deposit_balance == 0 {
            return Err(FarmPoolError::ZeroDepositBalance.into());
        }

        //borrow pool lp token mint account data
        let pool_mint = Mint::unpack_from_slice(&pool_lp_mint_info.data.borrow())?;

        //update this pool with up-to-date , distribute reward
        Self::update_pool(
            &mut farm_pool,
            cur_timestamp,
            pool_mint.supply, 
        );

        // harvest user's pending rewards
        if user_info.deposit_balance > 0 {

            // get pending amount
            let pending: u64 = farm_pool.pending_rewards(&mut user_info);

            // harvest
            if pending > 0 {
                // harvest fee
                let harvest_fee = pending * HARVEST_FEE_NUMERATOR / HARVEST_FEE_DENOMINATOR;
                
                // transfer harvest fee to fee owner wallet
                Self::token_transfer(
                    farm_id_info.key,
                    token_program_info.clone(), 
                    pool_reward_token_account_info.clone(), 
                    fee_owner_info.clone(), 
                    authority_info.clone(), 
                    farm_pool.nonce, 
                    harvest_fee
                )?;

                // real pending amount except fee
                let _pending = pending - harvest_fee;

                // transfer real pending amount from reward pool to user reward token account
                Self::token_transfer(
                    farm_id_info.key,
                    token_program_info.clone(), 
                    pool_reward_token_account_info.clone(), 
                    user_reward_token_account_info.clone(), 
                    authority_info.clone(), 
                    farm_pool.nonce, 
                    _pending
                )?;
            }
        }

        // unstake lp token
        if _amount > 0 {
            Self::token_transfer(
                farm_id_info.key,
                token_program_info.clone(), 
                pool_lp_token_account_info.clone(),
                user_lp_token_account_info.clone(), 
                authority_info.clone(), 
                farm_pool.nonce, 
                _amount
            )?;
        }
        
        // store user's wallet address
        user_info.wallet = *withdrawer_info.key;

        // store farm account address
        user_info.farm_id = *farm_id_info.key;

        // update deposited balance
        user_info.deposit_balance -= _amount;

        // update reward debt
        user_info.reward_debt = farm_pool.get_new_reward_debt(&user_info);

        // store user's information to network
        user_info
            .serialize(&mut *user_info_account_info.data.borrow_mut())?;

        // store farm account data to network
        farm_pool
            .serialize(&mut *farm_id_info.data.borrow_mut())
            .map_err(|e| e.into())
        
    }
    /// farm creator can add reward token to his farm
    /// but can't remove once added
    pub fn process_add_reward(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {

        // get account informations
        let account_info_iter = &mut accounts.iter();

        // farm account information to add reward
        let farm_id_info = next_account_info(account_info_iter)?;

        // authority information of this farm account
        let authority_info = next_account_info(account_info_iter)?;

        // creator account information who will add reward
        let creator_info = next_account_info(account_info_iter)?;

        // lp token account information in the creator's wallet
        let user_reward_token_account_info = next_account_info(account_info_iter)?;

        // reward token account information in the farm pool
        let pool_reward_token_account_info = next_account_info(account_info_iter)?;

        // lp token account information in the farm pool
        let pool_lp_mint_info = next_account_info(account_info_iter)?;

        // spl-token program information
        let token_program_info = next_account_info(account_info_iter)?;

        // clock account information to use timestamp
        let clock_sysvar_info = next_account_info(account_info_iter)?;

        // get clock from clock sysvar account information
        let clock = &Clock::from_account_info(clock_sysvar_info)?;

        // get current timestamp(second)
        let cur_timestamp: u64 = clock.unix_timestamp as u64;

        
        // borrow farm pool account data
        let mut farm_pool = try_from_slice_unchecked::<FarmPool>(&farm_id_info.data.borrow())?;

        // check if given creator is farm owner
        // if not, returns WrongManager error
        if *creator_info.key != farm_pool.owner {
            return Err(FarmPoolError::WrongManager.into());
        }

        //singers - check if depositor is signer
        if !creator_info.is_signer {
            return Err(FarmPoolError::InvalidSigner.into());
        }

        // farm account - check if the given program address and farm account are correct
        if *authority_info.key != Self::authority_id(program_id, farm_id_info.key, farm_pool.nonce)? {
            return Err(FarmPoolError::InvalidProgramAddress.into());
        }

        // check if this farm ends
        if cur_timestamp > farm_pool.end_timestamp {
            return Err(FarmPoolError::FarmEnded.into());
        }

        // token account - check if owner is saved token program
        if  *user_reward_token_account_info.owner != farm_pool.token_program_id ||
            *pool_reward_token_account_info.owner != farm_pool.token_program_id {
                return Err(FarmPoolError::InvalidOwner.into());
        }

        // token account - check if pool lp token account & pool reward token account is for given farm account
        if  farm_pool.pool_reward_token_account != *pool_reward_token_account_info.key{
                return Err(FarmPoolError::InvalidOwner.into());
        }

        let user_reward_token_data = Account::unpack_from_slice(&user_reward_token_account_info.data.borrow())?;
        let pool_reward_token_data = Account::unpack_from_slice(&pool_reward_token_account_info.data.borrow())?;

        // token account - check if user token's owner is depositor
        if  user_reward_token_data.owner != *creator_info.key ||
            pool_reward_token_data.owner != *authority_info.key {
            return Err(FarmPoolError::InvalidOwner.into());
        }

        // token account - check if user has enough token amount
        if user_reward_token_data.amount < amount {
            return Err(FarmPoolError::NotEnoughBalance.into());
        }

        // pool mint - check if pool mint is current program's mint address
        if *pool_lp_mint_info.key != farm_pool.pool_mint_address {
            return Err(FarmPoolError::WrongPoolMint.into());
        }

        // token program - check if given token program is correct
        if *token_program_info.key != farm_pool.token_program_id {
            return Err(FarmPoolError::InvalidProgramAddress.into());
        }

        

        // add reward
        if amount > 0 {
            // transfer reward token amount from user's reward token account to pool's reward token account
            Self::token_transfer(
                farm_id_info.key,
                token_program_info.clone(), 
                user_reward_token_account_info.clone(), 
                pool_reward_token_account_info.clone(), 
                creator_info.clone(), 
                farm_pool.nonce, 
                amount
            )?;

            // borrow pool lp token mint account data
            let pool_mint = Mint::unpack_from_slice(&pool_lp_mint_info.data.borrow())?;

            //update this pool with up-to-date
            Self::update_pool(
                &mut farm_pool,
                cur_timestamp,
                pool_mint.supply, 
            );

            // update reward per second in the rest period from now
            let duration = farm_pool.end_timestamp - cur_timestamp;
            let added_reward_per_second = amount / duration;
            farm_pool.reward_per_timestamp += added_reward_per_second;
        }

        // store farm pool account data to network
        farm_pool
            .serialize(&mut *farm_id_info.data.borrow_mut())
            .map_err(|e| e.into())
        
    }
    /// process PayFarmFee instruction
    /// If this farm is not CRP token pairing , farm creator has to pay farm fee
    /// So this farm is allowed to stake/unstake/harvest
    pub fn process_pay_farm_fee(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {

        // get account informations
        let account_info_iter = &mut accounts.iter();

        // farm account information to pay farm fee
        let farm_id_info = next_account_info(account_info_iter)?;

        // authority information of this farm account
        let authority_info = next_account_info(account_info_iter)?;

        // creator account information who will add reward
        let creator_info = next_account_info(account_info_iter)?;

        // USDC token account in the creator's wallet to pay farm fee as USDC stable coin
        let user_usdc_token_account_info = next_account_info(account_info_iter)?;

        // fee owner wallet account to collect all fees
        let fee_owner_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        // borrow farm pool account data
        let mut farm_pool = try_from_slice_unchecked::<FarmPool>(&farm_id_info.data.borrow())?;

        // check if given creator is owner of this farm
        // if not, returns WrongManager error
        if *creator_info.key != farm_pool.owner {
            return Err(FarmPoolError::WrongManager.into());
        }

        // check if given program address and farm account address are correct
        // if not returns InvalidProgramAddress
        if *authority_info.key != Self::authority_id(program_id, farm_id_info.key, farm_pool.nonce)? {
            return Err(FarmPoolError::InvalidProgramAddress.into());
        }

        //singers - check if depositor is signer
        if !creator_info.is_signer {
            return Err(FarmPoolError::InvalidSigner.into());
        }

        // farm account - check fee owner
        if farm_pool.fee_owner != *fee_owner_info.key {
            return Err(FarmPoolError::InvalidFeeAccount.into());
        }

        // token account - check if owner is saved token program
        if  *user_usdc_token_account_info.owner != farm_pool.token_program_id {
                return Err(FarmPoolError::InvalidOwner.into());
        }

        let user_usdc_token_data = Account::unpack_from_slice(&user_usdc_token_account_info.data.borrow())?;

        // token account - check if user token's owner is depositor
        if  user_usdc_token_data.owner != *creator_info.key {
            return Err(FarmPoolError::InvalidOwner.into());
        }

        // token account - check if user has enough token amount
        if user_usdc_token_data.amount < FARM_FEE {
            return Err(FarmPoolError::NotEnoughBalance.into());
        }

        // token program - check if given token program is correct
        if *token_program_info.key != farm_pool.token_program_id {
            return Err(FarmPoolError::InvalidProgramAddress.into());
        }

        // check if amount is same with FARM FEE
        // if not, returns InvalidFarmFee error
        if amount < FARM_FEE {
            return Err(FarmPoolError::InvalidFarmFee.into());
        }

        // transfer fee amount from user's USDC token account to fee owner's account
        Self::token_transfer(
            farm_id_info.key,
            token_program_info.clone(), 
            user_usdc_token_account_info.clone(), 
            fee_owner_info.clone(), 
            creator_info.clone(), 
            farm_pool.nonce, 
            amount
        )?;

        // allow this farm to stake/unstake/harvest
        farm_pool.is_allowed = 1;

        // store farm account data to network
        farm_pool
            .serialize(&mut *farm_id_info.data.borrow_mut())
            .map_err(|e| e.into())
        
    }

    // update pool information with up-to-date, distribute reward token
    pub fn update_pool<'a>(
        farm_pool: &mut FarmPool, 
        cur_timestamp: u64, 
        lp_supply: u64, 
    ){
        // check if valid current timestamp
        if farm_pool.last_timestamp >= cur_timestamp {
            return;
        }
        if lp_supply == 0 || farm_pool.reward_per_timestamp == 0 {
            farm_pool.last_timestamp = cur_timestamp;
            return;
        }

        // update reward per share net and last distributed timestamp
        let multiplier = cur_timestamp - farm_pool.last_timestamp;
        let reward = multiplier * farm_pool.reward_per_timestamp;
        farm_pool.reward_per_share_net = farm_pool.reward_per_share_net + REWARD_MULTIPLER * reward / lp_supply;
        farm_pool.last_timestamp = cur_timestamp;
    }
    /// get authority by given program address.
    pub fn authority_id(
        program_id: &Pubkey,
        my_info: &Pubkey,
        nonce: u8,
    ) -> Result<Pubkey, FarmPoolError> {
        Pubkey::create_program_address(&[&my_info.to_bytes()[..32], &[nonce]], program_id)
            .or(Err(FarmPoolError::InvalidProgramAddress))
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

    /// issue a spl_token `Transfer` instruction.
    pub fn token_transfer<'a>(
        pool: &Pubkey,
        token_program: AccountInfo<'a>,
        source: AccountInfo<'a>,
        destination: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        nonce: u8,
        amount: u64,
    ) -> Result<(), ProgramError> {
        let pool_bytes = pool.to_bytes();
        let authority_signature_seeds = [&pool_bytes[..32], &[nonce]];
        let signers = &[&authority_signature_seeds[..]];
        let ix = spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?;
        invoke_signed(
            &ix,
            &[source, destination, authority, token_program],
            signers,
        )
    } 
    
}

/// implement all farm error messages
impl PrintProgramError for FarmPoolError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            FarmPoolError::AlreadyInUse => msg!("Error: The account cannot be initialized because it is already being used"),
            FarmPoolError::InvalidProgramAddress => msg!("Error: The program address provided doesn't match the value generated by the program"),
            FarmPoolError::InvalidState => msg!("Error: The stake pool state is invalid"),
            FarmPoolError::CalculationFailure => msg!("Error: The calculation failed"),
            FarmPoolError::FeeTooHigh => msg!("Error: Stake pool fee > 1"),
            FarmPoolError::WrongAccountMint => msg!("Error: Token account is associated with the wrong mint"),
            FarmPoolError::WrongManager => msg!("Error: Wrong pool manager account"),
            FarmPoolError::SignatureMissing => msg!("Error: Required signature is missing"),
            FarmPoolError::InvalidValidatorStakeList => msg!("Error: Invalid validator stake list account"),
            FarmPoolError::InvalidFeeAccount => msg!("Error: Invalid manager fee account"),
            FarmPoolError::WrongPoolMint => msg!("Error: Specified pool mint account is wrong"),
            FarmPoolError::NotStarted => msg!("Error: The farm has not started yet"),
            FarmPoolError::FarmEnded => msg!("Error: The farm ended"),
            FarmPoolError::ZeroDepositBalance => msg!("Error: Zero deposit balance"),
            FarmPoolError::NotAllowed => msg!("Error: This farm is not allowed yet. The farm creator has to pay additional fee"),
            FarmPoolError::InvalidFarmFee => msg!("Error: Wrong Farm Fee. Farm fee has to be {}CRP",FARM_FEE),
            FarmPoolError::WrongAmmId => msg!("Error: Wrong Amm Id"),
            FarmPoolError::WrongFarmPool => msg!("Error: Wrong Farm pool"),
            FarmPoolError::WrongCreator => msg!("Error: Not allowed to create the farm by this creator"),
            FarmPoolError::WrongPeriod => msg!("Error: wrong start time and end time"),
            FarmPoolError::InvalidOwner => msg!("Error: invalid owner"),
            FarmPoolError::InvalidSigner => msg!("Error: invalid signer"),
            FarmPoolError::NotEnoughBalance => msg!("Error: Not enough balance"),
        }
    }
} 
