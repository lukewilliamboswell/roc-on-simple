use roc_std::roc_refcounted_noop_impl;
use roc_std::RocRefcounted;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct TimingTimecode {
    pub f0: crate::roc::fps_timing::FpsTiming,
    pub f1: u8,
}

roc_refcounted_noop_impl!(TimingTimecode);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum DiscriminantTiming {
    Metrical = 0,
    Timecode = 1,
}

impl core::fmt::Debug for DiscriminantTiming {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Metrical => f.write_str("discriminant_Timing::Metrical"),
            Self::Timecode => f.write_str("discriminant_Timing::Timecode"),
        }
    }
}

roc_refcounted_noop_impl!(DiscriminantTiming);

#[derive(Clone, Copy)]
#[repr(C, align(2))]
pub union UnionTiming {
    metrical: u16,
    timecode: TimingTimecode,
}

impl Timing {
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> DiscriminantTiming {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, DiscriminantTiming>(*bytes.as_ptr().add(2))
        }
    }

    /// Internal helper
    #[allow(dead_code)]
    fn set_discriminant(&mut self, discriminant: DiscriminantTiming) {
        let discriminant_ptr: *mut DiscriminantTiming = (self as *mut Timing).cast();

        unsafe {
            *(discriminant_ptr.add(2)) = discriminant;
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Timing {
    payload: UnionTiming,
    discriminant: DiscriminantTiming,
}

impl From<Timing> for midly::Timing {
    fn from(t: Timing) -> midly::Timing {
        unsafe {
            match t.discriminant {
                DiscriminantTiming::Metrical => midly::Timing::Metrical(t.payload.metrical.into()),
                DiscriminantTiming::Timecode => {
                    midly::Timing::Timecode(t.payload.timecode.f0.into(), t.payload.timecode.f1)
                }
            }
        }
    }
}

roc_refcounted_noop_impl!(Timing);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum Format {
    Parallel = 0,
    Sequential = 1,
    SingleTrack = 2,
}

impl core::fmt::Debug for Format {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Parallel => f.write_str("Format::Parallel"),
            Self::Sequential => f.write_str("Format::Sequential"),
            Self::SingleTrack => f.write_str("Format::SingleTrack"),
        }
    }
}

impl From<Format> for midly::Format {
    fn from(f: Format) -> midly::Format {
        match f {
            Format::Parallel => midly::Format::Parallel,
            Format::Sequential => midly::Format::Sequential,
            Format::SingleTrack => midly::Format::SingleTrack,
        }
    }
}

roc_refcounted_noop_impl!(Format);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Header {
    pub timing: Timing,
    pub format: Format,
}

impl From<Header> for midly::Header {
    fn from(h: Header) -> midly::Header {
        midly::Header {
            timing: h.timing.into(),
            format: h.format.into(),
        }
    }
}

roc_refcounted_noop_impl!(Header);
