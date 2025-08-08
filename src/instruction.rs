use borsh::{BorshDeserialize, BorshSerialize};
use spl_token::solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum SocialInstruction {
    // 初始化账户
    InitializeUser { seed_type: String },
    // 关注
    FollowUser { user_to_follow: Pubkey },
    // 取消关注
    UnfollowUser { user_to_unfollow: Pubkey },
    //
    QueryFollowers,
    PostContent { content: String },
    QueryPosts,
}
