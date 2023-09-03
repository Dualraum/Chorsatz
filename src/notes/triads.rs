use super::MultiNote;
use super::NoteName;

pub fn create_triad(key: &str) -> MultiNote {
    match key {
        // Cs
        "C" => MultiNote::Triad(NoteName::C, NoteName::E, NoteName::G),
        "Cm" => MultiNote::Triad(NoteName::C, NoteName::Dis, NoteName::G),
        "Cdim" => MultiNote::Triad(NoteName::C, NoteName::Dis, NoteName::Fis),
        "Caug" => MultiNote::Triad(NoteName::C, NoteName::E, NoteName::Gis),
        // Gs
        "G" => MultiNote::Triad(NoteName::G, NoteName::H, NoteName::D),
        "Gm" => MultiNote::Triad(NoteName::G, NoteName::Ais, NoteName::D),
        "Gdim" => MultiNote::Triad(NoteName::G, NoteName::Ais, NoteName::Cis),
        "Gaug" => MultiNote::Triad(NoteName::G, NoteName::H, NoteName::Dis),
        // Ds
        "D" => MultiNote::Triad(NoteName::D, NoteName::Fis, NoteName::A),
        "Dm" => MultiNote::Triad(NoteName::D, NoteName::F, NoteName::A),
        "Ddim" => MultiNote::Triad(NoteName::D, NoteName::F, NoteName::Gis),
        "Daug" => MultiNote::Triad(NoteName::D, NoteName::Fis, NoteName::Ais),

        _ => MultiNote::Triad(NoteName::C, NoteName::C, NoteName::C),
    }
}
