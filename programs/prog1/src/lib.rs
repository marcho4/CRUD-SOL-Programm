use anchor_lang::prelude::*;

declare_id!("2FRKbweUTZrUQEjLoPaDbA7nznbvP5jw7UtNhRQNKX1K");

#[program]
pub mod prog1 {
    use super::*;

    pub fn init_journal(ctx: Context<CreateJournal>, name: String, message: String) -> Result<()> {

        let journal = &mut ctx.accounts.journal_entry;
        journal.owner = ctx.accounts.owner.key();
        journal.name = name;
        journal.message = message;

        msg!("Journal Entry Created");
        msg!("Title: {}", journal.name);
        msg!("Message: {}", journal.message);
        Ok(())
    }

    pub fn update_journal(ctx: Context<UpdateJournal>, name: String, message: String) -> Result<()> {
        msg!("Journal Entry Updated");
        msg!("Title: {}", name);
        msg!("Message: {}", message);
        let journal = &mut ctx.accounts.journal;
        journal.message = message;
        Ok(())
    }

    pub fn delete_journal(ctx: Context<DeleteJournal>, name: String) -> Result<()> {
        msg!("Journal {} has been deleted!", name);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DeleteJournal<'info> {

    #[account(
        mut, 
        seeds = [name.as_bytes(), owner.key().as_ref()], 
        bump, 
        close = owner
    )]
    pub journal: Account<'info, Journal>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String, message: String)]
pub struct UpdateJournal<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + 32 + 1 + 4 + name.len() + 4 + message.len(),
        realloc::payer = owner,
        realloc::zero = true,
    )]
    pub journal: Account<'info, Journal>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(name: String, message: String)]
pub struct CreateJournal<'info> {
    #[account(
        init_if_needed,
        seeds = [name.as_bytes(), owner.key.as_ref()],
        bump, 
        payer = owner,
        space = 8 + Journal::INIT_SPACE,
    )]
    pub journal_entry: Account<'info, Journal>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Journal {
    pub owner: Pubkey,

    #[max_len(50)]
    pub name: String,

    #[max_len(1000)]
    pub message: String,
}