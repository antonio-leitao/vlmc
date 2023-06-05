use crate::VLMCObject; // Import the struct from lib.rs
use hashbrown::HashSet;
use std::collections::VecDeque;

impl VLMCObject {
    // Define additional methods here
    pub fn peres_shield_prune(&mut self) {
        //threshold parameters
        let maximum_extension = (self.total_symbols as f32).log(10.0).log(10.0) as usize;
        let divergence_threshold = (self.total_symbols as f32).powf(3.0 / 4.0);
        //instantiate data structures
        let mut definitive_seqs = HashSet::new();
        let mut queue = VecDeque::new();
        //Add the root node
        definitive_seqs.insert(vec![]);
        queue.push_back(vec![]);

        //iterate top-down
        while let Some(parent_sequence) = queue.pop_front() {
            //stop if depth exceeded;
            if parent_sequence.len() == self.max_depth {
                continue;
            }
            //expand breath first on each node
            for new_parent in self.peres_shield_extend_sequence(
                parent_sequence,
                maximum_extension,
                divergence_threshold,
            ) {
                //if it has been already evaluated move on
                if !definitive_seqs.insert(new_parent.clone()) {
                    continue;
                }
                //else add it to the queue
                queue.push_back(new_parent);
            }
        }
        //discard all but the ones we selected
        self.nodes.retain(|key, _| definitive_seqs.contains(key));
    }
    fn peres_shield_extend_sequence(
        &self,
        parent_key: Vec<usize>,
        maximum_extension: usize,
        divergence_threshold: f32,
    ) -> Vec<Vec<usize>> {
        let mut queue = VecDeque::new();
        let mut accepted_sequences = Vec::new();
        let maximum_len = self.max_depth.min(parent_key.len() + maximum_extension);
        queue.push_back(parent_key.clone());
        while let Some(key) = queue.pop_front() {
            //go through every tree child
            for child_key in self.nodes.get(&key).unwrap().predecessor_sequences(&key) {
                //we have reached the end of the line
                if child_key.len() > maximum_len {
                    break;
                }
                //accept new node if fluctuation is bigger than some threshold
                if self.peres_shield_divergence(&parent_key, &child_key) <= divergence_threshold {
                    continue;
                }
                queue.push_back(child_key.clone());
                accepted_sequences.push(child_key);
            }
        }
        accepted_sequences
    }

    fn peres_shield_divergence(&self, seq1: &Vec<usize>, seq2: &Vec<usize>) -> f32 {
        let n_v = self.nodes.get(seq1).unwrap();
        let n_w = self.nodes.get(seq2).unwrap();
        let mut max_divergence: f32 = 0.0;

        for successor in n_v.successors.union(&n_w.successors) {
            let divergence: f32;
            let n_va = n_v.distribution[*successor];
            let n_wa = n_w.distribution[*successor];
            if n_w.count == 0 {
                divergence = n_w.count as f32;
            } else {
                divergence =
                    (n_va as f32 - (n_wa as f32 * n_v.count as f32 / n_w.count as f32)).abs();
            }
            if divergence >= max_divergence {
                max_divergence = divergence;
            }
        }
        max_divergence
    }
}
