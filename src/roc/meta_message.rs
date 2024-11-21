use crate::roc::smpte_time::SmpteTime;
use roc_std::roc_refcounted_noop_impl;
use roc_std::RocRefcounted;

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct MetaMessageKeySignature {
    pub flats_or_sharps: i8,
    pub is_minor: u8,
}

roc_refcounted_noop_impl!(MetaMessageKeySignature);

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct MetaMessageTimeSignature {
    pub numerator: u8,
    pub denominator: u8,
    pub clocks_per_tick: u8,
    pub notes_32_per_quarter: u8,
}

roc_refcounted_noop_impl!(MetaMessageTimeSignature);

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum DiscriminantMaybeU16 {
    None = 0,
    Some = 1,
}

impl core::fmt::Debug for DiscriminantMaybeU16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::None => f.write_str("discriminant_MaybeU16::None"),
            Self::Some => f.write_str("discriminant_MaybeU16::Some"),
        }
    }
}

roc_refcounted_noop_impl!(DiscriminantMaybeU16);

#[derive(Clone, Copy)]
#[repr(C, align(2))]
pub union UnionMaybeU16 {
    none: (),
    some: u16,
}

#[repr(C)]
pub struct MetaMessage {
    payload: UnionMetaMessage,
    discriminant: DiscriminantMetaMessage,
}

impl Clone for MetaMessage {
    fn clone(&self) -> Self {
        use DiscriminantMetaMessage::*;

        let payload = unsafe {
            match self.discriminant {
                Copyright => UnionMetaMessage {
                    copyright: self.payload.copyright.clone(),
                },
                CuePoint => UnionMetaMessage {
                    cue_point: self.payload.cue_point.clone(),
                },
                DeviceName => UnionMetaMessage {
                    device_name: self.payload.device_name.clone(),
                },
                EndOfTrack => UnionMetaMessage { end_of_track: () },
                InstrumentName => UnionMetaMessage {
                    instrument_name: self.payload.instrument_name.clone(),
                },
                KeySignature => UnionMetaMessage {
                    key_signature: self.payload.key_signature,
                },
                Lyric => UnionMetaMessage {
                    lyric: self.payload.lyric.clone(),
                },
                Marker => UnionMetaMessage {
                    marker: self.payload.marker.clone(),
                },
                MidiChannel => UnionMetaMessage {
                    midi_channel: self.payload.midi_channel,
                },
                MidiPort => UnionMetaMessage {
                    midi_port: self.payload.midi_port,
                },
                ProgramName => UnionMetaMessage {
                    program_name: self.payload.program_name.clone(),
                },
                SequencerSpecific => UnionMetaMessage {
                    sequencer_specific: self.payload.sequencer_specific.clone(),
                },
                SmpteOffset => UnionMetaMessage {
                    smpte_offset: self.payload.smpte_offset,
                },
                Tempo => UnionMetaMessage {
                    tempo: self.payload.tempo,
                },
                Text => UnionMetaMessage {
                    text: self.payload.text.clone(),
                },
                TimeSignature => UnionMetaMessage {
                    time_signature: self.payload.time_signature,
                },
                TrackName => UnionMetaMessage {
                    track_name: self.payload.track_name.clone(),
                },
                TrackNumber => UnionMetaMessage {
                    track_number: self.payload.track_number,
                },
                Unknown => UnionMetaMessage {
                    unknown: self.payload.unknown.clone(),
                },
            }
        };

        Self {
            discriminant: self.discriminant,
            payload,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MaybeU16 {
    payload: UnionMaybeU16,
    discriminant: DiscriminantMaybeU16,
}

impl From<MaybeU16> for Option<u16> {
    fn from(value: MaybeU16) -> Option<u16> {
        unsafe {
            match value.discriminant {
                DiscriminantMaybeU16::None => None,
                DiscriminantMaybeU16::Some => Some(value.payload.some),
            }
        }
    }
}

roc_refcounted_noop_impl!(MaybeU16);

#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct MetaMessageUnknown {
    pub bytes: roc_std::RocList<u8>,
    pub raw_message_identifier: u8,
}

impl roc_std::RocRefcounted for MetaMessageUnknown {
    fn inc(&mut self) {
        self.bytes.inc();
    }
    fn dec(&mut self) {
        self.bytes.dec();
    }
    fn is_refcounted() -> bool {
        true
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum DiscriminantMetaMessage {
    Copyright = 0,
    CuePoint = 1,
    DeviceName = 2,
    EndOfTrack = 3,
    InstrumentName = 4,
    KeySignature = 5,
    Lyric = 6,
    Marker = 7,
    MidiChannel = 8,
    MidiPort = 9,
    ProgramName = 10,
    SequencerSpecific = 11,
    SmpteOffset = 12,
    Tempo = 13,
    Text = 14,
    TimeSignature = 15,
    TrackName = 16,
    TrackNumber = 17,
    Unknown = 18,
}

roc_refcounted_noop_impl!(DiscriminantMetaMessage);

#[repr(C, align(8))]
pub union UnionMetaMessage {
    copyright: core::mem::ManuallyDrop<roc_std::RocList<u8>>,
    cue_point: core::mem::ManuallyDrop<roc_std::RocList<u8>>,
    device_name: core::mem::ManuallyDrop<roc_std::RocList<u8>>,
    end_of_track: (),
    instrument_name: core::mem::ManuallyDrop<roc_std::RocList<u8>>,
    key_signature: MetaMessageKeySignature,
    lyric: core::mem::ManuallyDrop<roc_std::RocList<u8>>,
    marker: core::mem::ManuallyDrop<roc_std::RocList<u8>>,
    midi_channel: u8,
    midi_port: u8,
    program_name: core::mem::ManuallyDrop<roc_std::RocList<u8>>,
    sequencer_specific: core::mem::ManuallyDrop<roc_std::RocList<u8>>,
    smpte_offset: SmpteTime,
    tempo: u32,
    text: core::mem::ManuallyDrop<roc_std::RocList<u8>>,
    time_signature: MetaMessageTimeSignature,
    track_name: core::mem::ManuallyDrop<roc_std::RocList<u8>>,
    track_number: MaybeU16,
    unknown: core::mem::ManuallyDrop<MetaMessageUnknown>,
}

impl<'a> From<&'a MetaMessage> for midly::MetaMessage<'a> {
    fn from(s: &'a MetaMessage) -> midly::MetaMessage<'a> {
        use DiscriminantMetaMessage::*;

        unsafe {
            match s.discriminant {
                Copyright => midly::MetaMessage::Copyright(s.payload.copyright.as_slice()),
                CuePoint => midly::MetaMessage::CuePoint(s.payload.cue_point.as_slice()),
                DeviceName => midly::MetaMessage::DeviceName(s.payload.device_name.as_slice()),
                EndOfTrack => midly::MetaMessage::EndOfTrack,
                InstrumentName => {
                    midly::MetaMessage::InstrumentName(s.payload.instrument_name.as_slice())
                }
                KeySignature => midly::MetaMessage::KeySignature(
                    s.payload.key_signature.flats_or_sharps,
                    s.payload.key_signature.is_minor != 0,
                ),
                Lyric => midly::MetaMessage::Lyric(s.payload.lyric.as_slice()),
                Marker => midly::MetaMessage::Marker(s.payload.marker.as_slice()),
                MidiChannel => midly::MetaMessage::MidiChannel(s.payload.midi_channel.into()),
                MidiPort => midly::MetaMessage::MidiPort(s.payload.midi_port.into()),
                ProgramName => midly::MetaMessage::ProgramName(s.payload.program_name.as_slice()),
                SequencerSpecific => {
                    midly::MetaMessage::SequencerSpecific(s.payload.sequencer_specific.as_slice())
                }
                SmpteOffset => {
                    let time: Option<midly::SmpteTime> = s.payload.smpte_offset.into();
                    midly::MetaMessage::SmpteOffset(time.unwrap())
                }
                Tempo => midly::MetaMessage::Tempo(s.payload.tempo.into()),
                Text => midly::MetaMessage::Text(s.payload.text.as_slice()),
                TimeSignature => midly::MetaMessage::TimeSignature(
                    s.payload.time_signature.numerator,
                    s.payload.time_signature.denominator,
                    s.payload.time_signature.clocks_per_tick,
                    s.payload.time_signature.notes_32_per_quarter,
                ),
                TrackName => midly::MetaMessage::TrackName(s.payload.track_name.as_slice()),
                TrackNumber => midly::MetaMessage::TrackNumber(s.payload.track_number.into()),
                Unknown => midly::MetaMessage::Unknown(
                    s.payload.unknown.raw_message_identifier,
                    s.payload.unknown.bytes.as_slice(),
                ),
            }
        }
    }
}

impl Drop for MetaMessage {
    fn drop(&mut self) {
        // Drop the payloads
        match self.discriminant() {
            DiscriminantMetaMessage::Copyright => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.copyright)
            },
            DiscriminantMetaMessage::CuePoint => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.cue_point)
            },
            DiscriminantMetaMessage::DeviceName => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.device_name)
            },
            DiscriminantMetaMessage::EndOfTrack => {}
            DiscriminantMetaMessage::InstrumentName => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.instrument_name)
            },
            DiscriminantMetaMessage::KeySignature => {}
            DiscriminantMetaMessage::Lyric => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.lyric)
            },
            DiscriminantMetaMessage::Marker => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.marker)
            },
            DiscriminantMetaMessage::MidiChannel => {}
            DiscriminantMetaMessage::MidiPort => {}
            DiscriminantMetaMessage::ProgramName => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.program_name)
            },
            DiscriminantMetaMessage::SequencerSpecific => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.sequencer_specific)
            },
            DiscriminantMetaMessage::SmpteOffset => {}
            DiscriminantMetaMessage::Tempo => {}
            DiscriminantMetaMessage::Text => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.text)
            },
            DiscriminantMetaMessage::TimeSignature => {}
            DiscriminantMetaMessage::TrackName => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.track_name)
            },
            DiscriminantMetaMessage::TrackNumber => {}
            DiscriminantMetaMessage::Unknown => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.unknown)
            },
        }
    }
}

impl MetaMessage {
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> DiscriminantMetaMessage {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, DiscriminantMetaMessage>(*bytes.as_ptr().add(32))
        }
    }

    /// Internal helper
    #[allow(dead_code)]
    fn set_discriminant(&mut self, discriminant: DiscriminantMetaMessage) {
        let discriminant_ptr: *mut DiscriminantMetaMessage = (self as *mut MetaMessage).cast();

        unsafe {
            *(discriminant_ptr.add(32)) = discriminant;
        }
    }
}

impl roc_std::RocRefcounted for MetaMessage {
    fn inc(&mut self) {
        unsafe {
            match self.discriminant() {
                DiscriminantMetaMessage::Copyright => (*self.payload.copyright).inc(),
                DiscriminantMetaMessage::CuePoint => (*self.payload.cue_point).inc(),
                DiscriminantMetaMessage::DeviceName => (*self.payload.device_name).inc(),
                DiscriminantMetaMessage::EndOfTrack => {}
                DiscriminantMetaMessage::InstrumentName => (*self.payload.instrument_name).inc(),
                DiscriminantMetaMessage::KeySignature => {}
                DiscriminantMetaMessage::Lyric => (*self.payload.lyric).inc(),
                DiscriminantMetaMessage::Marker => (*self.payload.marker).inc(),
                DiscriminantMetaMessage::MidiChannel => {}
                DiscriminantMetaMessage::MidiPort => {}
                DiscriminantMetaMessage::ProgramName => (*self.payload.program_name).inc(),
                DiscriminantMetaMessage::SequencerSpecific => {
                    (*self.payload.sequencer_specific).inc()
                }
                DiscriminantMetaMessage::SmpteOffset => {}
                DiscriminantMetaMessage::Tempo => {}
                DiscriminantMetaMessage::Text => (*self.payload.text).inc(),
                DiscriminantMetaMessage::TimeSignature => {}
                DiscriminantMetaMessage::TrackName => (*self.payload.track_name).inc(),
                DiscriminantMetaMessage::TrackNumber => {}
                DiscriminantMetaMessage::Unknown => (*self.payload.unknown).inc(),
            }
        }
    }
    fn dec(&mut self) {
        unsafe {
            match self.discriminant() {
                DiscriminantMetaMessage::Copyright => (*self.payload.copyright).dec(),
                DiscriminantMetaMessage::CuePoint => (*self.payload.cue_point).dec(),
                DiscriminantMetaMessage::DeviceName => (*self.payload.device_name).dec(),
                DiscriminantMetaMessage::EndOfTrack => {}
                DiscriminantMetaMessage::InstrumentName => (*self.payload.instrument_name).dec(),
                DiscriminantMetaMessage::KeySignature => {}
                DiscriminantMetaMessage::Lyric => (*self.payload.lyric).dec(),
                DiscriminantMetaMessage::Marker => (*self.payload.marker).dec(),
                DiscriminantMetaMessage::MidiChannel => {}
                DiscriminantMetaMessage::MidiPort => {}
                DiscriminantMetaMessage::ProgramName => (*self.payload.program_name).dec(),
                DiscriminantMetaMessage::SequencerSpecific => {
                    (*self.payload.sequencer_specific).dec()
                }
                DiscriminantMetaMessage::SmpteOffset => {}
                DiscriminantMetaMessage::Tempo => {}
                DiscriminantMetaMessage::Text => (*self.payload.text).dec(),
                DiscriminantMetaMessage::TimeSignature => {}
                DiscriminantMetaMessage::TrackName => (*self.payload.track_name).dec(),
                DiscriminantMetaMessage::TrackNumber => {}
                DiscriminantMetaMessage::Unknown => (*self.payload.unknown).dec(),
            }
        }
    }
    fn is_refcounted() -> bool {
        true
    }
}
