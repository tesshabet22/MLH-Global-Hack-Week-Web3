use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("EB7aXseUQsbqTiJeWA8XvukcFUYKqGxWwxWJz7Z3QNJn");

#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data); // Message will show up in the tx logs
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct NewAccount {
    data: u64
}
pub struct JournalEntryState {
    pub owner: Pubkey, 
    #[max_len(50)] /* */
    pub title: String, 
    #[max_len(1000)] //* */
    pub message: String,
}

#[program] /* */
mod journal {
    use super::*;

    pub fn create_journal_entry(
        ctx: Context<CreateEntry>,
        title: String, 
        message: String, 
        ) -> Result<()> {
            msg!("Journal Entry Created");
            msg!("Title: {}", title);
            msg!("Message: {}",message);

            let journal_entry = &mut ctx.accounts.journal_entry;
            journal_entry.owner = ctx.accounts.owner.key();
            journal_entry.title = title;
            journal_entry.message = message;
            Ok(())
        }
}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct CreateEntry<'info> {
    #[account(
        init_if_needed,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        payer = owner, 
        space = 8 + JournalEntryState::INIT_SPACE
    )]
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// update instruction
#[program]
mod journal {
    use super::*;

    pub fn update_journal_entry(
        ctx: Context<UpdateEntry>,
        title: String, 
        message: String, 
        ) -> Result<()> {
            msg!("Journal Entry Updated");
            msg!("Title: {}", title);
            msg!("Message: {}",message);

            let journal_entry = &mut ctx.accounts.journal_entry;
            journal_entry.owner = ctx.accounts.owner.key();
            journal_entry.title = title;
            journal_entry.message = message;
            Ok(())
        }
}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct UpdateEntry<'info> {
    #[account(
        mut, 
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump, 
        realloc = 8 + 32 + 1 + 4 + title.len() + 4 + message.len(),
        realloc::payer = owner,
        realloc::zero = true,
        )]
        pub journal_entry: Account<'info, JournalEntryState>,
        #[account(mut)]
        pub owner: Signer<'info>,
        pub system_program: Program<'info, System>,
}

// Delete instruction
pub fn delete_journal_entry(_ctx: Context<DeleteEntry>, title:String) -> Result<()> {
    msg!("Journal entry title {} deleted", title);
    Ok(())
}

// Defining the delete entry context 
#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        close = owner,
    )]
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}