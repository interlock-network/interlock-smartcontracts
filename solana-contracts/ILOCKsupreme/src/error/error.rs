/****************************************************************
 * ILOCKsupreme Solana Contract
 ****************************************************************/

use thiserror::Error;

use solana_program::{
        program_error::ProgramError,
        msg,
};


#[derive(Error, Debug, Copy, Clone)]
pub enum ContractError {

    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Not Rent Exempt")]
    NotRentExempt,

    #[error("Amount Overflow")]
    AmountOverflow,

    #[error("Try From Slice Fail")]
    TryFromSliceError,

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

    #[error("Staker count threshold passed")]
    StakerCountThresholdPassedError,

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
