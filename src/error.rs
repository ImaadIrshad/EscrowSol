// Defining our own error type 
use thiserror :: Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    // Invalid Instruction
    #[error("Invalid Instruction")]
    InvalidInstruction
}

impl From<EscrowError> for ProgramError {
    fn from(e:EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}


