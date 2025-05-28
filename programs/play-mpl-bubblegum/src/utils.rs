use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction::transfer},
};
use anchor_spl::token_interface::spl_token_2022::{
    extension::{BaseStateWithExtensions, Extension, StateWithExtensions},
    state::Mint,
};
use bytemuck::Pod;
use spl_type_length_value::variable_len_pack::VariableLenPack;

#[derive(Clone)]
pub struct MplBubblegum;
impl Id for MplBubblegum {
    fn id() -> Pubkey {
        pubkey!("BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY")
    }
}

#[derive(Clone)]
pub struct Noop;
impl Id for Noop {
    fn id() -> Pubkey {
        pubkey!("mnoopTCrg4p8ry25e4bcWA9XZjbNjMTfgYVGGEdRsf3")
    }
}

#[derive(Clone)]
pub struct SplAccountCompression;
impl Id for SplAccountCompression {
    fn id() -> Pubkey {
        pubkey!("mcmt6YrQEMKw8Mw43FmpRLmf7BqRnFMKmAcbxE3xkAW")
    }
}

#[derive(Clone)]
pub struct MplCore;
impl Id for MplCore {
    fn id() -> Pubkey {
        pubkey!("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d")
    }
}

pub fn update_account_lamports_to_minimum_balance<'info>(
    account: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()> {
    let extra_lamports = Rent::get()?.minimum_balance(account.data_len()) - account.get_lamports();
    if extra_lamports > 0 {
        invoke(
            &transfer(payer.key, account.key, extra_lamports),
            &[payer, account, system_program],
        )?;
    }
    Ok(())
}

pub fn get_mint_extensible_extension_data<T: Extension + VariableLenPack>(
    account: &mut AccountInfo,
) -> Result<T> {
    let mint_data = account.data.borrow();
    let mint_with_extension = StateWithExtensions::<Mint>::unpack(&mint_data)?;
    let extension_data = mint_with_extension.get_variable_len_extension::<T>()?;
    Ok(extension_data)
}

pub fn get_mint_extension_data<T: Extension + Pod>(account: &mut AccountInfo) -> Result<T> {
    let mint_data = account.data.borrow();
    let mint_with_extension = StateWithExtensions::<Mint>::unpack(&mint_data)?;
    let extension_data = *mint_with_extension.get_extension::<T>()?;
    Ok(extension_data)
}
