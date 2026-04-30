#![cfg_attr(not(feature = "std"), no_std)]

mod arithmetic;
mod curve;
pub mod ff_ext;
pub mod fft;
pub mod hash_to_curve;
pub mod msm;
pub mod serde;

#[cfg(feature = "bls12-381")]
pub mod bls12381;

#[cfg(feature = "bn256")]
pub mod bn256;

#[cfg(feature = "grumpkin")]
pub mod grumpkin;

#[cfg(feature = "pasta")]
pub mod pasta;

#[cfg(feature = "pluto-eris")]
pub mod pluto_eris;

#[cfg(feature = "secp256k1")]
pub mod secp256k1;

#[cfg(feature = "secp256r1")]
pub mod secp256r1;

#[cfg(feature = "secq256k1")]
pub mod secq256k1;

#[cfg(feature = "t256")]
pub mod t256;

#[macro_use]
mod derive;

// Re-export to simplify downstream dependencies.
pub use curve::{Coordinates, CurveAffine, CurveExt};
pub use ff;
pub use group;
pub use pairing;

#[cfg(test)]
pub mod tests;
