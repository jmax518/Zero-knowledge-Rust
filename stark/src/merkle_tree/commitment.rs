use super::{Hash, Index, Proof};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Commitment {
    depth: usize,
    hash:  Hash,
}

impl Commitment {
    pub fn from_depth_hash(depth: usize, hash: &Hash) -> Self {
        // The number of leaves needs to fit `usize`
        assert!(depth < (0_usize.count_zeros() as usize));
        Self {
            depth,
            hash: hash.clone(),
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn num_leaves(&self) -> usize {
        1_usize << self.depth
    }

    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    // Convert leaf indices to a sorted list of unique MerkleIndices.
    fn sort_indices(&self, indices: &[usize]) -> Vec<Index> {
        let mut indices: Vec<Index> = indices
            .iter()
            .map(|i| Index::from_depth_offset(self.depth, *i).expect("Index out of range"))
            .collect();
        indices.sort_unstable();
        indices.dedup();
        indices
    }

    /// The number of hashes in the proof for the given set of indices.
    pub fn proof_size(&self, indices: &[usize]) -> usize {
        let indices = self.sort_indices(indices);

        // Start with the full path length for the first index
        // then add the path length of each next index up to the last common
        // ancestor with the previous index.
        // One is subtracted from each path because we omit the leaf hash.
        self.depth - 2 // TODO: Explain
            + indices
                .iter()
                .tuple_windows()
                .map(|(&current, &next)| {
                    self.depth - current.last_common_ancestor(next).depth() - 1
                })
                .sum::<usize>()
    }
}
