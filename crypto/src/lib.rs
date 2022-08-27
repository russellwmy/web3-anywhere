mod ed25519;
mod errors;
mod key_type;
mod public_key;
mod secp256k1;
mod secret_key;
mod signature;

#[allow(dead_code)]
pub mod serialize;

pub use ed25519::{ED25519PublicKey, ED25519SecretKey};
pub use errors::{ParseKeyError, ParseKeyTypeError, ParseSignatureError};
pub use key_type::KeyType;
pub use public_key::PublicKey;
pub use secp256k1::{Secp256K1PublicKey, Secp256K1Signature};
pub use secret_key::SecretKey;
pub use signature::Signature;
