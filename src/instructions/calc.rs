use crate::state::AccountData;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
};

pub fn increment_instruction(account: &AccountInfo, val: u32) -> ProgramResult {
    let mut data = account.data.borrow_mut();
    let mut account_data = AccountData::try_from_slice(&data)?;

    msg!("Account data before: {:?}", account_data);
    account_data.calc_val += val;
    msg!("Account data after: {:?}", account_data);
    account_data.serialize(&mut &mut data[..])?;

    Ok(())
}

pub fn decrement_instruction(account: &AccountInfo, val: u32) -> ProgramResult {
    let mut data = account.data.borrow_mut();
    let mut account_data = AccountData::try_from_slice(&data)?;

    msg!("Account data before: {:?}", account_data);
    account_data.calc_val -= val;
    msg!("Account data after: {:?}", account_data);
    account_data.serialize(&mut &mut data[..])?;

    Ok(())
}

pub fn multiply_instruction(account: &AccountInfo, val: u32) -> ProgramResult {
    let mut data = account.data.borrow_mut();
    let mut account_data = AccountData::try_from_slice(&data)?;

    msg!("Account data before: {:?}", account_data);
    account_data.calc_val *= val;
    msg!("Account data after: {:?}", account_data);
    account_data.serialize(&mut &mut data[..])?;

    Ok(())
}

pub fn divide_instruction(account: &AccountInfo, val: u32) -> ProgramResult {
    let mut data = account.data.borrow_mut();
    let mut account_data = AccountData::try_from_slice(&data)?;

    msg!("Account data before: {:?}", account_data);
    if val == 0 {
        msg!("Cannot divide by zero");
        return Err(ProgramError::InvalidInstructionData);
    }
    account_data.calc_val = match account_data.calc_val.checked_div(val) {
        Some(result) => result,
        None => {
            msg!("Error: Division operation failed!");
            return Err(ProgramError::InvalidInstructionData);
        }
    };

    msg!("Account data after: {:?}", account_data);
    account_data.serialize(&mut &mut data[..])?;

    Ok(())
}
