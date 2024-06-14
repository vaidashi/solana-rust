use anchor_lang::prelude::*;
use solana_program::system_program;

declare_id!("J2U8xogvRzRaYg3R2wa2qCowBa2T3twL9UZi3NZqKEex");

const MAX_TOPIC_LENGTH: usize = 50;
const MAX_CONTENT_LENGTH: usize = 280;
const DISCRIMINATOR_LENGTH: usize = 8; // serves as a type identifier for the account, distinguishing different account types within the program
const PUBLIC_KEY_LENGTH: usize = 32; // links each message to its respective author
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;

#[account]
pub struct Message {
    pub author: Pubkey,
    pub topic: String,
    pub content: String,
    pub timestamp: i64,
}

impl Message {
    const LEN: usize = DISCRIMINATOR_LENGTH 
    + PUBLIC_KEY_LENGTH 
    + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH 
    + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH 
    + TIMESTAMP_LENGTH;
}

#[derive(Accounts)]
pub struct SendMessage<'info> {
    #[account(init, payer = author, space = Message::LEN)] 
    pub message: Account<'info, Message>, 
    #[account(mut)] 
    pub author: Signer<'info>, 
    #[account(address = system_program::ID)] 
    // pub system_program: AccountInfo<'info>, 
    pub system_program: Program<'info, System>
}

#[program]
pub mod solana_dapp {
    use super::*;

    pub fn send_message(ctx: Context<SendMessage>, topic: String, content: String) -> Result<()> {
        let message: &mut Account<Message> = &mut ctx.accounts.message;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        if topic.chars().count() > MAX_TOPIC_LENGTH {
            return Err(ErrorCode::TopicTooLong.into());
        }

        if content.chars().count() > MAX_CONTENT_LENGTH {
            return Err(ErrorCode::ContentTooLong.into());
        }

        message.author = *author.key;
        message.topic = topic;
        message.content = content;
        message.timestamp = clock.unix_timestamp;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Error! The provided topic should be 50 characters max.")]
    TopicTooLong,
    #[msg("Error! The provided content should be 280 characters max.")]
    ContentTooLong,
}
