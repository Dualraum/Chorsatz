use super::notes::NoteName;

/// In the C_MAJOR scale, the number halftones each position is above the initial C
pub const POSITION_TO_HALFTONES: [f32; 12] = [1., 1.5, 2., 2.5, 3., 4., 4.5, 5., 5.5, 6., 6.5, 7.];

/// The notes in the C-Major scale
pub const C_MAJOR: Scale = Scale {
    notes: [
        NoteName::C,
        NoteName::Cs,
        NoteName::D,
        NoteName::Ds,
        NoteName::E,
        NoteName::F,
        NoteName::Fs,
        NoteName::G,
        NoteName::Gs,
        NoteName::A,
        NoteName::As,
        NoteName::B,
    ],
};

/// A musical scale of 12 notes.
pub struct Scale {
    /// The (ordered) notes of the scale.
    pub notes: [NoteName; 12],
}

#[allow(dead_code)]
/// Generate a scale in a major note.
pub fn generate_scale(name: &str) -> Scale {
    let mut res = C_MAJOR;
    res.notes[..].rotate_right(match name {
        "C" => 0,
        "G" => 5,
        "D" => 10,
        "A" => 3,
        "E" => 8,

        _ => 0,
    });
    res
}

impl Scale {
    #[allow(dead_code)]
    /// Converts a note to the number of halftones above the tonika of the scale.
    pub fn note_to_halftone(&self, note: NoteName) -> f32 {
        POSITION_TO_HALFTONES[self
            .notes
            .iter()
            .position(|n| *n == note)
            .expect("Could not find note name.")]
    }

    #[allow(dead_code)]
    /// Converts a number of halftones to the fitting note in this scale with that distance of halftones to the tonika.
    pub fn halftone_to_note(&self, halftone: f32) -> NoteName {
        self.notes[POSITION_TO_HALFTONES
            .binary_search_by(|ht| ht.partial_cmp(&halftone).expect("Found NaN in tones."))
            .expect("Invalid halftone value.")]
    }
}
