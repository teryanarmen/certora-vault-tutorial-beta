use crate::errors::{VaultError, VaultResult};

#[cfg(not(feature = "certora"))]
mod inner {
    use super::*;

    pub fn mul_div_floor(a: u64, b: u64, c: u64) -> VaultResult<u64> {
        (a as u128)
            .checked_mul(b as u128)
            .ok_or(VaultError::MathOverflow)?
            .checked_div(c as u128)
            .ok_or(VaultError::MathOverflow)?
            .try_into()
            .map_err(|_| VaultError::MathOverflow)
    }

    pub fn mul_div_ceil(a: u64, b: u64, c: u64) -> VaultResult<u64> {
        (a as u128)
            .checked_mul(b as u128)
            .ok_or(VaultError::MathOverflow)?
            .div_ceil(c as u128)
            .try_into()
            .map_err(|_| VaultError::MathOverflow)
    }
}

#[cfg(feature = "certora")]
mod inner {
    use super::*;
    use cvlr::mathint::NativeInt;

    pub fn mul_div_floor(a: u64, b: u64, c: u64) -> VaultResult<u64> {
        let a = NativeInt::from(a);
        let b = NativeInt::from(b);
        let c = NativeInt::from(c);

        let res = a.muldiv(b, c);

        if res.is_u64() {
            Ok(u64::from(res))
        } else {
            Err(VaultError::MathOverflow)
        }
    }

    pub fn mul_div_ceil(a: u64, b: u64, c: u64) -> VaultResult<u64> {
        let a = NativeInt::from(a);
        let b = NativeInt::from(b);
        let c = NativeInt::from(c);
        let res = a.muldiv_ceil(b, c);

        if res.is_u64() {
            Ok(u64::from(res))
        } else {
            Err(VaultError::MathOverflow)
        }
    }
}

pub use inner::*;
