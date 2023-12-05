use {crate::utils::LimitedString, anchor_lang::prelude::*};

#[account()]
#[derive(Default)]
pub struct Realm {
    pub bump: u8,
    pub name: LimitedString,
    pub admin: Pubkey, // must also be the owner of the Switchboard functions
}

impl Realm {
    pub const LEN: usize = 8 + std::mem::size_of::<Realm>();
}
