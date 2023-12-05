use {
    crate::{error::HackatonError, state::Realm, utils::LimitedString},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(name:String)]
pub struct InitializeRealm<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: any
    pub admin: AccountInfo<'info>,

    #[account(
        init,
        payer=payer,
        seeds=[b"realm", name.as_bytes()],
        bump,
        space = Realm::LEN,
    )]
    pub realm: Account<'info, Realm>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_realm(ctx: Context<InitializeRealm>, name: String) -> Result<()> {
    // Checks
    {
        // verify input parameters
        require!(
            name.len() <= LimitedString::MAX_LENGTH,
            HackatonError::LimitedStringLengthExceeded
        );
    }

    // Initialize Realm account
    {
        ctx.accounts.realm.bump = *ctx.bumps.get("realm").ok_or(ProgramError::InvalidSeeds)?;
        ctx.accounts.realm.name = LimitedString::new(name);
        ctx.accounts.realm.admin = ctx.accounts.admin.key();
    }

    Ok(())
}
