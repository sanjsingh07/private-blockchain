use crate::instruction::InstructionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaratsError {
    /// arithmetic underflowed
    #[error("Arithmetic underflowed")]
    ArithmeticUnderflow,

    /// arithmetic overflowed
    #[error("Arithmetic overflowed")]
    ArithmeticOverflow,
}

impl From<CaratsError> for InstructionError {
    fn from(error: CaratsError) -> Self {
        match error {
            CaratsError::ArithmeticOverflow => InstructionError::ArithmeticOverflow,
            CaratsError::ArithmeticUnderflow => InstructionError::ArithmeticOverflow,
        }
    }
}
