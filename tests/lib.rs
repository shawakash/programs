#[cfg(test)]
mod tests {
    use super::*;
    use borsh::BorshSerialize;
    use first_sol::{
        processor::process_instruction,
        state::{AccountData, InstructionType},
    };
    use solana_program::{
        account_info::AccountInfo, program_pack::Pack, pubkey::Pubkey, rent::Rent,
        system_instruction,
    };
    use solana_program_test::*;
    use solana_sdk::{account::Account, signature::Signer, transaction::Transaction};

    fn create_account() -> (Pubkey, AccountInfo) {
        let owner = Pubkey::new_unique();
        let mut lamports = 100000;
        let mut data = vec![0; 1000];

        let account = AccountInfo::new(
            &owner,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Rent::default().minimum_balance(data.len()),
        );

        (owner, account)
    }

    #[test]
    fn test_increment() {
        let (owner, account) = create_account();
        let account_data = AccountData { calc_val: 5 };
        account_data
            .serialize(&mut &mut account.data.borrow_mut()[..])
            .unwrap();

        let instruction = InstructionType::CalcType(first_sol::state::CalcType::Increment(3));
        let instruction_data = instruction.try_to_vec().unwrap();

        process_instruction(&owner, &[account.clone()], &instruction_data).unwrap();

        let final_data = AccountData::try_from_slice(&account.data.borrow()).unwrap();
        assert_eq!(final_data.val, 8);
    }

    #[test]
    fn test_decrement() {
        let (owner, account) = create_account();

        let account_data = AccountData { calc_val: 5 };
        account_data
            .serialize(&mut &mut account.data.borrow_mut()[..])
            .unwrap();

        let instruction = InstructionType::CalcType(CalcType::Decrement(3));
        let instruction_data = instruction.try_to_vec().unwrap();

        process_instruction(&owner, &[account.clone()], &instruction_data).unwrap();

        let final_data = AccountData::try_from_slice(&account.data.borrow()).unwrap();
        assert_eq!(final_data.val, 2);
    }

    #[test]
    fn test_multiply() {
        let (owner, account) = create_account();

        let account_data = AccountData { calc_val: 5 };
        account_data
            .serialize(&mut &mut account.data.borrow_mut()[..])
            .unwrap();

        let instruction = InstructionType::CalcType(CalcType::Multiply(3));
        let instruction_data = instruction.try_to_vec().unwrap();

        process_instruction(&owner, &[account.clone()], &instruction_data).unwrap();

        let final_data = AccountData::try_from_slice(&account.data.borrow()).unwrap();
        assert_eq!(final_data.val, 15);
    }

    #[test]
    fn test_divide() {
        let (owner, account) = create_account();

        let account_data = AccountData { calc_val: 15 };
        account_data
            .serialize(&mut &mut account.data.borrow_mut()[..])
            .unwrap();

        let instruction = InstructionType::CalcType(CalcType::Divide(3));
        let instruction_data = instruction.try_to_vec().unwrap();

        process_instruction(&owner, &[account.clone()], &instruction_data).unwrap();

        let final_data = AccountData::try_from_slice(&account.data.borrow()).unwrap();
        assert_eq!(final_data.val, 5);
    }

    #[test]
    #[should_panic(expected = "InvalidInstructionData")]
    fn test_divide_by_zero() {
        let (owner, account) = create_account();

        let account_data = AccountData { calc_val: 15 };
        account_data
            .serialize(&mut &mut account.data.borrow_mut()[..])
            .unwrap();

        let instruction = InstructionType::CalcType(CalcType::Divide(0));
        let instruction_data = instruction.try_to_vec().unwrap();

        process_instruction(&owner, &[account.clone()], &instruction_data).unwrap();
    }

    #[tokio::test]
    async fn test_calculator_integration() {
        let program_id = Pubkey::new_unique();
        let mut program_test =
            ProgramTest::new("first_sol", program_id, processor!(process_instruction));

        let account_pubkey = Pubkey::new_unique();
        let mut account = Account::default();
        account.data = vec![0; 1000];
        account.owner = program_id;
        program_test.add_account(account_pubkey, account);

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mut transaction = Transaction::new_with_payer(
            &[solana_program::instruction::Instruction::new_with_borsh(
                program_id,
                &InstructionType::CalcType(CalcType::Increment(5)),
                vec![AccountMeta::new(account_pubkey, false)],
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        banks_client.process_transaction(transaction).await.unwrap();

        let account = banks_client
            .get_account(account_pubkey)
            .await
            .unwrap()
            .unwrap();
        let account_data = AccountData::try_from_slice(&account.data).unwrap();
        assert_eq!(account_data.val, 5);
    }
}
