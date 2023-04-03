use hashbrown::HashMap;
use pyo3::prelude::*;

#[pyclass(name = "VLMC")]
struct VLMCObject {
    max_depth: usize,
    alphabet_size: usize,
    #[pyo3(get)]
    counts: usize,
}

#[pymethods]
impl VLMCObject {
    #[new]
    #[pyo3(signature = (max_depth, alphabet_size))]
    fn new(max_depth: usize, alphabet_size: usize) -> Self {
        VLMCObject {
            max_depth: max_depth,
            alphabet_size: alphabet_size,
            counts: 0,
        }
    }
    #[pyo3(signature = (data))]
    fn fit(&mut self, data: Vec<Vec<u32>>) -> PyResult<()> {
        self.counts = count_sequences(data, self.max_depth);
        Ok(())
    }
}

//#[pyfunction]
fn count_sequences(vectors: Vec<Vec<u32>>, n: usize) -> usize {
    let mut counts = HashMap::new();
    for vector in vectors {
        for i in 0..vector.len() {
            for j in i..std::cmp::min(i + n, vector.len()) {
                let sequence = &vector[i..j + 1];
                *counts.entry(sequence.to_vec()).or_insert(0) += 1;
            }
        }
    }
    counts.into_values().sum()
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn vlmc(_py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(count_sequences, m)?)?;
    m.add_class::<VLMCObject>()?;

    Ok(())
}
