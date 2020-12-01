#[cfg(test)]
mod test;

mod algorithm;
mod client;
mod error;
mod jwk;
mod key_provider;
mod token;

pub use crate::client::{AsyncClient, Client};
pub use crate::token::{IdPayload, RequiredClaims, Token};
pub use error::Error;

fn base64_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode_config(&input, base64::URL_SAFE)
}
