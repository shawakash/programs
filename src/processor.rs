use crate::{instructions, state::InstructionType};

use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;

    if acc.owner != program_id {
        msg!("Account is not owned by this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction_type = InstructionType::try_from_slice(&instruction_data)?;
    msg!("Instruction: {:?}", instruction_type);
    msg!("Called by: {:?}", acc.key);

    instructions::handle_instructions(instruction_type, &acc)
}
