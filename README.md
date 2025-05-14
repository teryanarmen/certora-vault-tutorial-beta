# Certora Vault Example Tutorial

This tutorial builds a simple Tokenized Vault, inspired by the  [ERC-4626
TOKENIZED VAULT STANDARD](https://eips.ethereum.org/EIPS/eip-4626) on Ethereum.

## Vault Invariant

The vault maintains the following invariant to ensure proportional ownership:

$$
\frac{\text{shares}_{\text{pre}}}{\text{assets}_{\text{pre}}} \geq \frac{\text{shares}_{\text{post}}}{\text{assets}_{\text{post}}}
$$

This ensures that the ratio of shares to assets does not increase after any operation.

## DISCLAIMER
The code and examples provided in this repository are for educational purposes only. They are not production-ready and may contain bugs or security vulnerabilities. Use at your own risk.