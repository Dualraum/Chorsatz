mod note_names;
pub use note_names::NoteName;

mod multinotes;
pub use multinotes::create_multinote;
pub use multinotes::MultiNote;

mod satb_block;
pub use satb_block::permute;
pub use satb_block::SatbBlock;

mod octaved_notes;
pub use octaved_notes::OctavedNote;

use super::scale;
