mod note_names;
pub use note_names::NoteName;

mod multinotes;
pub use multinotes::MultiNote;
mod multinotes_german;
pub use multinotes_german::MultiNoteGerman;

mod satb_template;
pub use satb_template::SatbTemplate;

mod satb_block;
pub use satb_block::permute;
pub use satb_block::SatbBlock;

mod octaved_notes;
pub use octaved_notes::OctavedNote;

mod chor_error;
pub use chor_error::ChorError;

use super::scale;
