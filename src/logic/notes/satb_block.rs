use itertools::Itertools;

use super::{MultiNote, NoteName, OctavedNote};

/// A block of four octaved notes describing one parallel note for all four voices.
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
    // generate the notes to be permuted for s/a/t
    let permuted_notes = match notes.accord {
        // Triad: Just the three notes in the triad.
        MultiNote::Triad(n1, n2, n3) => [n1, n2, n3],
        // Quatrain: If the forced base-tone is contained in the quatrain, permute the others. Otherwise,ignore the base of the quatrain and permute the rest.
        MultiNote::Quatrain(n1, n2, n3, n4) => {
            // create a vec of the notes
            let mut rest = vec![n1, n2, n3, n4];
            // remove the bass note if it is in there (by name)
            rest.retain(|n| n.to_string() != notes.bass.to_string());
            // if nothing was removed, remove the first element so we are left with only 3 notes
            if rest.len() == 4 {
                rest.remove(0);
            }
            [rest[0], rest[1], rest[2]]
        }
    };

    permuted_notes
        .iter()
        .permutations(3)
        .unique()
        .map(|permutation| {
            (
                *permutation[0],
                *permutation[1],
                *permutation[2],
                notes.bass,
            )
        })
        .flat_map(|(s, a, t, b)| {
            let mut sopran_res = Vec::with_capacity(3);

            sopran_res.push((OctavedNote::new(s, 1), a, t, b));

            match s {
                NoteName::Gs
                | NoteName::Af
                | NoteName::A
                | NoteName::As
                | NoteName::Bf
                | NoteName::B
                | NoteName::Cf => {}
                _ => sopran_res.push((OctavedNote::new(s, 2), a, t, b)),
            }

            sopran_res
        })
        .flat_map(|(s, a, t, b)| {
            let mut alto_res = Vec::with_capacity(3);

            alto_res.push((s, OctavedNote::new(a, 1), t, b));

            match a {
                NoteName::Cs
                | NoteName::Df
                | NoteName::D
                | NoteName::Ds
                | NoteName::Ef
                | NoteName::E
                | NoteName::F
                | NoteName::Es
                | NoteName::Fs
                | NoteName::Gf => {}
                NoteName::C => alto_res.push((s, OctavedNote::new(a, 2), t, b)),
                _ => alto_res.push((s, OctavedNote::new(a, 0), t, b)),
            }

            alto_res
        })
        .flat_map(|(s, a, t, b)| {
            let mut tenor_res = Vec::with_capacity(3);

            tenor_res.push((s, a, OctavedNote::new(t, 0), b));

            match t {
                NoteName::Gs
                | NoteName::Af
                | NoteName::A
                | NoteName::As
                | NoteName::Bf
                | NoteName::B
                | NoteName::Cf => {}
                _ => tenor_res.push((s, a, OctavedNote::new(t, 1), b)),
            }

            tenor_res
        })
        .flat_map(|(s, a, t, b)| {
            let mut bass_res = Vec::with_capacity(3);

            bass_res.push((s, a, t, OctavedNote::new(b, 0)));

            match b {
                NoteName::Cs
                | NoteName::Df
                | NoteName::D
                | NoteName::Ds
                | NoteName::Ef
                | NoteName::E
                | NoteName::F
                | NoteName::Es
                | NoteName::Fs
                | NoteName::Gf => {}
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
            accord: MultiNote::Triad(NoteName::Cs, NoteName::D, NoteName::F),
            bass: NoteName::Cs
        })
        .len(),
        24,
    );
}
