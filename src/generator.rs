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
        for SatbBlock(s, a, t, b) in notes::permute(&remaining_accords[0]) {
            // Check numerous conditions

            // If we have not yet continued, this continuation is okay, so recursively create all possible extensions with this prefix.
            res.extend(generate_satb_helper(
                &remaining_accords[1..],
                &[prefix, &[SatbBlock(s, a, t, b)]].concat(),
            ))
        }

        res
    }
}
