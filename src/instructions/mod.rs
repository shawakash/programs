use crate::state::{CalcType, InstructionType};

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg};

mod calc;

pub fn handle_instructions(instruction_type: InstructionType, acc: &AccountInfo) -> ProgramResult {
    match instruction_type {
        InstructionType::CalcType(calc_type) => handle_calc(calc_type, acc),
    }
}

fn handle_calc(calc_type: CalcType, acc: &AccountInfo) -> ProgramResult {
    msg!("Handling calc instruction: {:?}", calc_type);

    match calc_type {
        CalcType::Decrement(val) => calc::decrement_instruction(acc, val),
        CalcType::Increment(val) => calc::increment_instruction(acc, val),
        CalcType::Multiply(val) => calc::multiply_instruction(acc, val),
        CalcType::Divide(val) => calc::divide_instruction(acc, val),
    }
}
