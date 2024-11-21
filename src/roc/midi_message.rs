use roc_std::roc_refcounted_noop_impl;
use roc_std::RocRefcounted;

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct AfterTouchData {
    pub key: u8,
    pub vel: u8,
}

roc_refcounted_noop_impl!(AfterTouchData);

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(transparent)]
pub struct ChannelAftertouchData {
    pub vel: u8,
}

roc_refcounted_noop_impl!(ChannelAftertouchData);

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct ControllerData {
    pub controller: u8,
    pub value: u8,
}

roc_refcounted_noop_impl!(ControllerData);

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(transparent)]
pub struct PitchBendData {
    pub bend: u16,
}

roc_refcounted_noop_impl!(PitchBendData);

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(transparent)]
pub struct ProgramChangeData {
    pub program: u8,
}

roc_refcounted_noop_impl!(ProgramChangeData);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum DiscriminantMidiMessage {
    Aftertouch = 0,
    ChannelAftertouch = 1,
    Controller = 2,
    NoteOff = 3,
    NoteOn = 4,
    PitchBend = 5,
    ProgramChange = 6,
}

impl core::fmt::Debug for DiscriminantMidiMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Aftertouch => f.write_str("discriminant_MidiMessage::Aftertouch"),
            Self::ChannelAftertouch => f.write_str("discriminant_MidiMessage::ChannelAftertouch"),
            Self::Controller => f.write_str("discriminant_MidiMessage::Controller"),
            Self::NoteOff => f.write_str("discriminant_MidiMessage::NoteOff"),
            Self::NoteOn => f.write_str("discriminant_MidiMessage::NoteOn"),
            Self::PitchBend => f.write_str("discriminant_MidiMessage::PitchBend"),
            Self::ProgramChange => f.write_str("discriminant_MidiMessage::ProgramChange"),
        }
    }
}

roc_refcounted_noop_impl!(DiscriminantMidiMessage);

#[derive(Clone, Copy)]
#[repr(C, align(2))]
pub union UnionMidiMessage {
    aftertouch: AfterTouchData,
    channel_aftertouch: ChannelAftertouchData,
    controller: ControllerData,
    note_off: AfterTouchData,
    note_on: AfterTouchData,
    pitch_bend: PitchBendData,
    program_change: ProgramChangeData,
}

impl MidiMessage {
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> DiscriminantMidiMessage {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, DiscriminantMidiMessage>(*bytes.as_ptr().add(2))
        }
    }

    /// Internal helper
    #[allow(dead_code)]
    fn set_discriminant(&mut self, discriminant: DiscriminantMidiMessage) {
        let discriminant_ptr: *mut DiscriminantMidiMessage = (self as *mut MidiMessage).cast();

        unsafe {
            *(discriminant_ptr.add(2)) = discriminant;
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MidiMessage {
    payload: UnionMidiMessage,
    discriminant: DiscriminantMidiMessage,
}

impl core::fmt::Debug for MidiMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use DiscriminantMidiMessage::*;

        unsafe {
            match self.discriminant {
                Aftertouch => {
                    let field: &AfterTouchData = &self.payload.aftertouch;
                    f.debug_tuple("MidiMessage::Aftertouch")
                        .field(field)
                        .finish()
                }
                ChannelAftertouch => {
                    let field: &ChannelAftertouchData = &self.payload.channel_aftertouch;
                    f.debug_tuple("MidiMessage::ChannelAftertouch")
                        .field(field)
                        .finish()
                }
                Controller => {
                    let field: &ControllerData = &self.payload.controller;
                    f.debug_tuple("MidiMessage::Controller")
                        .field(field)
                        .finish()
                }
                NoteOff => {
                    let field: &AfterTouchData = &self.payload.note_off;
                    f.debug_tuple("MidiMessage::NoteOff").field(field).finish()
                }
                NoteOn => {
                    let field: &AfterTouchData = &self.payload.note_on;
                    f.debug_tuple("MidiMessage::NoteOn").field(field).finish()
                }
                PitchBend => {
                    let field: &PitchBendData = &self.payload.pitch_bend;
                    f.debug_tuple("MidiMessage::PitchBend")
                        .field(field)
                        .finish()
                }
                ProgramChange => {
                    let field: &ProgramChangeData = &self.payload.program_change;
                    f.debug_tuple("MidiMessage::ProgramChange")
                        .field(field)
                        .finish()
                }
            }
        }
    }
}

roc_refcounted_noop_impl!(MidiMessage);

impl From<MidiMessage> for midly::MidiMessage {
    fn from(msg: MidiMessage) -> midly::MidiMessage {
        use DiscriminantMidiMessage::*;

        unsafe {
            match msg.discriminant {
                Aftertouch => midly::MidiMessage::Aftertouch {
                    key: msg.payload.aftertouch.key.into(),
                    vel: msg.payload.aftertouch.vel.into(),
                },
                ChannelAftertouch => midly::MidiMessage::ChannelAftertouch {
                    vel: msg.payload.channel_aftertouch.vel.into(),
                },
                Controller => midly::MidiMessage::Controller {
                    controller: msg.payload.controller.controller.into(),
                    value: msg.payload.controller.value.into(),
                },
                NoteOff => midly::MidiMessage::NoteOff {
                    key: msg.payload.note_off.key.into(),
                    vel: msg.payload.note_off.vel.into(),
                },
                NoteOn => midly::MidiMessage::NoteOn {
                    key: msg.payload.note_on.key.into(),
                    vel: msg.payload.note_on.vel.into(),
                },
                PitchBend => midly::MidiMessage::PitchBend {
                    bend: midly::PitchBend(msg.payload.pitch_bend.bend.into()),
                },
                ProgramChange => midly::MidiMessage::ProgramChange {
                    program: msg.payload.program_change.program.into(),
                },
            }
        }
    }
}
