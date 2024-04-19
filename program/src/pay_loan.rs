use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	Broker,
	Loan,
};
use crate::generated::errors::InformalLenderError;

/// A client can pay a loan through this instruction. When paying
/// the contract will calculate the interest based on the loan approved
/// fee. Additioanlly, it will transfer money from the client's account
/// to the broker account
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` client: [AccountInfo] 
/// 2. `[writable]` loan: [Loan] 
/// 3. `[writable]` broker: [Broker] 
///
/// Data:
/// - amount: [u64] The amount to pay to the loan
/// - loan_seed_index: [u32] Auto-generated, from the input "loan" for the its seed definition "Loan", sets the seed named "index"
pub fn pay_loan(
	program_id: &Pubkey,
	client: &AccountInfo,
	loan: &mut AccountPDA<Loan>,
	broker: &mut AccountPDA<Broker>,
	amount: u64,
) -> ProgramResult {
    // Implement your business logic here...
    // Check if the client is not over-paying
    if loan.data.payed + amount > loan.data.amount {
        return Err(InformalLenderError::InvalidInstruction.into());
    }
    
    // Calculate how much interest the client need to pay
    let interest = loan.data.amount / loan.data.fee as u64;
    
    // Update the broker state
    broker.data.revenue += interest as u128;
    broker.data.capital += amount as u128;
    broker.data.lended -= amount as u128;
    
    // Update loan's payed amount
    loan.data.payed += amount;
    
    // Transfer from client's account to broker account
    let total = amount + interest;
    **client.try_borrow_mut_lamports()? -= total;
    **broker.info.try_borrow_mut_lamports()? += total;





    Ok(())
}