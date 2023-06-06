# Variable Length Markov Model (VLMC)
Implementation of Variable Length Markov Chains (VLMC) for Python.
Suffix tree building is done top-down using the ![Peres-Shield](https://link.springer.com/chapter/10.1007/11557067_24) order estimation method.
It is written in Rust and ported to Python with PyO3.

# Instalation
You can install using `pip`.

```shell
pip install vlmc
```

# Basic Usage

```python
import vlmc
tree = vlmc.VLMC(max_depth, alphabet_size, n_jobs=-1)
```
Parameters:
-`max_depth`: Maximum depth of tree. Subsequences whose length exceed the `max_depth` will not be considered nor counted. 
-`alphabet_size`: Total number of symbols in the alphabet. This number has to be bigger than the highest integer encountered, else it will cause runtime errors. 
-`n_jobs`: Number of subprocesses to spawn when running the vlmc. Choose `-1` for using all available processes.  

# Methods

### `fit`

```python
data = [
[1,2,3],
[2,3],
[1,0,1],
[2]
]

tree.fit(data)
```
> **Note**
> fit method returns `None` and not `self`. This is by design as to not expose the rust object to python.

Parameters:
-`data`: List of lists containing sequences of discrete values to fit on. Values are assumed to be integers form `0` to `alphabet_size`. List is expected to be two dimensional.

### `get_suffix`
Given a sequence, returns the longest suffix that is present in the VLMC.

```python
suffix = tree.get_suffix(sequence)
```
Arguments:
-`sequence`: list of integers representing a sequence of discrete varaibles. 
Returns:
- `suffix` : longest suffix of sequence that is present in the VLMC. 

### `get_counts`
Gets the total number of occurences of a given sequence of integers.
Will throw a `KeyError` if the sequence is not a tree node. Consider using `get_suffix` to make sure to get a tree node.

```python
counts = tree.get_counts(sequence)
```
Arguments:
-`sequence`: list of integers representing a sequence of discrete varaibles. 
Returns:
- `counts` : integer 

### `get_distribution`
Gets the vector of probabilities over the entire alphabet for the given sequence.
Will throw a `KeyError` if the sequence is not a tree node. Consider using `get_suffix` to make sure to get a tree node.

```python
probabilities = tree.get_distribution(sequence)
```
Arguments:
-`sequence`: list of integers representing a sequence of discrete variables. 
Returns:
- `probabilities` : list of floats representing the probability of observing a specific state (index) as the next symbol.

### `get_contexts`

```python
contexts = tree.get_contexts()
```
Returns:
-`contexts`: list of relevant contexts according to the Peres-Shield tree prunning method. Contexts are ordered by length.


## Parameters

### `contexts`
```python
contexts = tree.contexts
```
Returns:
- `contexts` : list of sequences of each node in the tree, each relevant context.
### `adjacency_matrix`
```python
matrix, labels = tree.adjacency_matrix
```
Returns:
- `matrix` : adjacency matrix representing the suffix tree.
- `labels` : list of sequences associated with each node of the tree.

# TODO
### Paralelization
After experimentation the best possible idea for paralelization would be to create different hashmaps for each sunsequence length.
Hashmaps are then joined from longest to smallest.
The hashmap at `max_depth + 1` can be discarded after.
Could be very fast depending on merging algo.
