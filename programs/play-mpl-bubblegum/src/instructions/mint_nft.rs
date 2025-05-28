use anchor_lang::prelude::*;

use crate::bubblegum::types::{Creator, MetadataArgsV2, TokenStandard};
use crate::utils::{MplBubblegum, MplCore, Noop, SplAccountCompression};

use crate::bubblegum::cpi::{accounts::MintV2, mint_v2};

#[derive(Accounts)]
pub struct MintNft<'info> {
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
    /// CHECK: will used by mpl_bubblegum program
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,
    pub mpl_bubblegum_program: Program<'info, MplBubblegum>,
    pub mpl_core_program: Program<'info, MplCore>,
    pub spl_compression_program: Program<'info, SplAccountCompression>,
    pub noop_program: Program<'info, Noop>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintNft<'info> {
    pub fn handler(&mut self) -> Result<()> {
        // Here you would implement the logic to mint an NFT using the mpl_bubblegum program.
        // This is a placeholder for the actual minting logic.
        msg!("Minting NFT...");

        mint_v2(
            CpiContext::new(
                self.mpl_bubblegum_program.to_account_info(),
                MintV2 {
                    payer: self.payer.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    compression_program: self.spl_compression_program.to_account_info(),
                    log_wrapper: self.noop_program.to_account_info(),
                    leaf_owner: self.payer.to_account_info(),
                    merkle_tree: self.merkle_tree.to_account_info(),
                    mpl_core_program: self.mpl_core_program.to_account_info(),
                    tree_authority: self.tree_config.to_account_info(),
                    collection_authority: Option::None,
                    core_collection: Option::None,
                    leaf_delegate: Option::None,
                    mpl_core_cpi_signer: Option::None,
                    tree_delegate: Option::None,
                },
            ),
            MetadataArgsV2 {
                name: "Nft Name".to_string(),
                symbol: "NFT".to_string(),
                uri:"https://raw.githubusercontent.com/HongThaiPham/summer-bootcamp-anchor-token2022-stake/main/app/assets/token-info.json".to_string(),
                collection: Option::None,
                creators:vec![
                 
                Creator {
                    address: self.payer.key(),
                    verified: true,
                    share: 100,
                }
                ],
                is_mutable:true,
                primary_sale_happened:false,
                seller_fee_basis_points: 550,
                token_standard: Some(TokenStandard::NonFungible)
            },
            Option::None,
            Option::None,

        )?;
        Ok(())
    }
}
