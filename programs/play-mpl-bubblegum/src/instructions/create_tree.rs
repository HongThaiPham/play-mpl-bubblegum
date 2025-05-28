use anchor_lang::prelude::*;

use crate::{
    bubblegum::cpi::{accounts::CreateTreeV2, create_tree_v2},
    utils::{MplBubblegum, Noop, SplAccountCompression},
};

#[derive(Accounts)]
pub struct CreateTree<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: will used by mpl_bubblegum program
    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = mpl_bubblegum_program.key()
    )]
    pub tree_config: UncheckedAccount<'info>,
    /// CHECK: Zero initialized account
    #[account(signer)]
    pub merkle_tree: AccountInfo<'info>,
    pub mpl_bubblegum_program: Program<'info, MplBubblegum>,
    pub spl_compression_program: Program<'info, SplAccountCompression>,
    pub noop_program: Program<'info, Noop>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateTree<'info> {
    pub fn handler(&mut self, max_depth: u32, max_buffer_size: u32) -> Result<()> {
        self.create_tree(max_depth, max_buffer_size)?;
        Ok(())
    }

    fn create_tree(&self, max_depth: u32, max_buffer_size: u32) -> Result<()> {
        create_tree_v2(
            CpiContext::new(
                self.mpl_bubblegum_program.to_account_info(),
                CreateTreeV2 {
                    merkle_tree: self.merkle_tree.to_account_info(),
                    payer: self.payer.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    compression_program: self.spl_compression_program.to_account_info(),
                    log_wrapper: self.noop_program.to_account_info(),
                    tree_authority: self.tree_config.to_account_info(),
                    tree_creator: Option::None,
                },
            ),
            max_depth,
            max_buffer_size,
            Option::Some(true),
        )?;

        Ok(())
    }
}
