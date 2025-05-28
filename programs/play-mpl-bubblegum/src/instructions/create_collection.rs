use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{token_metadata_initialize, Mint, TokenAccount, TokenMetadataInitialize},
};

use crate::utils::update_account_lamports_to_minimum_balance;

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        signer,
        payer = signer,
        mint::token_program = token_program,
        mint::decimals = 0,
        mint::authority = signer,
        mint::freeze_authority = signer,
        extensions::metadata_pointer::authority = signer,
        extensions::metadata_pointer::metadata_address = mint,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        associated_token::token_program = token_program,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
}

impl<'info> CreateCollection<'info> {
    pub fn handler(&mut self, name: String, symbol: String, uri: String) -> Result<()> {
        self.initialize_token_metadata(name, symbol, uri)?;

        self.mint.reload()?;

        update_account_lamports_to_minimum_balance(
            self.mint.to_account_info(),
            self.signer.to_account_info(),
            self.system_program.to_account_info(),
        )?;

        Ok(())
    }

    fn initialize_token_metadata(&self, name: String, symbol: String, uri: String) -> Result<()> {
        let cpi_accounts = TokenMetadataInitialize {
            program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
            metadata: self.mint.to_account_info(), // metadata account is the mint, since data is stored in mint
            mint_authority: self.signer.to_account_info(),
            update_authority: self.signer.to_account_info(),
        };
        token_metadata_initialize(
            CpiContext::new(self.token_program.to_account_info(), cpi_accounts),
            name,
            symbol,
            uri,
        )?;
        Ok(())
    }
}
