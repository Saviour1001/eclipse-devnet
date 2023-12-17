use anchor_lang::prelude::*;

declare_id!("E7YPSNL1RsHxBj8ByjcdnDbswfUDMYf8XMyby4kUDYxH");

#[program]
pub mod to_do_list {
    use super::*;

    pub fn add_task(ctx: Context<AddTask>, _description: String) -> Result<()> {
        let author = &ctx.accounts.author; // The `author` account
        let clock = Clock::get().unwrap(); // Getting the current timestamp
        let task = &mut ctx.accounts.task; // The `task` account

        task.is_done = false;
        task.description = _description;
        task.owner = *author.key;
        task.created_at = clock.unix_timestamp;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddTask<'info> {
    #[account(init, payer = author, space = Task::LEN)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Task {
    pub description: String,
    pub is_done: bool,
    pub owner: Pubkey,
    pub created_at: i64,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_DESCRIPTION_LENGTH: usize = 280 * 4; // 280 chars max.
const CREATED_AT: usize = 8;

impl Task {
    pub const LEN: usize =
        DISCRIMINATOR_LENGTH + // discriminator
        PUBLIC_KEY_LENGTH + // author
        STRING_LENGTH_PREFIX +
        MAX_DESCRIPTION_LENGTH + // task
        CREATED_AT; // timestamp
}
