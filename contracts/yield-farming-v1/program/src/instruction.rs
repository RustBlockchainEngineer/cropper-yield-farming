//! All instruction types
//! These instructions represent a function what will be processed by this program

// this allows many arguments for the function parameters
#![allow(clippy::too_many_arguments)]

use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar
    },
};

/// Instructions supported by the FarmPool program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum FarmPoolInstruction {
    ///   Initializes a new FarmPool.
    ///   These represent the parameters that will be included from client side
    ///   [w] - writable, [s] - signer
    /// 
    ///   0. `[w]` New FarmPool account to create.
    ///   1. `[]` authority to initialize this farm pool account
    ///   2. `[s]` Creator/Manager of this farm
    ///   3. `[w]` LP token account of this farm to store lp token
    ///   4. `[w]` reward token account of this farm to store rewards for the farmers
    ///             Creator has to transfer/deposit his reward token to this account.
    ///             only support spl tokens
    ///   5. `[]` Pool token mint address
    ///   6. `[]` Reward token mint address
    ///   7. `[]` Amm Id
    ///   8. `[]` Token program id
    ///   9. `[]` nonce
    ///   10. `[]` Farm program id
    ///   11.'[]' start timestamp. this reflects that the farm starts at this time
    ///   12.'[]' end timestamp. this reflects that the farm ends at this time
    Initialize {
        #[allow(dead_code)]
        /// nonce
        nonce: u8,

        #[allow(dead_code)]
        /// start timestamp
        start_timestamp: u64,

        #[allow(dead_code)]
        /// end timestamp
        end_timestamp: u64,
    },

    ///   Stake Lp tokens to this farm pool
    ///   If amount is zero, only performed "harvest"
    ///   If this farm is not allowed/not started/ended, it fails
    /// 
    ///   0. `[w]` FarmPool to deposit to.
    ///   1. `[]` authority of this farm pool
    ///   2. `[s]` Depositor
    ///   3. `[]` User Farming Information Account
    ///   4. `[w]` User transfer authority.
    ///   5. `[]` User LP token account
    ///   6. `[]` User reward token account
    ///   7. `[]` Pool LP token account
    ///   8. `[]` Pool reward token account
    ///   9. `[]` Pool LP token mint
    ///   10. `[]` Pool reward token mint
    ///   11. `[]` Token program id
    ///   12. `[]` Farm program id
    ///   13. `[]` amount
    Deposit(u64),

    ///   Unstake LP tokens from this farm pool
    ///   Before unstake lp tokens, "harvest" works
    /// 
    ///   0. `[w]` FarmPool to withdraw to.
    ///   1. `[]` authority of this farm pool
    ///   2. `[s]` Withdrawer
    ///   3. `[]` User Farming Information Account
    ///   4. `[w]` User transfer authority.
    ///   5. `[]` User LP token account
    ///   6. `[]` User reward token account
    ///   7. `[]` Pool LP token account
    ///   8. `[]` Pool reward token account
    ///   9. `[]` Pool LP token mint
    ///   10. `[]` Pool reward token mint
    ///   11. `[]` Token program id
    ///   12. `[]` Farm program id
    ///   13. `[]` amount
    Withdraw(u64),

    ///   Creator can add reward to his farm 
    /// 
    ///   0. `[w]` FarmPool to add reward to.
    ///   1. `[]` authority of this farm pool
    ///   2. `[s]` depositor
    ///   3. `[w]` User transfer authority.
    ///   4. `[]` User reward token account
    ///   5. `[]` Pool reward token account
    ///   6. `[]` Token program id
    ///   7. `[]` Farm program id
    ///   8. `[]` amount
    AddReward(u64),
    
    ///   Creator has to pay farm fee (if not CRP token pairing)
    ///   So this farm can be allowed to stake/unstake/harvest
    /// 
    ///   0. `[w]` FarmPool to pay farm fee.
    ///   1. `[]` authority of this farm pool
    ///   2. `[s]` payer
    ///   3. `[w]` User transfer authority.
    ///   4. `[]` User CRP token account
    ///   5. `[]` Fee Owner
    ///   6. `[]` Token program id
    ///   7. `[]` Farm program id
    ///   8. `[]` amount
    PayFarmFee(u64),
}

// below functions are used to test above instructions in the rust test side
// Function's parameters

/// Creates an 'initialize' instruction.
pub fn initialize(
    farm_id: &Pubkey,
    authority: &Pubkey,
    owner: &Pubkey,
    pool_lp_token_account: &Pubkey,
    pool_reward_token_account: &Pubkey,
    pool_mint_address: &Pubkey,
    reward_mint_address: &Pubkey,
    amm_id: &Pubkey,
    token_program_id: &Pubkey,
    nonce: u8,
    farm_program_id: &Pubkey,
    start_timestamp: u64,
    end_timestamp: u64,
) -> Instruction {
    
    let init_data = FarmPoolInstruction::Initialize{
        nonce,
        start_timestamp,
        end_timestamp
    };
    
    let data = init_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new(*farm_id, false),
        AccountMeta::new(*authority, false),
        AccountMeta::new_readonly(*owner, false),
        AccountMeta::new(*pool_lp_token_account, false),
        AccountMeta::new(*pool_reward_token_account, false),
        AccountMeta::new_readonly(*pool_mint_address, false),
        AccountMeta::new_readonly(*reward_mint_address, false),
        AccountMeta::new_readonly(*amm_id, false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];
    Instruction {
        program_id: *farm_program_id,
        accounts,
        data,
    }
}

/// Creates instructions required to deposit into a farm pool, given a farm
/// account owned by the user.
pub fn deposit(
    farm_id: &Pubkey,
    authority: &Pubkey,
    owner: &Pubkey,
    user_info_account: &Pubkey,
    user_lp_token_account: &Pubkey,
    user_reward_token_account: &Pubkey,
    pool_lp_token_account: &Pubkey,
    pool_reward_token_account: &Pubkey,
    pool_lp_mint: &Pubkey,
    pool_reward_mint: &Pubkey,
    fee_owner: &Pubkey,
    token_program_id: &Pubkey,
    farm_program_id: &Pubkey,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*farm_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new_readonly(*owner, true),
        AccountMeta::new(*user_info_account, false),
        AccountMeta::new(*user_lp_token_account, false),
        AccountMeta::new(*pool_lp_token_account, false),
        AccountMeta::new(*user_reward_token_account, false),
        AccountMeta::new(*pool_reward_token_account, false),
        AccountMeta::new(*pool_lp_mint, false),
        AccountMeta::new(*pool_reward_mint, false),
        AccountMeta::new(*fee_owner, false),
        AccountMeta::new(*token_program_id, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];
    Instruction {
        program_id: *farm_program_id,
        accounts,
        data: FarmPoolInstruction::Deposit(amount).try_to_vec().unwrap(),
    }
}

/// Creates a 'withdraw' instruction.
pub fn withdraw(
    farm_id: &Pubkey,
    authority: &Pubkey,
    owner: &Pubkey,
    user_info_account: &Pubkey,
    user_lp_token_account: &Pubkey,
    user_reward_token_account: &Pubkey,
    pool_lp_token_account: &Pubkey,
    pool_reward_token_account: &Pubkey,
    pool_lp_mint_info: &Pubkey,
    reward_mint_info: &Pubkey,
    fee_owner: &Pubkey,
    token_program_id: &Pubkey,
    farm_program_id: &Pubkey,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*farm_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new(*owner, true),
        AccountMeta::new(*user_info_account, false),
        AccountMeta::new(*user_lp_token_account, false),
        AccountMeta::new(*pool_lp_token_account, false),
        AccountMeta::new(*user_reward_token_account, false),
        AccountMeta::new(*pool_reward_token_account, false),
        AccountMeta::new(*pool_lp_mint_info, false),
        AccountMeta::new(*reward_mint_info, false),
        AccountMeta::new(*fee_owner, false),
        AccountMeta::new(*token_program_id, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];
    Instruction {
        program_id: *farm_program_id,
        accounts,
        data: FarmPoolInstruction::Withdraw(amount).try_to_vec().unwrap(),
    }
}


/// Creates a instruction required to add reward into a farm pool
pub fn add_reward(
    farm_id: &Pubkey,
    authority: &Pubkey,
    owner: &Pubkey,
    user_reward_token_account: &Pubkey,
    pool_reward_token_account: &Pubkey,
    pool_lp_mint_info: &Pubkey,
    token_program_id: &Pubkey,
    farm_program_id: &Pubkey,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*farm_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new_readonly(*owner, true),
        AccountMeta::new(*user_reward_token_account, false),
        AccountMeta::new(*pool_reward_token_account, false),
        AccountMeta::new(*pool_lp_mint_info, false),
        AccountMeta::new(*token_program_id, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];
    Instruction {
        program_id: *farm_program_id,
        accounts,
        data: FarmPoolInstruction::AddReward(amount).try_to_vec().unwrap(),
    }
}

/// Create a instruction required to pay additonal farm fee
pub fn pay_farm_fee(
    farm_id: &Pubkey,
    authority: &Pubkey,
    owner: &Pubkey,
    user_crp_token_account: &Pubkey,
    fee_owner: &Pubkey,
    token_program_id: &Pubkey,
    farm_program_id: &Pubkey,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*farm_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new_readonly(*owner, true),
        AccountMeta::new(*user_crp_token_account, false),
        AccountMeta::new(*fee_owner, false),
        AccountMeta::new(*token_program_id, false),
    ];
    Instruction {
        program_id: *farm_program_id,
        accounts,
        data: FarmPoolInstruction::PayFarmFee(amount).try_to_vec().unwrap(),
    }
}