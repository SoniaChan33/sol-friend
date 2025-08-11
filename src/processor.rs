use std::char::MAX;

use crate::instruction::*;
use crate::state::*;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::borsh1::try_from_slice_unchecked;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;
use solana_program::sysvar::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{lamports, msg, pubkey};
use spl_token::solana_program::program_error::ProgramError;
pub struct Processor;

const PUBKEY_SIZE: usize = 32;
const U16_SIZE: usize = 2;
const USER_PROFILE_SIZE: usize = 6;
const MAX_FOLLOWERS_SIZE: usize = 256;

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // 处理指令的逻辑将会在这里实现
        let instruction = SocialInstruction::try_from_slice(instruction_data).unwrap();

        match instruction {
            SocialInstruction::InitializeUser { seed_type } => {
                // 处理初始化用户账户的逻辑
                Self::initialize_user(program_id, accounts, seed_type)
            }
            SocialInstruction::FollowUser { user_to_follow } => {
                // 处理关注用户的逻辑
                Self::follow_user(accounts, user_to_follow)
            }
            SocialInstruction::QueryFollowers => Self::query_followers(accounts),
            SocialInstruction::UnfollowUser { user_to_unfollow } => Ok(()),
            SocialInstruction::PostContent { content } => {
                // 处理发布内容的逻辑
                Ok(())
            }
            SocialInstruction::QueryPosts => {
                // 处理查询用户帖子
                Ok(())
            }
        }
    }

    fn initialize_user(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        seed_type: String,
    ) -> ProgramResult {
        // 处理初始化用户账户的逻辑
        // 创建PDA账户来存储用户的信息

        let account_info_iter = &mut accounts.iter();
        let user_account = next_account_info(account_info_iter)?;
        let pda_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        // 确认pda的种类
        let seed = match seed_type.as_str() {
            "profile" => "profile",
            "post" => "post",
            _ => return Err(ProgramError::InvalidArgument),
        };

        msg!("seed: {}", seed);

        let (pda, bump_seed) =
            Pubkey::find_program_address(&[user_account.key.as_ref(), seed.as_bytes()], program_id);
        msg!("PDA: {}, Bump Seed: {}", pda, bump_seed);
        if pda != pda_account.key.clone() {
            return Err(ProgramError::InvalidArgument);
        }

        // 获取租金信息
        let rent = Rent::get()?;

        // 计算空间
        let space = match seed_type.as_str() {
            "profile" => counter_profile_space(MAX_FOLLOWERS_SIZE),
            "post" => 0,
            _ => return Err(ProgramError::InvalidArgument),
        };

        // 计算租金
        let lamports: u64 = rent.minimum_balance(space);

        // 创建账户指令
        let create_account_ix = system_instruction::create_account(
            user_account.key,
            &pda,
            lamports,
            space as u64,
            program_id,
        );

        // 调用系统程序创建账户
        invoke_signed(
            &create_account_ix,
            &[
                user_account.clone(),
                pda_account.clone(),
                system_program.clone(),
            ],
            &[&[&user_account.key.as_ref(), seed.as_bytes(), &[bump_seed]]],
        )?;

        // 处理初始化用户账户的逻辑
        match seed_type.as_str() {
            "profile" => {
                let user_profile = UserProfile::new();
                user_profile.serialize(&mut *pda_account.try_borrow_mut_data()?)?;
                msg!("User profile initialized: {:?}", user_profile);
            }
            "post" => {
                // 处理帖子账户的逻辑
            }
            _ => return Err(ProgramError::InvalidArgument),
        }
        msg!("User account initialized successfully");
        Ok(())
    }

    // 处理关注用户的逻辑
    fn follow_user(accounts: &[AccountInfo], user_to_follow: Pubkey) -> ProgramResult {
        // 获取用户账户和PDA账户
        let account_info_iter = &mut accounts.iter();
        let pda_account: &AccountInfo<'_> = next_account_info(account_info_iter)?;

        // 从PDA中获取用户个人资料
        let mut size: usize = 0;
        {
            // TODO 这里为什么要单独作用域，并且熟悉一下borrow
            let data = &pda_account.data.borrow();

            // TODO 为什么是数组切片
            let len: &[u8] = &data[..U16_SIZE];
            let pubkey_count = bytes_to_u16(len).unwrap() as usize;
            size = counter_profile_space(pubkey_count);
            msg!("size is {:?}", size);
        }

        // 反序列化用户个人资料
        let mut user_profile: UserProfile =
            UserProfile::try_from_slice(&pda_account.data.borrow()[..size])?;
        msg!("user_profile is {:?}", user_profile);
        user_profile.follow(user_to_follow);
        // TODO 这里为什么需要再序列化
        user_profile.serialize(&mut *pda_account.try_borrow_mut_data()?)?;
        Ok(())
    }

    fn query_followers(accounts: &[AccountInfo]) -> ProgramResult {
        // 获取用户账户和PDA账户
        let account_info_iter = &mut accounts.iter();
        let pda_account = next_account_info(account_info_iter)?;

        // 这样比较麻烦
        // // 从PDA中获取用户个人资料
        // let mut size: usize = 0;
        // {
        //     let data = &pda_account.data.borrow();
        //     let len = &data[..U16_SIZE];
        //     let pubkey_count = bytes_to_u16(len).unwrap() as usize;
        //     size = counter_profile_space(pubkey_count);
        //     msg!("size is {:?}", size);
        // }

        // // 反序列化用户个人资料
        // let user_profile: UserProfile =
        //     UserProfile::try_from_slice(&pda_account.data.borrow()[..size])?;
        // msg!("user_profile is {:?}", user_profile);

        let user_profile =
            try_from_slice_unchecked::<UserProfile>(&pda_account.data.borrow()).unwrap();

        msg!("user_profile is {:?}", user_profile);
        // TODO 返回关注者列表
        Ok(())
    }
}

fn bytes_to_u16(bytes: &[u8]) -> Result<u16, ProgramError> {
    if bytes.len() != U16_SIZE {
        return Err(ProgramError::InvalidArgument);
    }
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn counter_profile_space(pubkey_count: usize) -> usize {
    // 计算用户个人资料的空间
    USER_PROFILE_SIZE + PUBKEY_SIZE * pubkey_count
}
