## `vlmc.VLMC(alphabet_size, method ="peres-shield", max_depth=2)`

Estimates a Variable Length Markov Chain model from an input sequence of integers. The standard Markov model assumes that the next state in a sequence depends on the previous $k$ observations. Variable Length Markov Chain models generalize standard Markoc Chain models as the finite memory value is directly measured from the data and depends on the observed sequence.

VLMCs provide the large memory advantage of higher-order Markov chains when needed, without the drawback of having too many unnecessary parameters in the model. Fitting a VLMC is the process of deciding how much memory is necessary to model specific sequence.

Precisely how this comparison is done depends on the method chosed. The standard metho

##### Parameters
- `alphabet_size`: `int` 
    - Total number of symbols in the alphabet. This number has to be bigger than the highest integer encountered, else it will cause runtime errors. 
- `method`: `string` 
    - Method used when prunning the suffix tree. Options are `"peres-shield"` or `"kl-divergence"`. Default is `"peres-shield"` see below for more information.
- `max_depth`: `int`
    -  Maximum depth of tree. Subsequences whose length exceed the `max_depth` will not be considered nor counted. 

##### Returns
- `tree`: `VLMC`

##### Example

```python
import vlmc
tree = vlmc.VLMC(alphabet_size,max_depth=10)
```

##### Methods
- `fit()` : estimates a VLMC from an input sequence.
- `get_contexts()` : returns the list of all contexts.
- `get_suffix()` : returns the longest context given a sequence.
- `get_counts()` : returns the longest context given a sequence.
- `get_distribution()` : returns transition distribution given a sequence.
- `distance_to()` : calculates the distance to another `VLMC` object.


### `.fit(data)`
> **Note**
> fit method returns `None`. This is by design as to not expose the rust object to python.


Estimates a Variable Length Markov Chain model from a sequence of integers, using the previously chosed method. 

##### Parameters
- `data`:`array-like`
    - List of lists containing sequences of discrete values to fit on. Values are assumed to be integers form `0` to `alphabet_size-1`. List can be two dimensional or one dimensional. If the list is two dimensional it is assumed that there is no transition between last element of row and the first element of the next row.

##### Returns
- `None`

##### Example
```python
import vlmc
data = [
  [1,2,3],
  [2,3],
  [1,0,1],
  [2]
]
tree = vlmc.VLMC(alphabet_size=4)
tree.fit(data)
```


### `.get_suffix(sequence)`
Given a sequence, returns the longest suffix that is present in the VLMC.

##### Parameters
- `sequence`: `array-like`
    - list of integers representing a sequence of discrete varaibles. 

##### Returns
- `suffix`: `array-like`
    - longest suffix of sequence that is present in the `VLMC`. 

##### Example
```python
suffix = tree.get_suffix(sequence)
```


### `.get_counts(sequence)`
Gets the total number of occurences of a given sequence of integers.

> **Warning**: Will throw a `KeyError` if the sequence is not a context. Consider using `get_suffix` to make sure to get a tree node.

##### Parameters
- `sequence`: `array-like`
    - List of integers representing a sequence of discrete varaibles.

##### Returns
- `counts`: `int`
    - Total Number of occurences of `sequence`

##### Example
```python
counts = tree.get_counts(sequence)
```

### `.get_distribution(sequence)`
Gets the vector of transition probabilities over the entire alphabet for the given sequence.

> **Warning**: Will throw a `KeyError` if the sequence is not a tree node. Consider using `get_suffix` to make sure to get a tree node.


##### Parameters
- `sequence`: `array-like`
    - list of integers representing a sequence of discrete variables. 

##### Returns
- `distribution`:`array-like`
    - list of floats representing the probability of observing a specific state (index) as the next symbol.

##### Example
```python
distribution = tree.get_distribution(sequence)
```


### `.get_contexts()`
Outputs the list of all statistically relevant contexts.

##### Returns
- `contexts`: `array-like`
    - list of relevant contexts according to the chosen tree prunning method. Contexts are ordered by length.

##### Example
```python
contexts = tree.get_contexts()
```

### `.distance_to(second_tree, p=2)`
> **Warning** under construction

Computes the distance between two different Variable Length Markov Models over the same alphabet. The distance is not symmetric.

##### Parameters
- `second_tree`: `VLMC`
    - list of relevant contexts according to the Peres-Shield tree prunning method. Contexts are ordered by length.
- `p`: `int`
    - Order of the Mikowsky distance to be considered.

##### Returns
- `distance`:`float`
    - Distance between the `VLMC` object and a second one.

##### Example
```python
distance = tree.distance_to(tree2, p=2)
```
