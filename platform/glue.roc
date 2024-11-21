platform ""
    requires {} { main : _ }
    exposes []
    packages {}
    imports []
    provides [mainForHost]

FpsTiming : [Fps24, Fps25, Fps29, Fps30]
Format : [SingleTrack, Parallel, Sequential]
Timing : [Metrical U16, Timecode FpsTiming U8]

Header : {
    format : Format,
    timing : Timing,
}

MidiMessage : [
    NoteOff {
        key: U8,
        vel: U8,
    },
    NoteOn {
        key: U8,
        vel: U8,
    },
    Aftertouch {
        key: U8,
        vel: U8,
    },
    Controller {
        controller: U8,
        value: U8,
    },
    ProgramChange {
        program: U8,
    },
    ChannelAftertouch {
        vel: U8,
    },
    PitchBend {
        bend: U16,
    },
]

Bytes : List U8

SmpteTime : {
    hour : U8,
    minute : U8,
    second : U8,
    frame : U8,
    subframe : U8,
    fps : FpsTiming,
}

MaybeU16 : [None, Some U16]

MetaMessage : [
    TrackNumber MaybeU16,
    Text Bytes,
    Copyright Bytes,
    TrackName Bytes,
    InstrumentName Bytes,
    Lyric Bytes,
    Marker Bytes,
    CuePoint Bytes,
    ProgramName Bytes,
    DeviceName Bytes,
    MidiChannel U8,
    MidiPort U8,
    EndOfTrack,
    Tempo U32,
    SmpteOffset SmpteTime,
    TimeSignature U8 U8 U8 U8,
    KeySignature I8 U8,
    SequencerSpecific Bytes,
    Unknown U8 Bytes,
]

TrackEventKindMidi : {
    channel: U8,
    message: MidiMessage,
}

TrackEventKind : [
    Midi TrackEventKindMidi,
    SysEx Bytes,
    Escape Bytes,
    Meta MetaMessage,
]

TrackEvent : {
    delta : U32,
    kind : TrackEventKind,
}

Track : List TrackEvent

Smf : {
    header : Header,
    tracks : List Track,
}

mainForHost : Smf
mainForHost = main
