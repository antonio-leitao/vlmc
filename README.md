# Variable Length Markov Model (VLMC)
Implementation of Variable Length Markov Chains (VLMC) for Python.
Suffix tree building is done top-down using the ![Peres-Shield](https://link.springer.com/chapter/10.1007/11557067_24) order estimation method.
It is written in Rust and ported to Python with PyO3.

# Basic Usage

```python
import vlmc
tree = vlmc.VLMC(max_depth, alphabet_size, n_jobs=-1)
```
Parameters:
-`max_depth`: Maximum depth of tree. Subsequences whose length exceed the `max_depth` will not be considered nor counted. 
-`alphabet_size`: Total number of symbols in the alphabet. 
-`n_jobs`: Number of subprocesses to spawn when running the vlmc. Choose `-1` for using all available processes.  

# Methods

### `fit_transform`

```python
tree.fit_transform(data)
```
Parameters:
-`data`: List of lists containing sequences of discrete values to fit on. Values are assumed to be integers form `0` to `alphabet_size`. List is expected to be two dimensional.

### `get_distribution`
Gets the vector of probabilities over the entire alphabet for the given sequence. If the sequence is not a node then its longest suffix will be used, check the `get_suffix` method below.

```python
probabilities = tree.get_distribution(sequence)
```
Parameters:
-`sequence`: list of integers representing a sequence of discrete varaibles. 
Returns:
- `probabilities` : list of floats representing the probability of observing a specific state (index) as the next symbol.

### `get_context`

```python
suffix = tree.get_suffix(sequence)
```
Parameters:
-`sequence`: list of integers representing a sequence of discrete variables.

Returns:
- `suffix`: list of integers representing the longest suffix of sequence that is present in the VLMC

### `distance_to`

```python
distance = tree.distance_to(VLMC)
```
Returns:
- `distance` : float representing the distance between the VLMC and another instance over the same alphabet. Returns error underlying alphabets are different.

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
