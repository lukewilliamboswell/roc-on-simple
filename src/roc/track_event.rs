use crate::roc::meta_message::MetaMessage;
use crate::roc::midi_message::MidiMessage;
use roc_std::roc_refcounted_noop_impl;
use roc_std::RocRefcounted;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TrackEventKindMidi {
    pub message: MidiMessage,
    pub channel: u8,
}

roc_refcounted_noop_impl!(TrackEventKindMidi);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum DiscriminantTrackEventKind {
    Escape = 0,
    Meta = 1,
    Midi = 2,
    SysEx = 3,
}

impl core::fmt::Debug for DiscriminantTrackEventKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Escape => f.write_str("TrackEventKind::Escape"),
            Self::Meta => f.write_str("TrackEventKind::Meta"),
            Self::Midi => f.write_str("TrackEventKind::Midi"),
            Self::SysEx => f.write_str("TrackEventKind::SysEx"),
        }
    }
}

roc_refcounted_noop_impl!(DiscriminantTrackEventKind);

#[repr(C, align(8))]
pub union UnionTrackEventKind {
    escape: core::mem::ManuallyDrop<roc_std::RocList<u8>>,
    meta: core::mem::ManuallyDrop<MetaMessage>,
    midi: TrackEventKindMidi,
    sys_ex: core::mem::ManuallyDrop<roc_std::RocList<u8>>,
}

impl TrackEventKind {
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> DiscriminantTrackEventKind {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, DiscriminantTrackEventKind>(*bytes.as_ptr().add(40))
        }
    }

    /// Internal helper
    #[allow(dead_code)]
    fn set_discriminant(&mut self, discriminant: DiscriminantTrackEventKind) {
        let discriminant_ptr: *mut DiscriminantTrackEventKind =
            (self as *mut TrackEventKind).cast();

        unsafe {
            *(discriminant_ptr.add(40)) = discriminant;
        }
    }
}

#[repr(C)]
pub struct TrackEventKind {
    payload: UnionTrackEventKind,
    discriminant: DiscriminantTrackEventKind,
}

impl Clone for TrackEventKind {
    fn clone(&self) -> Self {
        use DiscriminantTrackEventKind::*;

        let payload = unsafe {
            match self.discriminant {
                Escape => UnionTrackEventKind {
                    escape: self.payload.escape.clone(),
                },
                Meta => UnionTrackEventKind {
                    meta: self.payload.meta.clone(),
                },
                Midi => UnionTrackEventKind {
                    midi: self.payload.midi,
                },
                SysEx => UnionTrackEventKind {
                    sys_ex: self.payload.sys_ex.clone(),
                },
            }
        };

        Self {
            discriminant: self.discriminant,
            payload,
        }
    }
}

impl Drop for TrackEventKind {
    fn drop(&mut self) {
        // Drop the payloads
        match self.discriminant() {
            DiscriminantTrackEventKind::Escape => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.escape)
            },
            DiscriminantTrackEventKind::Meta => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.meta)
            },
            DiscriminantTrackEventKind::Midi => {}
            DiscriminantTrackEventKind::SysEx => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.sys_ex)
            },
        }
    }
}

impl roc_std::RocRefcounted for TrackEventKind {
    fn inc(&mut self) {
        unsafe {
            match self.discriminant() {
                DiscriminantTrackEventKind::Escape => (*self.payload.escape).inc(),
                DiscriminantTrackEventKind::Meta => (*self.payload.meta).inc(),
                DiscriminantTrackEventKind::Midi => {}
                DiscriminantTrackEventKind::SysEx => (*self.payload.sys_ex).inc(),
            }
        }
    }
    fn dec(&mut self) {
        unsafe {
            match self.discriminant() {
                DiscriminantTrackEventKind::Escape => (*self.payload.escape).dec(),
                DiscriminantTrackEventKind::Meta => (*self.payload.meta).dec(),
                DiscriminantTrackEventKind::Midi => {}
                DiscriminantTrackEventKind::SysEx => (*self.payload.sys_ex).dec(),
            }
        }
    }
    fn is_refcounted() -> bool {
        true
    }
}

impl<'a> From<&'a TrackEventKind> for midly::TrackEventKind<'a> {
    fn from(e: &'a TrackEventKind) -> midly::TrackEventKind<'a> {
        unsafe {
            match e.discriminant() {
                DiscriminantTrackEventKind::Escape => {
                    midly::TrackEventKind::Escape(e.payload.escape.as_slice())
                }
                DiscriminantTrackEventKind::Meta => {
                    midly::TrackEventKind::Meta((&*e.payload.meta).into())
                }
                DiscriminantTrackEventKind::Midi => midly::TrackEventKind::Midi {
                    channel: e.payload.midi.channel.into(),
                    message: e.payload.midi.message.into(),
                },
                DiscriminantTrackEventKind::SysEx => {
                    midly::TrackEventKind::SysEx(e.payload.sys_ex.as_slice())
                }
            }
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct TrackEvent {
    pub kind: TrackEventKind,
    pub delta: u32,
}

impl roc_std::RocRefcounted for TrackEvent {
    fn inc(&mut self) {
        self.kind.inc();
    }
    fn dec(&mut self) {
        self.kind.dec();
    }
    fn is_refcounted() -> bool {
        true
    }
}

impl<'a> From<&'a TrackEvent> for midly::TrackEvent<'a> {
    fn from(e: &'a TrackEvent) -> midly::TrackEvent<'a> {
        midly::TrackEvent {
            delta: e.delta.into(),
            kind: (&e.kind).into(),
        }
    }
}
