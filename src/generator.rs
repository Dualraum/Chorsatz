use crate::notes::SatbBlock;

use super::notes;

/// Generates a number of valid, unsorted SATB-progression from the passed triads or quadrains.
pub fn generate_satb(accords: &[notes::MultiNote]) -> Vec<Vec<SatbBlock>> {
    generate_satb_helper(accords, &[])
}

/// Recursive helper function
fn generate_satb_helper(
    // The accords not yet used
    remaining_accords: &[notes::MultiNote],
    // The SATB notes resulting from the already-used accords in this branch.
    prefix: &[notes::SatbBlock],
) -> Vec<Vec<SatbBlock>> {
    // No accors remaining -> Just return the (finished) prefix
    if remaining_accords.is_empty() {
        vec![prefix.to_vec()]
    } else {
        let mut res = Vec::new();
        // Go through all allowed permutations of the notes of the accord.
        'outer: for SatbBlock(s, a, t, b) in notes::permute(&remaining_accords[0]) {
            // Check numerous conditions
            // Not differences larger than sixth
            if (s - a).abs() > 7. || (a - t).abs() > 7. || (t - b).abs() > 10. {
                continue;
            }
            // No crossings
            if (a - s) > 0. || (t - a) > 0. || (b - t) > 0. {
                continue;
            }
            // Sopran-Bass-Difference
            if (b - s) > 21. {
                continue;
            }
            if let Some(SatbBlock(s_pre, a_pre, t_pre, b_pre)) = prefix.last().copied() {
                // Conditions based on the preceeding accord
                // Let sleeping dogs lie.
                if s_pre == a || s_pre == t || a_pre == s || a_pre == t || t_pre == s || t_pre == a
                {
                    continue;
                }
                // Avoid Fifth and Octave paralels
                for diff in [5., 8.] {
                    if (s_pre - a_pre).abs() == diff && (s - a).abs() == diff
                        || (s_pre - t_pre).abs() == diff && (s - t).abs() == diff
                        || (s_pre - b_pre).abs() == diff && (s - b).abs() == diff
                        || (a_pre - t_pre).abs() == diff && (a - t).abs() == diff
                        || (a_pre - b_pre).abs() == diff && (a - b).abs() == diff
                        || (t_pre - b_pre).abs() == diff && (t - b).abs() == diff
                    {
                        continue 'outer;
                    }
                }
                // If no tone is staying, all must move opposing to the bass
                if s_pre != s && a_pre != a && t_pre != t && b_pre != b {
                    let bass_diff = b - b_pre;
                    if bass_diff * (s - s_pre) > 0.
                        || bass_diff * (a - a_pre) > 0.
                        || bass_diff * (t - t_pre) > 0.
                    {
                        continue;
                    }
                }
                // prevent big jumps
                if (s_pre - s).abs() > 6.
                    || (a_pre - a).abs() > 6.
                    || (t_pre - t).abs() > 6.
                    || (b_pre - b).abs() > 7.
                {
                    continue;
                }
            } else {
                // Conditions for only the very first accord

                // no exposed edge tones
            }

            // If we have not yet continued, this continuation is okay, so recursively create all possible extensions with this prefix.
            res.extend(generate_satb_helper(
                &remaining_accords[1..],
                &[prefix, &[SatbBlock(s, a, t, b)]].concat(),
            ))
        }

        res
    }
}