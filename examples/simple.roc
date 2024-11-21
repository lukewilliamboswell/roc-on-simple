app [main] { pf: platform "../platform/main.roc" }

main = \{} -> {
    header: {
        format: SingleTrack,
        timing: Metrical 10,
    },
    tracks: [
        [
            { delta: 1, kind: Escape [] },
            {
                delta: 2,
                kind: Midi {
                    channel: 1,
                    message: NoteOn { key: 1, vel: 2 },
                },
            },
            {
                delta: 3,
                kind: Meta EndOfTrack,
            },
        ],
    ],
}
