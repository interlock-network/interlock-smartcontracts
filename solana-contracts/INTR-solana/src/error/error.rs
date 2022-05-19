/****************************************************************
 * Solana program template
 ****************************************************************/

use thiserror::Error;

use solana_program::program_error::ProgramError;

// besides the msg! I didn't these I didn't alter these much from the paulx implementation
// the msg! prints the error name in the log, instead of 'custom program error x00', etc

#[derive(Error, Debug, Copy, Clone)]
pub enum TemplateError {
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
}

impl From<TemplateError> for ProgramError {
    fn from(error: TemplateError) -> Self {
        msg!(":?", error);
        ProgramError::Custom(error as u32)
    }
}
