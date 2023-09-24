use super::notes::NoteName;

pub const POSITION_TO_HALFTONES: [f32; 12] = [1., 1.5, 2., 2.5, 3., 4., 4.5, 5., 5.5, 6., 6.5, 7.];

pub const C_MAJOR: Scale = Scale {
    notes: [
        NoteName::C,
        NoteName::Cis,
        NoteName::D,
        NoteName::Dis,
        NoteName::E,
        NoteName::F,
        NoteName::Fis,
        NoteName::G,
        NoteName::Gis,
        NoteName::A,
        NoteName::Ais,
        NoteName::H,
    ],
};

pub struct Scale {
    pub notes: [NoteName; 12],
}

#[allow(dead_code)]
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
    pub fn note_to_halftone(&self, note: NoteName) -> f32 {
        POSITION_TO_HALFTONES[self
            .notes
            .iter()
            .position(|n| *n == note)
            .expect("Could not find note name.")]
    }

    #[allow(dead_code)]
    pub fn halftone_to_note(&self, halftone: f32) -> NoteName {
        self.notes[POSITION_TO_HALFTONES
            .binary_search_by(|ht| ht.partial_cmp(&halftone).expect("Found NaN in tones."))
            .expect("Invalid halftone value.")]
    }
}
