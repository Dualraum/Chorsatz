use itertools::Itertools;

use super::notes::SatbBlock;

use super::notes;

/// Generates a number of valid, unsorted SATB-progression from the passed triads or quadrains.
pub fn generate_satb(
    accords: &[notes::SatbTemplate],
    conf: &super::Config,
) -> Vec<(Vec<SatbBlock>, f32)> {
    generate_satb_helper(&accords.iter().map(notes::permute).collect_vec(), &[], conf)
        .into_iter()
        .map(|solution| {
            let score = satb_score(&solution, conf);
            (solution, score)
        })
        .sorted_by(|(_, score1), (_, score2)| score1.partial_cmp(score2).unwrap())
        .collect()
}

/// Recursive helper function
fn generate_satb_helper(
    // The accords not yet used
    remaining_accords_permutations: &[Vec<SatbBlock>],
    // The SATB notes resulting from the already-used accords in this branch.
    prefix: &[notes::SatbBlock],
    // The Config file describing how to discard pathes
    conf: &super::Config,
) -> Vec<Vec<SatbBlock>> {
    // No accors remaining -> Just return the (finished) prefix
    if remaining_accords_permutations.is_empty() {
        vec![prefix.to_vec()]
    } else {
        let mut res = Vec::new();
        // Go through all allowed permutations of the notes of the accord.
        'outer: for &SatbBlock(s, a, t, b) in remaining_accords_permutations[0].iter() {
            // Check numerous conditions
            // Not differences larger than sixth
            if (s - a).abs() > conf.max_diff_sopran_alt
                || (a - t).abs() > conf.max_diff_alt_tenor
                || (t - b).abs() > conf.max_diff_tenor_bass
            {
                continue;
            }
            // No crossings
            if !conf.allow_crossings
                && ((a - s) > 0.
                    || (t - a) > 0.
                    || (b - t) > 0.
                    || (!conf.allow_bass_tenor_equal && (b - t) >= 0.))
            {
                continue;
            }
            // Sopran-Bass-Difference
            if (b - s).abs() > conf.max_diff_sopran_bass {
                continue;
            }
            if let Some(SatbBlock(s_pre, a_pre, t_pre, b_pre)) = prefix.last().copied() {
                // Conditions based on the preceeding accord
                // Let sleeping dogs lie.
                if conf.force_letting_lie
                    && (s_pre == a
                        || s_pre == t
                        || a_pre == s
                        || a_pre == t
                        || t_pre == s
                        || t_pre == a)
                {
                    continue;
                }
                // Avoid Fifth and Octave paralels
                for diff in conf.forbidden_parallels.iter().copied() {
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
                if conf.force_base_countermovement
                    && (s_pre != s && a_pre != a && t_pre != t && b_pre != b)
                {
                    let bass_diff = b - b_pre;
                    if bass_diff * (s - s_pre) > 0.
                        || bass_diff * (a - a_pre) > 0.
                        || bass_diff * (t - t_pre) > 0.
                    {
                        continue;
                    }
                }
                // prevent big jumps
                if (s_pre - s).abs() > conf.max_jump_sopran
                    || (a_pre - a).abs() > conf.max_jump_alt
                    || (t_pre - t).abs() > conf.max_jump_tenor
                    || (b_pre - b).abs() > conf.max_jump_bass
                {
                    continue;
                }
            } else {
                // Conditions for only the very first accord

                // no exposed edge tones
            }

            // If we have not yet continued, this continuation is okay, so recursively create all possible extensions with this prefix.
            res.extend(generate_satb_helper(
                &remaining_accords_permutations[1..],
                &[prefix, &[SatbBlock(s, a, t, b)]].concat(),
                conf,
            ))
        }

        res
    }
}

fn satb_score(solution: &[SatbBlock], conf: &super::Config) -> f32 {
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

        score += conf.sub_of_abs_weight * sum_abs
            + conf.abs_of_sum_weight * sum.abs()
            + conf.soprano_alt_diff_weight * (s - b).abs();
    }

    if conf.exposure_threshold_sopran - solution[0].0 < 0. {
        score += conf.exposure_penalty_sopran * (solution.len() as f32 - 1.);
    }
    if conf.exposure_threshold_bass - solution[0].3 < 0. {
        score += conf.exposure_penalty_bass * (solution.len() as f32 - 1.);
    }

    score
}
