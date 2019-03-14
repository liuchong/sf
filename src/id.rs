use super::defs::{MAX_SEQ, MAX_TS, MAX_WID, TS_SHIFT, WID_SHIFT};
use crate::errors::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Id(i64);

impl Id {
    /// New Id instance, context need to make sure these limits are satisfied.
    ///
    /// # Panics
    ///
    /// Panics if ts, wid, seq out of limit.
    pub fn new(ts: u64, wid: u16, seq: u16) -> Self {
        assert!(ts <= MAX_TS);
        assert!(wid <= MAX_WID);
        assert!(seq <= MAX_SEQ);

        Id((ts << TS_SHIFT) as i64
            | i64::from(wid << WID_SHIFT)
            | i64::from(seq))
    }

    #[cfg(feature = "const_fn")]
    pub const fn nil() -> Self {
        Id(0)
    }

    #[cfg(not(feature = "const_fn"))]
    pub fn nil() -> Id {
        Id(0)
    }

    /// From_i64 require a positive i64 input, or else it will panic.
    ///
    /// # Panics
    /// Panics if input value is not positive number.
    pub fn from_i64(i: i64) -> Self {
        assert!(i > 0);

        let u = i as u64;
        let ts = u >> TS_SHIFT;
        let wid = ((u >> WID_SHIFT) % (u64::from(MAX_WID) + 1)) as u16;
        let seq = (u % (u64::from(MAX_SEQ) + 1)) as u16;

        Id::new(ts, wid, seq)
    }

    pub fn from_slice(b: &[u8]) -> Result<Self, Error> {
        const BYTES_LEN: usize = 8;
        let len = b.len();

        if len != BYTES_LEN {
            return Err(Error::BytesError("bytes too short".to_owned()));
        }

        let i = i64::from(b[0]) << 56
            | i64::from(b[1]) << 48
            | i64::from(b[2]) << 40
            | i64::from(b[3]) << 32
            | i64::from(b[4]) << 24
            | i64::from(b[5]) << 16
            | i64::from(b[6]) << 8
            | i64::from(b[7]);

        Ok(Id(i))
    }

    /// As bytes Big-Endian.
    pub fn as_bytes(self) -> [u8; 8] {
        let b1: u8 = ((self.0 >> 56) & 0xff) as u8;
        let b2: u8 = ((self.0 >> 48) & 0xff) as u8;
        let b3: u8 = ((self.0 >> 40) & 0xff) as u8;
        let b4: u8 = ((self.0 >> 32) & 0xff) as u8;
        let b5: u8 = ((self.0 >> 24) & 0xff) as u8;
        let b6: u8 = ((self.0 >> 16) & 0xff) as u8;
        let b7: u8 = ((self.0 >> 8) & 0xff) as u8;
        let b8: u8 = (self.0 & 0xff) as u8;

        [b1, b2, b3, b4, b5, b6, b7, b8]
    }
}

impl fmt::LowerHex for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl fmt::UpperHex for Id {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl Into<i64> for Id {
    fn into(self) -> i64 {
        self.0
    }
}

impl<'a> Into<i64> for &'a Id {
    fn into(self) -> i64 {
        self.0
    }
}

impl Into<Id> for i64 {
    fn into(self) -> Id {
        Id(self)
    }
}

impl<'a> Into<Id> for &'a i64 {
    fn into(self) -> Id {
        Id(*self)
    }
}

impl Default for Id {
    #[inline]
    fn default() -> Self {
        Id::nil()
    }
}

#[cfg(test)]
mod tests {
    use super::Id;

    #[test]
    #[should_panic]
    fn test_bad_worker_id() {
        let _id = Id::new(0, 1024, 0);
    }

    #[test]
    fn test_id_nil() {
        let id = Id::nil();

        assert_eq!(id, 0.into());
    }

    #[test]
    fn test_id_from_i64_success() {
        let i = 123;
        let id = Id::from_i64(i);

        assert_eq!(i, id.into());
    }

    #[test]
    #[should_panic]
    fn test_id_from_i64_fail() {
        let i: i64 = -1;
        let _id = Id::from_i64(i);
    }
}
