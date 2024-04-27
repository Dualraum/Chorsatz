use super::notes;
//
/// A configuration struct that contains details on what pathes to discard during the tree search for a solution.
#[derive(Clone, Debug)]
pub struct Config {
    /// The tone difference of sopran and alt is enforced to be no larger than this value.
    pub max_diff_sopran_alt: f32,
    /// The tone difference of alt and tenor is enforced to be no larger than this value.
    pub max_diff_alt_tenor: f32,
    /// The tone difference of tenor and bass is enforced to be no larger than this value.
    pub max_diff_tenor_bass: f32,
    /// The tone difference of sopran and bass is enforced to be no larger than this value.
    pub max_diff_sopran_bass: f32,
    /// If set to true, the notes must always fulfil sopran > alt > tenor > bass.
    pub allow_crossings: bool,
    /// If set to true, bass and tenor may be the same note even if allow_crossings is set to false.
    pub allow_bass_tenor_equal: bool,
    /// If there is any movement where two voices (e.g. sopran and tenor) have a difference of x both before and after the movement,
    /// and x is contained in this vector, the solution path is discarded.
    pub forbidden_parallels: Vec<f32>,
    /// If set to true, if it is possible for a voice to be the same note in two succeeding accords, all other posibilities are discarded.
    pub force_letting_lie: bool,
    /// If set to true, when there are no parallel movements, all voices must always more opposing to the movement of the base voice (or stay).
    pub force_base_countermovement: bool,
    /// The maximum difference between two notes in the soprano voice from one accord to the next.
    pub max_jump_sopran: f32,
    /// The maximum difference between two notes in the alto voice from one accord to the next.
    pub max_jump_alt: f32,
    /// The maximum difference between two notes in the tenor voice from one accord to the next.
    pub max_jump_tenor: f32,
    /// The maximum difference between two notes in the bass voice from one accord to the next.
    pub max_jump_bass: f32,
    /// Describes the relative weight of the sum of the absolute values of the tone jumps in each voice to the total score.
    pub sub_of_abs_weight: f32,
    /// Describes the relative weight of the absolute value of the sum of the tone jumps in each voice to the total score.
    pub abs_of_sum_weight: f32,
    /// Describes the relative weight of the soprano-alt-difference on the total score.
    pub soprano_bass_diff_weight: f32,
    /// Describes the relative penalty to the score if the initial soprano note is above the threshold.
    pub exposure_penalty_sopran: f32,
    /// The highest note possible for the initial soprano note without being penalized.
    pub exposure_threshold_sopran: notes::OctavedNote,
    /// Describes the relative penalty to the score if the initial bass note is above the threshold.
    pub exposure_penalty_bass: f32,
    /// The highest note possible for the initial bass note without being penalized.
    pub exposure_threshold_bass: notes::OctavedNote,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_diff_sopran_alt: 7.,
            max_diff_alt_tenor: 7.,
            max_diff_tenor_bass: 10.,
            max_diff_sopran_bass: 21.,
            allow_crossings: false,
            allow_bass_tenor_equal: false,
            forbidden_parallels: vec![5., 8.],
            force_letting_lie: true,
            force_base_countermovement: true,
            max_jump_sopran: 6.,
            max_jump_alt: 6.,
            max_jump_tenor: 6.,
            max_jump_bass: 8.,
            sub_of_abs_weight: 0.8,
            abs_of_sum_weight: 1.0,
            soprano_bass_diff_weight: 0.4,
            exposure_penalty_sopran: 1.2,
            exposure_threshold_sopran: notes::OctavedNote::new(notes::NoteName::Cs, 2),
            exposure_penalty_bass: 1.2,
            exposure_threshold_bass: notes::OctavedNote::new(notes::NoteName::As, 1),
        }
    }
}
