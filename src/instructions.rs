// Imports
use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

// Struct definition, derive is used to automatically generate implementations for the Debug
#[derive(Debug, BorshDeserialize, BorshSerialize)]
// Defines a public struct UpdateArgs with a single public field value of type u32
pub struct UpdateArgs {
    pub value: u32,
}

// Represents different instructions for a counter.
pub enum CounterInstructions {
    Increment(UpdateArgs),
    Decrement(UpdateArgs),
    Update(UpdateArgs),
    Reset,
}

// Implements methods for the CounterInstructions enum
impl CounterInstructions {
    // This method is used to deserialize an array of bytes (input) into a CounterInstructions enum.
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Used to split the byte slice into a tuple containing the first element (variant) and the rest of the slice (rest).
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // Wrapped in an Ok() to create a Result containing either the successfully deserialized CounterInstructions enum or an error of type ProgramError.
        Ok(match variant {
            0 => Self::Increment(UpdateArgs::try_from_slice(rest).unwrap()),
            1 => Self::Decrement(UpdateArgs::try_from_slice(rest).unwrap()),
            2 => Self::Update(UpdateArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
