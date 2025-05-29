use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;

use crate::bubblegum::types::{Creator, MetadataArgsV2, TokenStandard};
use crate::utils::{MplBubblegum, MplCore, Noop, SplAccountCompression};

use crate::bubblegum::cpi::{accounts::MintV2, mint_v2};
use crate::MPL_CORE_CPI_SIGNER_PREFIX;

#[derive(Accounts)]
pub struct MintNftToCollection<'info> {
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
    /// CHECK: will used by mpl_core program
    #[account(
        mut,
        owner = mpl_core_program.key(),
    )]
    pub collection: AccountInfo<'info>,
    /// CHECK: will used by mpl_core program
    #[account(
        seeds = [<str as AsRef<[u8]>>::as_ref(MPL_CORE_CPI_SIGNER_PREFIX)],
        bump,
        seeds::program = mpl_bubblegum_program.key()
    )]
    pub mpl_core_cpi_signer: UncheckedAccount<'info>,
    pub mpl_bubblegum_program: Program<'info, MplBubblegum>,
    pub mpl_core_program: Program<'info, MplCore>,
    pub spl_compression_program: Program<'info, SplAccountCompression>,
    pub noop_program: Program<'info, Noop>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintNftToCollection<'info> {
    pub fn handler(&mut self) -> Result<()> {
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
                    collection_authority: Option::Some(self.payer.to_account_info()),
                    core_collection: Option::Some(self.collection.to_account_info()),
                    leaf_delegate: Option::None,
                    mpl_core_cpi_signer: Option::Some(self.mpl_core_cpi_signer.to_account_info()),
                    tree_delegate: Option::None,
                },
            ),
            MetadataArgsV2 {
                name: "Nft in Collection".to_string(),
                symbol: "NFT".to_string(),
                uri:"https://raw.githubusercontent.com/HongThaiPham/summer-bootcamp-anchor-token2022-stake/main/app/assets/token-info.json".to_string(),
                collection: Option::Some(self.collection.to_account_info().key()),
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
