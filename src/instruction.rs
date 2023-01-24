use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum GuideInstruction {
    AddGuideReview {
        name: String,
        rating: u8,
        description: String,
    },
    UpdateGuideReview {
        name: String,
        rating: u8,
        description: String,
    },
    AddComment {
        comment: String,
    },
}

#[derive(BorshDeserialize)]
struct GuideReviewPayload {
    name: String,
    rating: u8,
    description: String,
}

#[derive(BorshDeserialize)]
struct CommentPayload {
    comment: String,
}

impl GuideInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => {
                let payload = GuideReviewPayload::try_from_slice(rest).unwrap();
                Self::AddGuideReview {
                    name: payload.name,
                    rating: payload.rating,
                    description: payload.description,
                }
            }
            1 => {
                let payload = GuideReviewPayload::try_from_slice(rest).unwrap();
                Self::UpdateGuideReview {
                    name: payload.name,
                    rating: payload.rating,
                    description: payload.description,
                }
            }
            2 => {
                let payload = CommentPayload::try_from_slice(rest).unwrap();
                Self::AddComment {
                    comment: payload.comment,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
