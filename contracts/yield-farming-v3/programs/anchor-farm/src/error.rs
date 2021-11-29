use anchor_lang::prelude::*;

#[error]
pub enum FarmError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("AlreadyInUse")]
    AlreadyInUse,
    #[msg("InvalidProgramAddress")]
    InvalidProgramAddress,
    #[msg("InvalidState")]
    InvalidState,
    #[msg("InvalidOwner")]
    InvalidOwner,
    #[msg("NotAllowed")]
    NotAllowed,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("InvalidOracleConfig")]
    InvalidOracleConfig,
    #[msg("InvalidAccountInput")]
    InvalidAccountInput,
    #[msg("PreciseError")]
    PreciseError,
    #[msg("Error: reward overflow")]
    RewardOverflow,
    #[msg("Error: timestamp overflow")]
    TimeOverflow,
    #[msg("Error: invalid pubkey")]
    InvalidPubkey,
    #[msg("Error: amm error")]
    AmmError,
    #[msg("Error: farm is not started yet")]
    NotStarted,
    #[msg("Error: farm was ended")]
    FarmEnded,
    #[msg("Error: not enough balance")]
    NotEnoughBalance,
    #[msg("WrongPoolMint")]
    WrongPoolMint,
    #[msg("WrongMintAddress")]
    WrongMintAddress,
}