#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
// pub use state::*;
pub mod utils;
declare_id!("7xJqwFGPehY5SFzGGhW9d8V77x8Fc76XiWTyh3zWzJjG");

declare_program!(bubblegum);

#[program]
pub mod play_mpl_bubblegum {
    use super::*;

    pub fn create_tree(
        ctx: Context<CreateTree>,
        max_depth: u32,
        max_buffer_size: u32,
        is_public: bool,
    ) -> Result<()> {
        ctx.accounts.handler(max_depth, max_buffer_size, is_public)
    }

    pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {
        ctx.accounts.handler()
    }

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        name: String,
        uri: String,
    ) -> Result<()> {
        ctx.accounts.handler(name, uri)
    }

    pub fn mint_nft_to_collection(ctx: Context<MintNftToCollection>) -> Result<()> {
        ctx.accounts.handler()
    }
}
