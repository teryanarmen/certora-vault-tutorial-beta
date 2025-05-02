#[derive(Debug)]
pub enum VaultError {
    Unspecified,
    MathOverflow,
    GuardFail,
}

// Define a custom Result type
pub type VaultResult<T> = std::result::Result<T, VaultError>;
