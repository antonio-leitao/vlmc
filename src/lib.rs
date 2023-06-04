use hashbrown::{HashMap, HashSet};
use pyo3::prelude::*;
//use std::collections::VecDeque;
//use std::thread;
mod peres_shield;
use self::peres_shield::peres_shield_prunning;

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
    counts: HashMap<String, u32>,
}

#[pymethods]
impl VLMCObject {
    #[new]
    fn new() -> Self {
        VLMCObject {
            counts: HashMap::new(),
        }
    }

    #[pyo3(signature = (data))]
    fn fit(&mut self, data: Vec<&str>) -> PyResult<()> {
        self._fit(data);
        Ok(())
    }
}

//############################# BACKEND ##########################
//Here should be methods and stuff that will not be exposed to Python

impl VLMCObject {
    fn _fit(&mut self, data: Vec<&str>) {
        //creates counts
        let mut raw_counts: HashMap<String, u32> = HashMap::new();
        for string in data {
            *raw_counts.entry(string.to_string()).or_insert(0) += 1;
        }
        //prunes counts
        self.counts = peres_shield_prunning(raw_counts);
    }
}
#[derive(Debug, PartialEq)]
struct Node {
    count: usize,
    precedents: HashSet<u32>,
    successors: HashSet<u32>,
    distribution: Vec<u32>,
}
