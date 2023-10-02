use itertools::Itertools;

use super::{MultiNote, NoteName, OctavedNote};

#[derive(Debug, Clone, Copy)]
pub struct SatbBlock(
    pub OctavedNote,
    pub OctavedNote,
    pub OctavedNote,
    pub OctavedNote,
);

impl std::fmt::Display for SatbBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}

pub fn permute(notes: &super::SatbTemplate) -> Vec<SatbBlock> {
    let notes = match notes.accord {
        MultiNote::Triad(n1, n2, n3) => (n1, n2, n3, notes.bass),
        MultiNote::Quatrain(_, n2, n3, n4) => (n2, n3, n4, notes.bass),
    };

    [notes.0, notes.1, notes.2]
        .iter()
        .permutations(3)
        .unique()
        .map(|permutation| (*permutation[0], *permutation[1], *permutation[2], notes.3))
        .flat_map(|(s, a, t, b)| {
            let mut sopran_res = Vec::with_capacity(3);

            sopran_res.push((OctavedNote::new(s, 1), a, t, b));

            match s {
                NoteName::Gis
                | NoteName::As
                | NoteName::A
                | NoteName::Ais
                | NoteName::B
                | NoteName::H
                | NoteName::Ces => {}
                _ => sopran_res.push((OctavedNote::new(s, 2), a, t, b)),
            }

            sopran_res
        })
        .flat_map(|(s, a, t, b)| {
            let mut alto_res = Vec::with_capacity(3);

            alto_res.push((s, OctavedNote::new(a, 1), t, b));

            match a {
                NoteName::Cis
                | NoteName::Des
                | NoteName::D
                | NoteName::Dis
                | NoteName::Es
                | NoteName::E
                | NoteName::F
                | NoteName::Eis
                | NoteName::Fis
                | NoteName::Ges => {}
                NoteName::C => alto_res.push((s, OctavedNote::new(a, 2), t, b)),
                _ => alto_res.push((s, OctavedNote::new(a, 0), t, b)),
            }

            alto_res
        })
        .flat_map(|(s, a, t, b)| {
            let mut tenor_res = Vec::with_capacity(3);

            tenor_res.push((s, a, OctavedNote::new(t, 0), b));

            match t {
                NoteName::Gis
                | NoteName::As
                | NoteName::A
                | NoteName::Ais
                | NoteName::B
                | NoteName::H
                | NoteName::Ces => {}
                _ => tenor_res.push((s, a, OctavedNote::new(t, 1), b)),
            }

            tenor_res
        })
        .flat_map(|(s, a, t, b)| {
            let mut bass_res = Vec::with_capacity(3);

            bass_res.push((s, a, t, OctavedNote::new(b, 0)));

            match b {
                NoteName::Cis
                | NoteName::Des
                | NoteName::D
                | NoteName::Dis
                | NoteName::Es
                | NoteName::E
                | NoteName::F
                | NoteName::Eis
                | NoteName::Fis
                | NoteName::Ges => {}
                NoteName::C => bass_res.push((s, a, t, OctavedNote::new(b, 1))),
                _ => bass_res.push((s, a, t, OctavedNote::new(b, -1))),
            }

            bass_res
        })
        .map(|(s, a, t, b)| SatbBlock(s, a, t, b))
        .collect_vec()
}

#[test]
fn test_permutes() {
    assert_eq!(
        permute(&crate::logic::notes::SatbTemplate {
            accord: MultiNote::Triad(NoteName::Cis, NoteName::D, NoteName::F),
            bass: NoteName::Cis
        })
        .len(),
        24,
    );
}
