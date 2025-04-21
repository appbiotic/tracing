#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

#[cfg(feature = "prost-serde")]
pub mod prost_serde;
