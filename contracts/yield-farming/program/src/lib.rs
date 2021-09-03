/// Main Entrypoint and declaration file

use solana_program::{
    account_info::{ AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::PrintProgramError,
    pubkey::Pubkey,
};
/// module declaration
/// 
/// error module
pub mod error;
/// instruction module
pub mod instruction;
/// processor module
pub mod processor;
/// state modulesolana-keygen new
pub mod state;
/// constants
pub mod constant;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the Yield Farming program was loaded into
    accounts: &[AccountInfo], // account informations
    _instruction_data: &[u8], // Instruction data
) -> ProgramResult {
    // process a passed instruction
    if let Err(error) = processor::Processor::process(program_id, accounts, _instruction_data) {
        
        // catch the error so we can print it
        error.print::<error::FarmPoolError>();
        Err(error)
    } else {
        // processed successfully
        Ok(())
    }
    
}
