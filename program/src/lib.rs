use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint, entrypoint::ProgramResult, msg, program::invoke, pubkey::Pubkey, system_instruction};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("transfer lamports started");
    let account_info_iter = &mut accounts.iter();
    let alice = next_account_info(account_info_iter)?;
    let bob = next_account_info(account_info_iter)?;
    // the third account is SystemProgram.programId. Don't forget it


    // // Withdraw five lamports from the source
    // **source_info.try_borrow_mut_lamports()? -= 5;
    // // Deposit five lamports into the destination
    // **destination_info.try_borrow_mut_lamports()? += 5;
    // <-- it doesn't work! Use invoke()

    invoke(&system_instruction::transfer(alice.key, bob.key, 777), &[alice.clone(), bob.clone()])?;
    msg!("transfer done");

    Ok(())
}