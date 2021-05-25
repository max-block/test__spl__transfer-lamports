use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    msg!("transfer lamports started");
    let account_info_iter = &mut accounts.iter();

    // As part of the program specification the first account is the source
    // account and the second is the destination account
    let source_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;

    // Withdraw five lamports from the source
    **source_info.try_borrow_mut_lamports()? -= 5;
    // Deposit five lamports into the destination
    **destination_info.try_borrow_mut_lamports()? += 5;

    msg!("transfer lamports done");
    Ok(())
}