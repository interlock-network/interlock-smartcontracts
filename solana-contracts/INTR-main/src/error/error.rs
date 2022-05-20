/****************************************************************
 * INTR Solana Contract
 ****************************************************************/

use thiserror::Error;

use solana_program::program_error::ProgramError;

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
}

impl From<ContractError> for ProgramError {
    fn from(error: ContractError) -> Self {
        msg!("{:?}", error);
        ProgramError::Custom(error as u32)
    }
}
