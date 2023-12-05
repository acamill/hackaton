pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use {anchor_lang::prelude::*, instructions::*};

#[cfg(feature = "localnet")]
declare_id!("GiN7xhFgwGTciboPZHyGu2v16LDezaXgkhMW9Pv5xiet");
#[cfg(feature = "devnet")]
declare_id!("BDWigaQZHnLxc2vZ5MqvorW5ki5uFpaNWezeyUoBaZgj");
#[cfg(feature = "mainnet-beta")]
declare_id!("prog111");

solana_security_txt::security_txt! {
    name: "Hackaton",
    project_url: "https://github.com/acamill",
    contacts: "email:alexcamill@gmail.com",
    policy: "",
    preferred_languages: "en",
    auditors: "None"
}

#[program]
pub mod hackaton {
    use super::*;

    // Public IX ----------------------------------------------------------------

    /// Called to initialize a new realm.
    /// Will be called by us once at inception but we can imagine Seasonal realms afterward or player run realms.
    pub fn initialize_realm(ctx: Context<InitializeRealm>, name: String) -> Result<()> {
        instructions::initialize_realm(ctx, name)
    }
}
