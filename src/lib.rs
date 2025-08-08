use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod instruction;
pub mod processor;
pub mod state;
use crate::processor::Processor;
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 程序入口点 - 当前为空实现
    processor::Processor::process_instruction(program_id, accounts, instruction_data)
}
