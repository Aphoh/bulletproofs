#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "docs", doc = include_str!("../README.md"))]

extern crate alloc;

mod util;

#[cfg_attr(feature = "docs", doc = include_str!("../docs/notes-intro.md"))]
mod notes {
    #[cfg_attr(feature = "docs", doc = include_str!("../docs/notes-ipp.md"))]
    mod inner_product_proof {}
    #[cfg_attr(feature = "docs", doc = include_str!("../docs/notes-rp.md"))]
    mod range_proof {}
    #[cfg_attr(feature = "docs", doc = include_str!("../docs/notes-r1cs.md"))]
    mod r1cs_proof {}
}

pub mod errors;
pub mod generators;
pub mod inner_product_proof;
pub mod range_proof;
pub mod transcript;

pub use crate::errors::ProofError;
pub use crate::generators::{BulletproofGens, BulletproofGensShare, PedersenGens};
pub use crate::range_proof::RangeProof;

#[cfg_attr(feature = "docs", doc = include_str!("../docs/aggregation-api.md"))]
pub mod range_proof_mpc {
    pub use crate::errors::MPCError;
    pub use crate::range_proof::dealer;
    pub use crate::range_proof::messages;
    pub use crate::range_proof::party;
}

#[cfg(feature = "yoloproofs")]
#[cfg(feature = "std")]
pub mod r1cs;
