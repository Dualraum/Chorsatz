use super::MultiNote;
use super::NoteName;

pub fn create_triad(key: &str) -> Result<MultiNote, String> {
    match key {
        // Kreuztonarten
        // C
        "C" => Ok(MultiNote::Triad(NoteName::C, NoteName::E, NoteName::G)),
        "Cm" => Ok(MultiNote::Triad(NoteName::C, NoteName::Es, NoteName::G)),
        "Cdim" => Ok(MultiNote::Triad(NoteName::C, NoteName::Es, NoteName::Ges)),
        "Caug" => Ok(MultiNote::Triad(NoteName::C, NoteName::E, NoteName::Gis)),
        // G
        "G" => Ok(MultiNote::Triad(NoteName::G, NoteName::H, NoteName::D)),
        "Gm" => Ok(MultiNote::Triad(NoteName::G, NoteName::B, NoteName::D)),
        "Gdim" => Ok(MultiNote::Triad(NoteName::G, NoteName::B, NoteName::Des)),
        "Gaug" => Ok(MultiNote::Triad(NoteName::G, NoteName::H, NoteName::Dis)),
        // D
        "D" => Ok(MultiNote::Triad(NoteName::D, NoteName::Fis, NoteName::A)),
        "Dm" => Ok(MultiNote::Triad(NoteName::D, NoteName::F, NoteName::A)),
        "Ddim" => Ok(MultiNote::Triad(NoteName::D, NoteName::F, NoteName::As)),
        "Daug" => Ok(MultiNote::Triad(NoteName::D, NoteName::Fis, NoteName::Ais)),
        // A
        "A" => Ok(MultiNote::Triad(NoteName::A, NoteName::Cis, NoteName::E)),
        "Am" => Ok(MultiNote::Triad(NoteName::A, NoteName::C, NoteName::E)),
        "Adim" => Ok(MultiNote::Triad(NoteName::A, NoteName::C, NoteName::Es)),
        "Aaug" => Ok(MultiNote::Triad(NoteName::A, NoteName::Cis, NoteName::Eis)),
        // E
        "E" => Ok(MultiNote::Triad(NoteName::E, NoteName::Gis, NoteName::H)),
        "Em" => Ok(MultiNote::Triad(NoteName::E, NoteName::G, NoteName::H)),
        "Edim" => Ok(MultiNote::Triad(NoteName::E, NoteName::G, NoteName::B)),
        "Eaug" => Ok(MultiNote::Triad(NoteName::E, NoteName::Gis, NoteName::His)),
        // H
        "H" => Ok(MultiNote::Triad(NoteName::H, NoteName::Dis, NoteName::Fis)),
        "Hm" => Ok(MultiNote::Triad(NoteName::H, NoteName::D, NoteName::Fis)),
        "Hdim" => Ok(MultiNote::Triad(NoteName::H, NoteName::D, NoteName::F)),
        "Haug" => Ok(MultiNote::Triad(NoteName::H, NoteName::Dis, NoteName::Fisis)), 
        // Fis
        "Fis" => Ok(MultiNote::Triad(
            NoteName::Fis,
            NoteName::Ais,
            NoteName::Cis,
        )),
        "Fism" => Ok(MultiNote::Triad(NoteName::Fis, NoteName::A, NoteName::Cis)),
        "Fisdim" => Ok(MultiNote::Triad(NoteName::Fis, NoteName::A, NoteName::C)),
        "Fisaug" => Ok(MultiNote::Triad(NoteName::Fis, NoteName::Ais, NoteName::Cisis)),

        // b-Tonarten
        // F
        "F" => Ok(MultiNote::Triad(NoteName::F, NoteName::A, NoteName::C)),
        "Fm" => Ok(MultiNote::Triad(NoteName::F, NoteName::As, NoteName::C)),
        "Fdim" => Ok(MultiNote::Triad(NoteName::F, NoteName::As, NoteName::Ces)),
        "Faug" => Ok(MultiNote::Triad(NoteName::F, NoteName::A, NoteName::Cis)),
        // B
        "B" => Ok(MultiNote::Triad(NoteName::B, NoteName::D, NoteName::F)),
        "Bm" => Ok(MultiNote::Triad(NoteName::B, NoteName::Des, NoteName::F)),
        "Bdim" => Ok(MultiNote::Triad(NoteName::B, NoteName::Des, NoteName::Fes)), 
        "Baug" => Ok(MultiNote::Triad(NoteName::B, NoteName::D, NoteName::Fis)),
        // Es
        "Es" => Ok(MultiNote::Triad(NoteName::Es, NoteName::G, NoteName::B)),
        "Esm" => Ok(MultiNote::Triad(NoteName::Es, NoteName::Ges, NoteName::B)),
        "Esdim" => Ok(MultiNote::Triad(NoteName::Es, NoteName::Ges, NoteName::Heses)), 
        "Esaug" => Ok(MultiNote::Triad(NoteName::Es, NoteName::G, NoteName::H)),
        // As
        "As" => Ok(MultiNote::Triad(NoteName::As, NoteName::C, NoteName::Es)),
        "Asm" => Ok(MultiNote::Triad(NoteName::As, NoteName::Ces, NoteName::Es)),
        "Asdim" => Ok(MultiNote::Triad(NoteName::As, NoteName::Ces, NoteName::Eses)), 
        "Asaug" => Ok(MultiNote::Triad(NoteName::As, NoteName::C, NoteName::E)),
        // Des
        "Des" => Ok(MultiNote::Triad(NoteName::Des, NoteName::F, NoteName::As)),
        "Desm" => Ok(MultiNote::Triad(NoteName::Des, NoteName::Fes, NoteName::As)), 
        "Desdim" => Ok(MultiNote::Triad(NoteName::Des, NoteName::Fes, NoteName::Asas)), 
        "Desaug" => Ok(MultiNote::Triad(NoteName::Des, NoteName::F, NoteName::A)),
        // Ges
        "Ges" => Ok(MultiNote::Triad(NoteName::Ges, NoteName::B, NoteName::Des)),
        "Gesm" => Ok(MultiNote::Triad(NoteName::Ges, NoteName::Heses, NoteName::Des)), 
        "Gesdim" => Ok(MultiNote::Triad(NoteName::Ges, NoteName::Heses, NoteName::Deses)), 
        "Gesaug" => Ok(MultiNote::Triad(NoteName::Ges, NoteName::B, NoteName::D)),
        //

        // enharmonisch gleiche sinnvolle Tonarten
        // Cis
        "Cis" => Ok(MultiNote::Triad(
            NoteName::Cis,
            NoteName::Eis,
            NoteName::Gis,
        )),
        "Cism" => Ok(MultiNote::Triad(NoteName::Cis, NoteName::E, NoteName::Gis)),
        "Cisdim" => Ok(MultiNote::Triad(NoteName::Cis, NoteName::E, NoteName::G)),
        "Cisaug" => Ok(MultiNote::Triad(NoteName::Cis, NoteName::Eis, NoteName::Gisis)), 
        // Dis
        "Dis" => Ok(MultiNote::Triad(NoteName::Dis, NoteName::Fisis, NoteName::Ais)),
        "Dism" => Ok(MultiNote::Triad(
            NoteName::Dis,
            NoteName::Fis,
            NoteName::Ais,
        )),
        "Disdim" => Ok(MultiNote::Triad(NoteName::Dis, NoteName::Fis, NoteName::A)),
        "Disaug" => Ok(MultiNote::Triad(NoteName::Dis, NoteName::Fisis, NoteName::Aisis)), 
        "Gis" => Ok(MultiNote::Triad(NoteName::Gis, NoteName::His, NoteName::Dis)), 
        "Gism" => Ok(MultiNote::Triad(NoteName::Gis, NoteName::H, NoteName::Dis)),
        "Gisdim" => Ok(MultiNote::Triad(NoteName::Gis, NoteName::H, NoteName::D)),
        "Gisaug" => Ok(MultiNote::Triad(NoteName::Gis, NoteName::His, NoteName::Disis)), 
        // Ais
        "Ais" => Ok(MultiNote::Triad(NoteName::Ais, NoteName::Cisis, NoteName::Eis)), 
        "Aism" => Ok(MultiNote::Triad(NoteName::Ais, NoteName::Cis, NoteName::Eis)),
        "Aisdim" => Ok(MultiNote::Triad(NoteName::Ais, NoteName::Cis, NoteName::E)),
        "Aisaug" => Ok(MultiNote::Triad(NoteName::Ais, NoteName::Cisis, NoteName::Eisis)), 

        other => Err(format!("Could not find note <{other}>.")),
    }
}
