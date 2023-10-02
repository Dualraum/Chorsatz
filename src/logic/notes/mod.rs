mod note_names;
pub use note_names::NoteName;

mod multinotes;
pub use multinotes::create_multinote;
pub use multinotes::MultiNote;
pub use multinotes::SatbTemplate;

mod satb_block;
pub use satb_block::permute;
pub use satb_block::SatbBlock;

mod octaved_notes;
pub use octaved_notes::OctavedNote;

mod chor_error;
pub use chor_error::ChorError;

use super::scale;
