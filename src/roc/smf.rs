use crate::roc::header::Header;
use crate::roc::track_event::TrackEvent;

#[derive(Clone)]
#[repr(C)]
pub struct Smf {
    pub tracks: roc_std::RocList<roc_std::RocList<TrackEvent>>,
    pub header: Header,
}

impl roc_std::RocRefcounted for Smf {
    fn inc(&mut self) {
        self.tracks.inc();
    }
    fn dec(&mut self) {
        self.tracks.dec();
    }
    fn is_refcounted() -> bool {
        true
    }
}

impl<'a> From<&'a Smf> for midly::Smf<'a> {
    fn from(s: &'a Smf) -> midly::Smf<'a> {
        let tracks = s
            .tracks
            .into_iter()
            .map(|track| {
                track
                    .into_iter()
                    .map(|track_event| track_event.into())
                    .collect()
            })
            .collect();

        midly::Smf {
            tracks,
            header: s.header.into(),
        }
    }
}
