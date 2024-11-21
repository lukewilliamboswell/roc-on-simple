use crate::roc::fps_timing::FpsTiming;
use roc_std::roc_refcounted_noop_impl;
use roc_std::RocRefcounted;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct SmpteTime {
    pub fps: FpsTiming,
    pub frame: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub subframe: u8,
}

impl From<SmpteTime> for Option<midly::SmpteTime> {
    fn from(s: SmpteTime) -> Option<midly::SmpteTime> {
        midly::SmpteTime::new(
            s.hour,
            s.minute,
            s.second,
            s.frame,
            s.subframe,
            s.fps.into(),
        )
    }
}

roc_refcounted_noop_impl!(SmpteTime);
