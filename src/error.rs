use thiserror :: Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    // Invalid Instruction
    #[error("Invalid Instruction")]
    InvalidInstruction
}