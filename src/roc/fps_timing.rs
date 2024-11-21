use roc_std::roc_refcounted_noop_impl;
use roc_std::RocRefcounted;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum FpsTiming {
    Fps24 = 0,
    Fps25 = 1,
    Fps29 = 2,
    Fps30 = 3,
}

impl core::fmt::Debug for FpsTiming {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Fps24 => f.write_str("FpsTiming::Fps24"),
            Self::Fps25 => f.write_str("FpsTiming::Fps25"),
            Self::Fps29 => f.write_str("FpsTiming::Fps29"),
            Self::Fps30 => f.write_str("FpsTiming::Fps30"),
        }
    }
}

impl From<FpsTiming> for midly::Fps {
    fn from(s: FpsTiming) -> midly::Fps {
        match s {
            FpsTiming::Fps24 => midly::Fps::Fps24,
            FpsTiming::Fps25 => midly::Fps::Fps25,
            FpsTiming::Fps29 => midly::Fps::Fps29,
            FpsTiming::Fps30 => midly::Fps::Fps30,
        }
    }
}

roc_refcounted_noop_impl!(FpsTiming);
