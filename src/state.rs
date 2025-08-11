use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct UserProfile {
    pub data_len: u16,
    pub followers: Vec<Pubkey>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct UserPost {
    pub post_count: u16,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Post {
    pub content: String,
    pub timestamp: u64,
}

impl UserProfile {
    pub fn new() -> Self {
        UserProfile {
            data_len: 0,
            followers: Vec::new(),
        }
    }
    pub fn follow(&mut self, user: Pubkey) {
        if !self.followers.contains(&user) {
            self.followers.push(user);
            self.data_len += 1;
        }
    }

    pub fn unfollow(&mut self, user_to_follow: Pubkey) {
        // If the user is being followed, remove them from the followers list
        self.followers.retain(|&x| x != user_to_follow);
        self.data_len = self.followers.len() as u16;
    }
}
