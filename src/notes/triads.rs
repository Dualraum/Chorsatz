use super::MultiNote;
use super::NoteName;

pub fn create_triad(key: &str) -> MultiNote {
    match key {
        // Kreuztonarten
        // C
        "C" => MultiNote::Triad(NoteName::C, NoteName::E, NoteName::G),
        "Cm" => MultiNote::Triad(NoteName::C, NoteName::Dis, NoteName::G),
        "Cdim" => MultiNote::Triad(NoteName::C, NoteName::Dis, NoteName::Fis),
        "Caug" => MultiNote::Triad(NoteName::C, NoteName::E, NoteName::Gis),
        // G
        "G" => MultiNote::Triad(NoteName::G, NoteName::H, NoteName::D),
        "Gm" => MultiNote::Triad(NoteName::G, NoteName::Ais, NoteName::D),
        "Gdim" => MultiNote::Triad(NoteName::G, NoteName::Ais, NoteName::Cis),
        "Gaug" => MultiNote::Triad(NoteName::G, NoteName::H, NoteName::Dis),
        // D
        "D" => MultiNote::Triad(NoteName::D, NoteName::Fis, NoteName::A),
        "Dm" => MultiNote::Triad(NoteName::D, NoteName::F, NoteName::A),
        "Ddim" => MultiNote::Triad(NoteName::D, NoteName::F, NoteName::Gis),
        "Daug" => MultiNote::Triad(NoteName::D, NoteName::Fis, NoteName::Ais),
        // A
        "A" => MultiNote::Triad(NoteName::A, NoteName::Cis, NoteName::E),
        "Am" => MultiNote::Triad(NoteName::A, NoteName::C, NoteName::E),
        "Adim" => MultiNote::Triad(NoteName::A, NoteName::C, NoteName::Dis),
        "Aaug" => MultiNote::Triad(NoteName::A, NoteName::Cis, NoteName::F),
        // E
        "E" => MultiNote::Triad(NoteName::E, NoteName::Gis, NoteName::H),
        "Em" => MultiNote::Triad(NoteName::E, NoteName::G, NoteName::H),
        "Edim" => MultiNote::Triad(NoteName::E, NoteName::G, NoteName::Ais),
        "Eaug" => MultiNote::Triad(NoteName::E, NoteName::Gis, NoteName::C),
        // H
        "H" => MultiNote::Triad(NoteName::H, NoteName::Dis, NoteName::Fis),
        "Hm" => MultiNote::Triad(NoteName::H, NoteName::D, NoteName::Fis),
        "Hdim" => MultiNote::Triad(NoteName::H, NoteName::D, NoteName::Eis),
        "Haug" => MultiNote::Triad(NoteName::H, NoteName::Dis, NoteName::G),
        // Fis
        "Fis" => MultiNote::Triad(NoteName::Fis, NoteName::Ais, NoteName::Cis),
        "Fism" => MultiNote::Triad(NoteName::Fis, NoteName::A, NoteName::Cis),
        "Fisdim" => MultiNote::Triad(NoteName::Fis, NoteName::A, NoteName::C),
        "Fisaug" => MultiNote::Triad(NoteName::Fis, NoteName::Ais, NoteName::D),

        // b-Tonarten
        // F
        "F" => MultiNote::Triad(NoteName::F, NoteName::A, NoteName::C),
        "Fm" => MultiNote::Triad(NoteName::F, NoteName::Gis, NoteName::C),
        "Fdim" => MultiNote::Triad(NoteName::F, NoteName::A, NoteName::Cis),
        "Faug" => MultiNote::Triad(NoteName::F, NoteName::A, NoteName::Cis),
        // B
        "B" => MultiNote::Triad(NoteName::Ais, NoteName::D, NoteName::F),
        "Bm" => MultiNote::Triad(NoteName::Ais, NoteName::Des, NoteName::F),
        "Bdim" => MultiNote::Triad(NoteName::Ais, NoteName::Des, NoteName::E),
        "Baug" => MultiNote::Triad(NoteName::Ais, NoteName::D, NoteName::Ges),
        // Es
        "Es" => MultiNote::Triad(NoteName::Es, NoteName::G, NoteName::B),
        "Esm" => MultiNote::Triad(NoteName::Es, NoteName::Ges, NoteName::B),
        "Esdim" => MultiNote::Triad(NoteName::Es, NoteName::Ges, NoteName::A),
        "Esaug" => MultiNote::Triad(NoteName::Es, NoteName::G, NoteName::Ces),
        // As
        "As" => MultiNote::Triad(NoteName::As, NoteName::C, NoteName::Es),
        "Asm" => MultiNote::Triad(NoteName::As, NoteName::Ces, NoteName::Es),
        "Asdim" => MultiNote::Triad(NoteName::As, NoteName::Ces, NoteName::D),
        "Asaug" => MultiNote::Triad(NoteName::As, NoteName::C, NoteName::E),
        // Des
        "Des" => MultiNote::Triad(NoteName::Des, NoteName::F, NoteName::As),
        "Desm" => MultiNote::Triad(NoteName::Des, NoteName::E, NoteName::As),
        "Desdim" => MultiNote::Triad(NoteName::Des, NoteName::E, NoteName::G),
        "Desaug" => MultiNote::Triad(NoteName::Des, NoteName::F, NoteName::A),
        // Ges
        "Ges" => MultiNote::Triad(NoteName::Ges, NoteName::B, NoteName::Des),
        "Gesm" => MultiNote::Triad(NoteName::Ges, NoteName::A, NoteName::Des),
        "Gesdim" => MultiNote::Triad(NoteName::Ges, NoteName::A, NoteName::C),
        "Gesaug" => MultiNote::Triad(NoteName::Ges, NoteName::B, NoteName::D),
        //

        // enharmonisch gleiche sinnvolle Tonarten
        // Cis
        "Cis" => MultiNote::Triad(NoteName::Cis, NoteName::Eis, NoteName::Gis),
        "Cism" => MultiNote::Triad(NoteName::Cis, NoteName::E, NoteName::Gis),
        "Cisdim" => MultiNote::Triad(NoteName::Cis, NoteName::E, NoteName::G),
        "Cisaug" => MultiNote::Triad(NoteName::Cis, NoteName::Eis, NoteName::A),
        // Dis
        "Dis" => MultiNote::Triad(NoteName::Dis, NoteName::G, NoteName::Ais),
        "Dism" => MultiNote::Triad(NoteName::Dis, NoteName::Fis, NoteName::Ais),
        "Disdim" => MultiNote::Triad(NoteName::Dis, NoteName::Fis, NoteName::A),
        "Disaug" => MultiNote::Triad(NoteName::Dis, NoteName::G, NoteName::H),
        // Gis
        "Gis" => MultiNote::Triad(NoteName::Gis, NoteName::C, NoteName::Dis),
        "Gism" => MultiNote::Triad(NoteName::Gis, NoteName::H, NoteName::Dis),
        "Gisdim" => MultiNote::Triad(NoteName::Gis, NoteName::H, NoteName::D),
        "Gisaug" => MultiNote::Triad(NoteName::Gis, NoteName::C, NoteName::E),
        // Ais
        "Ais" => MultiNote::Triad(NoteName::Ais, NoteName::D, NoteName::F),
        "Aism" => MultiNote::Triad(NoteName::Ais, NoteName::Cis, NoteName::F),
        "Aisdim" => MultiNote::Triad(NoteName::Ais, NoteName::Cis, NoteName::E),
        "Aisaug" => MultiNote::Triad(NoteName::Ais, NoteName::D, NoteName::Fis),

        _ => MultiNote::Triad(NoteName::C, NoteName::C, NoteName::C),
    }
}
