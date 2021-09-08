/// the use import wasn't auto-imported
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
  #[error("Invalid Instruction")]
  InvalidInstruction,
}
