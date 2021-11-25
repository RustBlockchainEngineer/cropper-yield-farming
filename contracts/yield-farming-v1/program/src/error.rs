//! All error types for this program

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the FarmPool program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum FarmError {
    // 0.
    /// The account cannot be initialized because it is already being used.
    #[error("AlreadyInUse")]
    AlreadyInUse,
    /// The program address provided doesn't match the value generated by the program.
    #[error("InvalidProgramAddress")]
    InvalidProgramAddress,
    /// The farm pool state is invalid.
    #[error("InvalidState")]
    InvalidState,
    /// The calculation failed.
    #[error("CalculationFailure")]
    CalculationFailure,
    /// Farm pool fee > 1.
    #[error("FeeTooHigh")]
    FeeTooHigh,

    // 5.
    /// Token account is associated with the wrong mint.
    #[error("WrongAccountMint")]
    WrongAccountMint,
    /// Wrong pool manager account.
    #[error("WrongManager")]
    WrongManager,
    /// Required signature is missing.
    #[error("SignatureMissing")]
    SignatureMissing,
    /// Invalid validator stake list account.
    #[error("InvalidValidatorStakeList")]
    InvalidValidatorStakeList,
    /// Invalid owner fee account.
    #[error("InvalidFeeAccount")]
    InvalidFeeAccount,

    // 10.
    /// Specified pool mint account is wrong.
    #[error("WrongPoolMint")]
    WrongPoolMint,
    /// The farm was not started yet
    #[error("NotStarted")]
    NotStarted,
    /// The farm was ended
    #[error("FarmEnded")]
    FarmEnded,
    /// Zero deposit balance
    #[error("Zero deposit balance!")]
    ZeroDepositBalance,
    /// This farm is not allowed yet
    #[error("This farm is not allowed yet")]
    NotAllowed,

    // 15.
    /// Wrong Farm Fee
    #[error("Wrong Farm Fee")]
    InvalidFarmFee,
    /// Wrong Amm Id
    #[error("Wrong Amm Id")]
    WrongAmmId,
    /// Wrong Farm pool
    #[error("Wrong farm pool")]
    WrongFarmPool,
    /// Wrong Creator
    #[error("Wrong creator")]
    WrongCreator,
    /// Wrong Period
    #[error("Wrong Period")]
    WrongPeriod,

    /// Invalid Owner
    #[error("Invalid Owner")]
    InvalidOwner,

    /// Invalid Signer
    #[error("Invalid Signer")]
    InvalidSigner,

    /// not enough amount
    #[error("not enough amount")]
    NotEnoughBalance,

    /// invalid token account
    #[error("Invalid token account")]
    InvalidTokenAccount,

    /// invalid pubkey
    #[error("Invalid Pubkey")]
    InvalidPubkey,

    /// precise error
    #[error("Precise Error")]
    PreciseError,

    /// Program data is not initialized yet
    #[error("Program data is not initialized yet")]
    NotInitializedProgramData,

    /// invalid delegate
    #[error("Token account has a delegate")]
    InvalidDelegate,

    /// invalid delegate
    #[error("Token account has a close authority")]
    InvalidCloseAuthority,

    /// invalid delegate
    #[error("Pool token mint has a freeze authority")]
    InvalidFreezeAuthority,

     /// Pool token mint has a non-zero supply
     #[error("Pool token mint has a non-zero supply")]
     InvalidSupply,

     /// Not initialized
     #[error("Not initialized")]
     NotInitialized,

     /// Invalid SystemProgram Id
     #[error("Invalid SystemProgram Id")]
     InvalidSystemProgramId,

     /// Invalid Rent Sysvar Id
     #[error("Invalid Rent Sysvar Id")]
     InvalidRentSysvarId,

     /// Invalid Clock Sysvar Id
     #[error("Invalid Clock Sysvar Id")]
     InvalidClockSysvarId,

     /// InvalidDualYieldAddress
     #[error("InvalidDualYieldAddress")]
     InvalidDualYieldAddress,

}
impl From<FarmError> for ProgramError {
    fn from(e: FarmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for FarmError {
    fn type_of() -> &'static str {
        "Farm Pool Error"
    }
} 