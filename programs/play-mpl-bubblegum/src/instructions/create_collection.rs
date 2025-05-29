use anchor_lang::prelude::*;

use crate::utils::MplCore;
use mpl_core::instructions::{CreateCollectionV2CpiBuilder};
use mpl_core::types::{PluginAuthorityPair, Plugin, BubblegumV2};


#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Collection account will be use in mpl_core program
    #[account(
        mut, 
        constraint = collection.data_is_empty() == true,
    )]
    pub collection: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub mpl_core_program: Program<'info, MplCore>,
}

impl<'info> CreateCollection<'info> {
    pub fn handler(&mut self, name: String,  uri: String) -> Result<()> {
        CreateCollectionV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.collection.to_account_info())
            .name(name)
            .uri(uri)
            .payer(&self.signer.to_account_info())
            .update_authority(Option::Some(&self.signer.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .plugins(vec![
                PluginAuthorityPair {
                    plugin: Plugin::BubblegumV2(BubblegumV2 {}),
                    authority: None,
                }
            ])
            .invoke()?;
        Ok(())
    }

 
}
