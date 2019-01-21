use super::config::{MAX_SEQ, MAX_TS, MAX_WID, TS_SHIFT, WID_SHIFT};
use super::Sf;
use std::{fmt, str};

impl Sf {
    pub fn new(ts: u64, wid: u16, seq: u16) -> Result<Self, String> {
        if ts > MAX_TS {
            return Err(format!("bad timestamp {}", ts));
        }
        if wid > MAX_WID {
            return Err(format!("bad worker id {}", wid));
        }
        if seq > MAX_SEQ {
            return Err(format!("bad sequence {}", seq));
        }

        Ok(Sf((ts << TS_SHIFT)
            | u64::from(wid << WID_SHIFT)
            | u64::from(seq)))
    }

    #[cfg(feature = "const_fn")]
    pub const fn nil() -> Self {
        Sf(0)
    }

    #[cfg(not(feature = "const_fn"))]
    pub fn nil() -> Sf {
        Sf(0)
    }
}

/* TODO */
impl fmt::Display for Sf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::LowerHex for Sf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl fmt::UpperHex for Sf {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

/* TODO */
impl str::FromStr for Sf {
    type Err = String;

    fn from_str(_sf_str: &str) -> Result<Self, Self::Err> {
        Ok(Sf::nil())
    }
}

impl Into<u64> for Sf {
    fn into(self) -> u64 {
        self.0
    }
}

impl Default for Sf {
    #[inline]
    fn default() -> Self {
        Sf::nil()
    }
}
