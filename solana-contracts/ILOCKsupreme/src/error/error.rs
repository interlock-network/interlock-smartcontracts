/****************************************************************
 * ILOCKsupreme Solana Contract
 ****************************************************************/

use thiserror::Error;

use solana_program::{
        program_error::ProgramError,
        msg,
};

// TODO:
// . clean out unneeded err
//


#[derive(Error, Debug, Copy, Clone)]
pub enum ContractError {

    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,

    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,

    /// Amount Overflow
    #[error("Amount Overflow")]
    AmountOverflow,

    /// Try From Slice
    #[error("Try From Slice Fail")]
    TryFromSliceError,

    /// Instruction One Attempt Fail
    #[error("Instruction One Attempt Fail")]
    InstructionOneAttemptError,

    #[error("Global Account Already Exists")]
    GlobalAlreadyExistsError,

    #[error("'Owner' is an imposter!")]
    OwnerImposterError,

    #[error("Stake not yet resolved")]
    StakeNotResolvedError,

    #[error("Entity not yet settled")]
    EntityNotSettledError,

    #[error("Entity not yet settling")]
    EntityNotSettlingError,

    #[error("Hunter already set")]
    HunterAlreadySetError,

    #[error("Entity settling")]
    EntitySettlingError,

    #[error("Entity settled")]
    EntitySettledError,
    
    #[error("Entity claimed")]
    EntityClaimedError,

    #[error("Entity unclaimed")]
    EntityUnclaimedError,

    #[error("Wrong stake valence")]
    WrongStakeValenceError,

    #[error("Time threshold passed")]
    TimeThresholdPassedError,

    #[error("Total stake threshold passed")]
    TotalStakeThresholdPassedError,

    #[error("Positiive stake threshold passed")]
    PositiveStakeThresholdPassedError,
    
    #[error("Negative stake threshold passed")]
    NegativeStakeThresholdPassedError,

    #[error("Max staker threshold passed")]
    MaxStakerThresholdPassedError,

    #[error("Minimum stake not met")]
    MinimumStakeNotMetError,
    
    #[error("User not a hunter")]
    UserNotHunterError,

    #[error("Not User stake")]
    NotUserStakeError,

    #[error("Insufficient balance")]
    InsufficientBalanceError,

}

impl From<ContractError> for ProgramError {
    fn from(error: ContractError) -> Self {
        msg!("{:?}", error);
        ProgramError::Custom(error as u32)
    }
}
