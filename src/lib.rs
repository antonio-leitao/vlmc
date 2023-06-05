use hashbrown::{HashMap, HashSet};
use pyo3::prelude::*;
//use std::thread;
mod peres_shield;

//###################### PYTHON INTERFACE ##########################
//Anything inside this section is exposed to python

#[pymodule]
fn vlmc(_py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(count_sequences, m)?)?;
    m.add_class::<VLMCObject>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}

//--------------------------- MAIN CLASS ----------------------
#[pyclass(name = "VLMC")]
pub struct VLMCObject {
    nodes: HashMap<Vec<usize>, Node>,
    alphabet_size: usize,
    max_depth: usize,
    total_symbols: u32,
}

#[pymethods]
impl VLMCObject {
    #[new]
    #[pyo3(signature = (alphabet_size, max_depth=10))]
    fn new(alphabet_size: usize, max_depth: usize) -> Self {
        VLMCObject {
            nodes: HashMap::new(),
            alphabet_size: alphabet_size,
            max_depth: max_depth,
            total_symbols: 0,
        }
    }

    #[pyo3(signature = (data))]
    fn fit(&mut self, data: Vec<Vec<usize>>) -> PyResult<()> {
        self._fit(data);
        Ok(())
    }
    #[pyo3(signature = (sequence))]
    fn get_suffix(&self, sequence: Vec<usize>) -> Vec<usize> {
        self._suffix(&sequence)
    }
    #[pyo3(signature = (sequence))]
    fn get_counts(&self, sequence: Vec<usize>) -> u32 {
        self._counts(&sequence)
    }
    #[pyo3(signature = (sequence))]
    fn get_distribution(&self, sequence: Vec<usize>) -> Vec<u32> {
        self._distribution(&sequence).to_vec()
    }
}

//############################# BACKEND ##########################
//Here should be methods and stuff that will not be exposed to Python
impl VLMCObject {
    fn _fit(&mut self, data: Vec<Vec<usize>>) {
        //creates counts
        let raw_counts = count_subsequences(data, self.alphabet_size, self.max_depth);
        self.nodes = raw_counts;
        self.total_symbols = self.nodes.get(&vec![]).unwrap().count;
        //prunes
        self.peres_shield_prune();
    }
    fn _suffix(&self, sequence: &Vec<usize>) -> Vec<usize> {
        let mut suffix: &[usize] = sequence;
        while !suffix.is_empty() && !self.nodes.contains_key(suffix) {
            suffix = &suffix[1..];
        }
        suffix.to_vec()
    }
    fn _node(&self, sequence: &Vec<usize>) -> &Node {
        let suffix = self._suffix(sequence);
        self.nodes.get(&suffix).unwrap()
    }
    fn _counts(&self, sequence: &Vec<usize>) -> u32 {
        let node = self._node(sequence);
        node.count
    }
    fn _distribution(&self, sequence: &Vec<usize>) -> &Vec<u32> {
        let node = self._node(sequence);
        &node.distribution
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    count: u32,
    precedents: HashSet<usize>,
    successors: HashSet<usize>,
    distribution: Vec<u32>,
}
impl Node {
    fn new(alphabet_size: usize) -> Self {
        Node {
            count: 0,
            precedents: HashSet::new(),
            successors: HashSet::new(),
            distribution: vec![0; alphabet_size],
        }
    }
    fn predecessor_sequences(&self, key: &[usize]) -> Vec<Vec<usize>> {
        let mut pred_seqs = Vec::new();
        for pred_key in &self.precedents {
            let mut pred_seq = vec![*pred_key];
            pred_seq.extend_from_slice(key);
            pred_seqs.push(pred_seq);
        }
        pred_seqs
    }
}

fn count_subsequences(
    data: Vec<Vec<usize>>,
    alphabet_size: usize,
    max_depth: usize,
) -> HashMap<Vec<usize>, Node> {
    let mut counts: HashMap<Vec<usize>, Node> = HashMap::new();
    let mut zero_counts: u32 = 0;
    let mut zero_successors: HashSet<usize> = HashSet::new();
    let mut zero_distribution: Vec<u32> = vec![0; alphabet_size];
    for seq in data {
        zero_counts += seq.len() as u32;
        let n = seq.len().min(max_depth);
        for i in 0..seq.len() {
            //update root node
            zero_successors.insert(seq[i]);
            zero_distribution[seq[i]] += 1;
            //update higher order memory
            for j in i..(i + n).min(seq.len()) {
                let subseq = &seq[i..=j];
                let node = counts
                    .entry(subseq.to_vec())
                    .or_insert(Node::new(alphabet_size));
                node.count += 1;

                if i > 0 {
                    node.precedents.insert(seq[i - 1]);
                }
                if j < seq.len() - 1 {
                    let element = seq[j + 1];
                    node.successors.insert(element);
                    node.distribution[element] += 1
                }
            }
        }
    }
    let zero_node = Node {
        count: zero_counts,
        precedents: HashSet::new(),
        successors: zero_successors,
        distribution: zero_distribution,
    };
    counts.insert(vec![], zero_node);
    counts
}
