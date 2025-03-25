use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum InstructionType {
    CalcType(CalcType),
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CalcType {
    Increment(u32),
    Decrement(u32),
    Multiply(u32),
    Divide(u32),
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountData {
    pub calc_val: u32,
}
