pub const TS_BITS: i32 = 41; // Bits of timestamp
pub const WID_BITS: i32 = 10; // Bits of worker id
pub const SEQ_BITS: i32 = 12; // Bits of sequence

pub const TS_SHIFT: i32 = SEQ_BITS + WID_BITS; // timestamp left shift
pub const WID_SHIFT: i32 = SEQ_BITS; //worker id left shift

pub const MAX_TS: u64 = (-1 as i64 ^ (-1 << TS_BITS)) as u64; // maximize timestamp value
pub const MAX_WID: u16 = (-1 as i16 ^ (-1 << WID_BITS)) as u16; // maximize worker id value
pub const MAX_SEQ: u16 = (-1 as i16 ^ (-1 << SEQ_BITS)) as u16; // maximize sequence value
