use sp_core::{Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

#[cfg(feature = "testnet")]
pub mod testnet;
#[cfg(feature = "testnet")]
pub use testnet::*;

#[cfg(feature = "mainnet")]
pub mod mainnet;
#[cfg(feature = "mainnet")]
pub use mainnet::*;

pub type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{seed}"), None)
		.expect("static values are valid;")
		.public()
}
