use crate::errors::Error;
use crate::id::Id;
use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use std::fmt;
use std::str::FromStr;

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base64_str = encode_config(&self.as_bytes(), URL_SAFE_NO_PAD);
        write!(f, "{}", base64_str)
    }
}

impl FromStr for Id {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match decode_config(s, URL_SAFE_NO_PAD) {
            Ok(ref bytes) => Id::from_slice(&bytes),
            _ => Err(Error::ParseError),
        }
    }
}
