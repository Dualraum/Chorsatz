use itertools::Itertools;

use super::ChorError;
use super::MultiNote;
use super::MultiNoteGerman;
use super::NoteName;
use crate::app::languages;

/// A template to create an SATB-Block from, containing either a Triad or Quatrain as well as a bass note.
#[derive(Debug, Clone, Copy)]
pub struct SatbTemplate {
    /// The Triad/Quatrain to base the harmony of the accord on.
    pub accord: MultiNote,
    /// The note to be sung in the bass voice.
    pub bass: NoteName,
}

impl SatbTemplate {
    pub fn from_str_language(s: &str, language: languages::Language) -> Result<Self, ChorError> {
        let parts = s.split('/').collect_vec();
        if parts.len() == 1 {
            match language {
                languages::Language::English => Ok(s.parse::<MultiNote>()?.into()),
                languages::Language::German => {
                    Ok(s.parse::<MultiNoteGerman>()?.to_multinote().into())
                }
            }
        } else {
            match language {
                languages::Language::English => Ok(SatbTemplate {
                    accord: parts[0].parse()?,
                    bass: parts[1]
                        .parse::<NoteName>()
                        .map_err(ChorError::NoteNameParse)?,
                }),
                languages::Language::German => Ok(SatbTemplate {
                    accord: parts[0].parse::<MultiNoteGerman>()?.to_multinote(),
                    bass: NoteName::from_german_name(parts[1])?,
                }),
            }
        }
    }
}

impl From<MultiNote> for SatbTemplate {
    fn from(value: MultiNote) -> Self {
        match value {
            MultiNote::Triad(n1, _, _) => SatbTemplate {
                accord: value,
                bass: n1,
            },
            MultiNote::Quatrain(n1, _, _, _) => SatbTemplate {
                accord: value,
                bass: n1,
            },
        }
    }
}

impl std::str::FromStr for SatbTemplate {
    type Err = ChorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('/').collect_vec();
        if parts.len() == 1 {
            Ok(s.parse::<MultiNote>()?.into())
        } else {
            Ok(SatbTemplate {
                accord: parts[0].parse()?,
                bass: parts[1]
                    .parse::<NoteName>()
                    .map_err(ChorError::NoteNameParse)?,
            })
        }
    }
}
