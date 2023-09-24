use itertools::Itertools;

use super::notes::SatbBlock;

use super::notes;

/// Generates a number of valid, unsorted SATB-progression from the passed triads or quadrains.
pub fn generate_satb(accords: &[notes::MultiNote]) -> Vec<(Vec<SatbBlock>, f32)> {
    generate_satb_helper(accords, &[])
        .into_iter()
        .map(|solution| {
            let score = satb_score(&solution);
            (solution, score)
        })
        .sorted_by(|(_, score1), (_, score2)| score1.partial_cmp(score2).unwrap())
        .collect()
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
            if (b - s).abs() > 21. {
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
                    || (b_pre - b).abs() > 8.
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

fn satb_score(solution: &Vec<SatbBlock>) -> f32 {
    let mut score = 0.;

    for (SatbBlock(s_pre, a_pre, t_pre, b_pre), SatbBlock(s, a, t, b)) in
        solution.iter().copied().tuple_windows()
    {
        let mut sum_abs = 0.;
        let mut sum = 0.;
        sum += s - s_pre;
        sum_abs += (s - s_pre).abs();
        sum += a - a_pre;
        sum_abs += (a - a_pre).abs();
        sum += t - t_pre;
        sum_abs += (t - t_pre).abs();
        sum += b - b_pre;
        sum_abs += (b - b_pre).abs();

        score += sum_abs + 0.8 * sum.abs() + 0.4 * (s - b).abs();
    }

    if notes::OctavedNote::new(notes::NoteName::Cis, 2) - solution[0].0 < 0. {
        score += 1.2 * (solution.len() as f32 - 1.);
    }
    if notes::OctavedNote::new(notes::NoteName::Ais, 1) - solution[0].3 < 0. {
        score += 1.2 * (solution.len() as f32 - 1.);
    }

    score
}
