use solana_program::pubkey::Pubkey;

use crate::utils::{guards::require_gt, math::mul_div_floor};
use crate::{VaultError, VaultResult};

#[derive(Default)]
pub struct Vault {
    pub admin: Pubkey,
    pub shares_mint: Pubkey,
    pub assets_mint: Pubkey,

    pub shares: u64,
    pub assets: u64,

    pub fee_bps: u64,
    pub fee_token_account: Pubkey,
}

impl Vault {
    pub fn new() -> Self {
        Vault::default()
    }

    pub fn num_shares(&self) -> u64 {
        self.shares
    }

    pub fn num_assets(&self) -> u64 {
        self.assets
    }

    pub fn is_solvent(&self) -> bool {
        self.num_shares() <= self.num_assets()
    }

    pub fn check_invariant(&self) -> VaultResult<()> {
        self.is_solvent().then_some(()).ok_or(VaultError::GuardFail)
    }

    pub fn convert_shares_to_assets(&self, shares: u64) -> VaultResult<u64> {
        let assets = if self.num_shares() == self.num_assets() {
            shares
        } else {
            mul_div_floor(shares, self.assets, self.shares)?
        };
        Ok(assets)
    }

    pub fn convert_assets_to_shares(&self, token: u64) -> VaultResult<u64> {
        let shares = if self.num_shares() == self.num_assets() {
            token
        } else {
            mul_div_floor(token, self.shares, self.assets)?
        };

        Ok(shares)
    }
    
    pub fn burn_shares(&mut self, amt: u64) -> VaultResult<()> {
        self.shares = self
            .shares
            .checked_sub(amt)
            .ok_or(VaultError::MathOverflow)?;
        Ok(())
    }

    pub fn mint_shares(&mut self, amt: u64) -> VaultResult<()> {
        require_gt!(amt, 0);
        self.shares = self
            .shares
            .checked_add(amt)
            .ok_or(VaultError::MathOverflow)?;
        Ok(())
    }

    pub fn add_token(&mut self, amt: u64) -> VaultResult<()> {
        require_gt!(amt, 0);
        self.assets = self
            .assets
            .checked_add(amt)
            .ok_or(VaultError::MathOverflow)?;
        Ok(())
    }

    pub fn del_token(&mut self, amt: u64) -> VaultResult<()> {
        self.assets = self
            .assets
            .checked_sub(amt)
            .ok_or(VaultError::MathOverflow)?;

        // ensure that vault is still solvent after slashing
        self.check_invariant()?;
        Ok(())
    }
}
