use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GuideAccountState {
    pub discriminator: String,
    pub is_initialized: bool,
    pub reviewer: Pubkey,
    pub rating: u8,
    pub name: String,
    pub description: String,
}

impl GuideAccountState {
    pub const DISCRIMINATOR: &'static str = "review";

    pub fn get_account_size(name: String, description: String) -> usize {
        return (4 + GuideAccountState::DISCRIMINATOR.len())
            + 1
            + 1
            + (4 + name.len())
            + (4 + description.len());
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GuideCommentCounter {
    pub discriminator: String,
    pub is_initialized: bool,
    pub counter: u64,
}

impl GuideCommentCounter {
    pub const DISCRIMINATOR: &'static str = "counter";
    pub const SIZE: usize = (4 + GuideCommentCounter::DISCRIMINATOR.len()) + 1 + 8;
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GuideComment {
    pub discriminator: String,
    pub is_initialized: bool,
    pub review: Pubkey,
    pub commenter: Pubkey,
    pub comment: String,
    pub count: u64,
}

impl GuideComment {
    pub const DISCRIMINATOR: &'static str = "comment";

    pub fn get_account_size(comment: String) -> usize {
        return (4 + GuideComment::DISCRIMINATOR.len()) + 1 + 32 + 32 + (4 + comment.len()) + 8;
    }
}

impl Sealed for GuideAccountState {}

impl IsInitialized for GuideAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for GuideCommentCounter {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for GuideComment {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
