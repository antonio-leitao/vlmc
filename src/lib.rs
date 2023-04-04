use hashbrown::{HashMap, HashSet};
use pyo3::prelude::*;
use std::thread;

//------------------ STRUCTURES ---------------------
#[derive(Debug, PartialEq)]
struct Node {
    count: usize,
    precedents: HashSet<u32>,
    successors: HashSet<u32>,
}
impl Node {
    fn new() -> Self {
        Node {
            count: 0,
            precedents: HashSet::new(),
            successors: HashSet::new(),
        }
    }
}

enum NumJobs {
    One,
    Many(usize),
}

impl NumJobs {
    fn from_i32(n_jobs: i32) -> NumJobs {
        if n_jobs == 1 {
            NumJobs::One
        } else if n_jobs > 1 {
            NumJobs::Many(n_jobs as usize)
        } else {
            NumJobs::Many(num_cpus::get())
        }
    }
}
//------------------------------------------------------

//------------------ AUX FUNCTIONS ---------------------
fn count_subsequences_parallel(
    data: Vec<Vec<u32>>,
    max_depth: usize,
    num_threads: usize,
) -> HashMap<Vec<u32>, Node> {
    //let num_threads = num_cpus::get(); // get the number of available CPU cores
    let chunk_size = data.len() / num_threads; // divide the data into equal-sized chunks
    let mut threads = Vec::with_capacity(num_threads);

    // spawn threads to process each chunk
    for chunk in data.chunks(chunk_size) {
        let chunk_data = chunk.to_vec(); // convert the chunk slice to a vector

        let handle = thread::spawn(move || count_subsequences(chunk_data, max_depth));

        threads.push(handle);
    }

    // collect the results from all threads and combine them into a single HashMap
    let mut counts = HashMap::new();

    for handle in threads {
        let chunk_counts = handle.join().unwrap();
        for (k, v) in chunk_counts {
            let node = counts.entry(k).or_insert(Node::new());
            node.count += v.count;
            node.precedents.extend(v.precedents);
            node.successors.extend(v.successors);
        }
    }

    counts
}

fn count_subsequences(data: Vec<Vec<u32>>, max_depth: usize) -> HashMap<Vec<u32>, Node> {
    let mut counts: HashMap<Vec<u32>, Node> = HashMap::new();
    let mut zero_counts: usize = 0;
    let mut zero_successors: HashSet<u32> = HashSet::new();
    for seq in data {
        zero_counts += seq.len();
        let n = seq.len().min(max_depth);
        for i in 0..seq.len() {
            zero_successors.insert(seq[i]);
            for j in i..(i + n).min(seq.len()) {
                let subseq = &seq[i..=j];
                let node = counts.entry(subseq.to_vec()).or_insert(Node::new());
                node.count += 1;

                if i > 0 {
                    node.precedents.insert(seq[i - 1]);
                }
                if j < seq.len() - 1 {
                    node.successors.insert(seq[j + 1]);
                }
            }
        }
    }
    let zero_node = Node {
        count: zero_counts,
        precedents: HashSet::new(),
        successors: zero_successors,
    };
    counts.insert(vec![], zero_node);
    counts
}
//------------------------------------------------------

//-------------------- MAIN CLASS ----------------------
#[pyclass(name = "VLMC")]
struct VLMCObject {
    max_depth: usize,
    alphabet_size: usize,
    n_jobs: NumJobs,
    #[pyo3(get)]
    counts: usize,
    nodes: HashMap<Vec<u32>, Node>,
}

#[pymethods]
impl VLMCObject {
    #[new]
    #[pyo3(signature = (max_depth, alphabet_size,n_jobs=-1))]
    fn new(max_depth: usize, alphabet_size: usize, n_jobs: i32) -> Self {
        VLMCObject {
            max_depth: max_depth,
            alphabet_size: alphabet_size,
            n_jobs: NumJobs::from_i32(n_jobs),
            counts: 0,
            nodes: HashMap::new(),
        }
    }

    //THIS ONE IS IMPORTANT NOT JUST FOR SKLEARN API BUT ALSO
    // TO AVOID UNNECESSARY CHECKS DOWN THE LINE.
    fn check_params(&self) {}

    #[pyo3(signature = (data))]
    fn fit(&mut self, data: Vec<Vec<u32>>) -> PyResult<()> {
        match self.n_jobs {
            NumJobs::One => {
                self.counts = count_subsequences(data, self.max_depth + 1)
                    .into_values()
                    .map(|node| node.count)
                    .sum();
            }
            NumJobs::Many(num_jobs) => {
                self.counts = count_subsequences_parallel(data, self.max_depth + 1, num_jobs)
                    .into_values()
                    .map(|node| node.count)
                    .sum();
            }
        }

        Ok(())
    }
    #[pyo3(signature = (data))]
    fn fit_transform(&mut self, data: Vec<Vec<u32>>) -> PyResult<()> {
        //max_depth has tp be plus 1 so we get the successors of the sequences at max depth.
        match self.n_jobs {
            NumJobs::One => {
                self.nodes = count_subsequences(data, self.max_depth + 1);
            }
            NumJobs::Many(num_jobs) => {
                self.nodes = count_subsequences_parallel(data, self.max_depth + 1, num_jobs);
            }
        }
        Ok(())
    }
}
impl VLMCObject {
    fn get_suffix(&self, sequence: Vec<u32>) -> &Node {
        let mut sequence = sequence;
        while !sequence.is_empty() {
            match self.nodes.get(&sequence) {
                Some(node) => return node,
                None => sequence.remove(0),
            };
        }
        self.nodes.get(&vec![]).unwrap()
    }

    fn get_distribution(&self, sequence: Vec<u32>) -> Vec<usize> {
        //node has to exist and be less or equal to max_depth
        let mut distribution: Vec<usize> = vec![0; self.alphabet_size];
        let node = self.nodes.get(&sequence).unwrap();
        for successor in &node.successors {
            distribution[*successor as usize] = self.get_successor_counts(&sequence, *successor);
        }
        distribution
    }
    fn prune_tree(&mut self) {
        //starts at 0
        //iterates down untill max_depth or loglogn
        //if condition is satisfied adds node to nodes to visit
        //next node to visit
        //if condition not satisfied remove node
    }
    fn peres_shield_divergence(&self, seq1: Vec<u32>, seq2: Vec<u32>) -> f32 {
        let N_v = self.nodes.get(&seq1).unwrap();
        let N_w = self.nodes.get(&seq2).unwrap();
        let mut max_divergence: f32 = 0.0;

        for successor in N_v.successors.union(&N_w.successors) {
            let divergence: f32;
            let N_va = self.get_successor_counts(&seq1, *successor);
            let N_wa = self.get_successor_counts(&seq2, *successor);
            if N_w.count == 0 {
                divergence = N_w.count as f32;
            } else {
                divergence =
                    (N_va as f32 - (N_wa as f32 * N_v.count as f32 / N_w.count as f32)).abs();
            }
            if divergence >= max_divergence {
                max_divergence = divergence;
            }
        }
        max_divergence
    }

    fn get_successor_counts(&self, sequence: &Vec<u32>, successor: u32) -> usize {
        let mut query_sequence: Vec<u32> = sequence.clone();
        query_sequence.push(successor);
        match self.nodes.get(&query_sequence) {
            Some(node) => return node.count,
            None => return 0,
        }
    }
}

//------------------------------------------------------

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn vlmc(_py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(count_sequences, m)?)?;
    m.add_class::<VLMCObject>()?;

    Ok(())
}
