<script>
  function scrollIntoView({ target }) {
    const el = document.querySelector(target.getAttribute("href"));
    if (!el) return;
    el.scrollIntoView({
      behavior: "smooth",
    });
  }
</script>

<div class="markdown-body">
<h2>
  <a
    id="user-content-vlmcvlmcalphabet_size-method-peres-shield-max_depth2"
    class="anchor"
    aria-hidden="true"
    href="#vlmcvlmcalphabet_size-method-peres-shield-max_depth2"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  ><code>vlmc.VLMC(alphabet_size, method ="peres-shield", max_depth=2)</code>
</h2>
<p>
  Estimates a Variable Length Markov Chain model from an input sequence of
  integers. The standard Markov model assumes that the next state in a sequence
  depends on the previous $k$ observations. Variable Length Markov Chain models
  generalize standard Markoc Chain models as the finite memory value is directly
  measured from the data and depends on the observed sequence.
</p>
<p>
  VLMCs provide the large memory advantage of higher-order Markov chains when
  needed, without the drawback of having too many unnecessary parameters in the
  model. Fitting a VLMC is the process of deciding how much memory is necessary
  to model specific sequence.
</p>
<p>
  Precisely how this comparison is done depends on the method chosed. The
  standard metho
</p>
<h5>
  <a
    id="user-content-parameters"
    class="anchor"
    aria-hidden="true"
    href="#parameters"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Parameters
</h5>
<ul>
  <li>
    <code>alphabet_size</code>: <code>int</code>
    <ul>
      <li>
        Total number of symbols in the alphabet. This number has to be bigger
        than the highest integer encountered, else it will cause runtime errors.
      </li>
    </ul>
  </li>
  <li>
    <code>method</code>: <code>string</code>
    <ul>
      <li>
        Method used when prunning the suffix tree. Options are
        <code>"peres-shield"</code> or <code>"kl-divergence"</code>. Default is
        <code>"peres-shield"</code> see below for more information.
      </li>
    </ul>
  </li>
  <li>
    <code>max_depth</code>: <code>int</code>
    <ul>
      <li>
        Maximum depth of tree. Subsequences whose length exceed the
        <code>max_depth</code> will not be considered nor counted.
      </li>
    </ul>
  </li>
</ul>
<h5>
  <a id="user-content-returns" class="anchor" aria-hidden="true" href="#returns"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Returns
</h5>
<ul>
  <li><code>tree</code>: <code>VLMC</code></li>
</ul>
<h5>
  <a id="user-content-example" class="anchor" aria-hidden="true" href="#example"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Example
</h5>
<div class="highlight highlight-source-python">
  <pre><span class="pl-k">import</span> <span class="pl-s1">vlmc</span>
<span class="pl-s1">tree</span> <span class="pl-c1">=</span> <span class="pl-s1">vlmc</span>.<span class="pl-v">VLMC</span>(<span class="pl-s1">alphabet_size</span>,<span class="pl-s1">max_depth</span><span class="pl-c1">=</span><span class="pl-c1">10</span>)</pre>
</div>
<h5>
  <a id="user-content-methods" class="anchor" aria-hidden="true" href="#methods"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Methods
</h5>
<ul>
  <li><code>fit()</code> : estimates a VLMC from an input sequence.</li>
  <li><code>get_contexts()</code> : returns the list of all contexts.</li>
  <li>
    <code>get_suffix()</code> : returns the longest context given a sequence.
  </li>
  <li>
    <code>get_counts()</code> : returns the longest context given a sequence.
  </li>
  <li>
    <code>get_distribution()</code> : returns transition distribution given a
    sequence.
  </li>
  <li>
    <code>distance_to()</code> : calculates the distance to another
    <code>VLMC</code> object.
  </li>
</ul>
<h3>
  <a id="user-content-fitdata" class="anchor" aria-hidden="true" href="#fitdata"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  ><code>.fit(data)</code>
</h3>
<blockquote>
  <p>
    <strong>Note</strong> fit method returns <code>None</code>. This is by
    design as to not expose the rust object to python.
  </p>
</blockquote>
<p>
  Estimates a Variable Length Markov Chain model from a sequence of integers,
  using the previously chosed method.
</p>
<h5>
  <a
    id="user-content-parameters-1"
    class="anchor"
    aria-hidden="true"
    href="#parameters-1"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Parameters
</h5>
<ul>
  <li>
    <code>data</code>:<code>array-like</code>
    <ul>
      <li>
        List of lists containing sequences of discrete values to fit on. Values
        are assumed to be integers form <code>0</code> to
        <code>alphabet_size-1</code>. List can be two dimensional or one
        dimensional. If the list is two dimensional it is assumed that there is
        no transition between last element of row and the first element of the
        next row.
      </li>
    </ul>
  </li>
</ul>
<h5>
  <a
    id="user-content-returns-1"
    class="anchor"
    aria-hidden="true"
    href="#returns-1"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Returns
</h5>
<ul>
  <li><code>None</code></li>
</ul>
<h5>
  <a
    id="user-content-example-1"
    class="anchor"
    aria-hidden="true"
    href="#example-1"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Example
</h5>
<div class="highlight highlight-source-python">
  <pre><span class="pl-k">import</span> <span class="pl-s1">vlmc</span>
<span class="pl-s1">data</span> <span class="pl-c1">=</span> [
  [<span class="pl-c1">1</span>,<span class="pl-c1">2</span>,<span class="pl-c1">3</span>],
  [<span class="pl-c1">2</span>,<span class="pl-c1">3</span>],
  [<span class="pl-c1">1</span>,<span class="pl-c1">0</span>,<span class="pl-c1">1</span>],
  [<span class="pl-c1">2</span>]
]
<span class="pl-s1">tree</span> <span class="pl-c1">=</span> <span class="pl-s1">vlmc</span>.<span class="pl-v">VLMC</span>(<span class="pl-s1">alphabet_size</span><span class="pl-c1">=</span><span class="pl-c1">4</span>)
<span class="pl-s1">tree</span>.<span class="pl-en">fit</span>(<span class="pl-s1">data</span>)</pre>
</div>
<h3>
  <a
    id="user-content-get_contexts"
    class="anchor"
    aria-hidden="true"
    href="#get_contexts"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  ><code>.get_contexts()</code>
</h3>
<p>Outputs the list of all statistically relevant contexts.</p>
<h5>
  <a
    id="user-content-returns-5"
    class="anchor"
    aria-hidden="true"
    href="#returns-5"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Returns
</h5>
<ul>
  <li>
    <code>contexts</code>: <code>array-like</code>
    <ul>
      <li>
        list of relevant contexts according to the chosen tree prunning method.
        Contexts are ordered by length.
      </li>
    </ul>
  </li>
</ul>
<h5>
  <a
    id="user-content-example-5"
    class="anchor"
    aria-hidden="true"
    href="#example-5"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Example
</h5>
<div class="highlight highlight-source-python">
  <pre><span class="pl-s1">contexts</span> <span class="pl-c1">=</span> <span class="pl-s1">tree</span>.<span class="pl-en">get_contexts</span>()</pre>
</div>
<h3>
  <a
    id="user-content-get_suffixsequence"
    class="anchor"
    aria-hidden="true"
    href="#get_suffixsequence"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  ><code>.get_suffix(sequence)</code>
</h3>
<p>Given a sequence, returns the longest suffix that is present in the VLMC.</p>
<h5>
  <a
    id="user-content-parameters-2"
    class="anchor"
    aria-hidden="true"
    href="#parameters-2"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Parameters
</h5>
<ul>
  <li>
    <code>sequence</code>: <code>array-like</code>
    <ul>
      <li>list of integers representing a sequence of discrete varaibles.</li>
    </ul>
  </li>
</ul>
<h5>
  <a
    id="user-content-returns-2"
    class="anchor"
    aria-hidden="true"
    href="#returns-2"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Returns
</h5>
<ul>
  <li>
    <code>suffix</code>: <code>array-like</code>
    <ul>
      <li>
        longest suffix of sequence that is present in the <code>VLMC</code>.
      </li>
    </ul>
  </li>
</ul>
<h5>
  <a
    id="user-content-example-2"
    class="anchor"
    aria-hidden="true"
    href="#example-2"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Example
</h5>
<div class="highlight highlight-source-python">
  <pre><span class="pl-s1">suffix</span> <span class="pl-c1">=</span> <span class="pl-s1">tree</span>.<span class="pl-en">get_suffix</span>(<span class="pl-s1">sequence</span>)</pre>
</div>
<h3>
  <a
    id="user-content-get_countssequence"
    class="anchor"
    aria-hidden="true"
    href="#get_countssequence"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  ><code>.get_counts(sequence)</code>
</h3>
<p>Gets the total number of occurences of a given sequence of integers.</p>
<blockquote>
  <p>
    <strong>Warning</strong>: Will throw a <code>KeyError</code> if the sequence
    is not a context. Consider using <code>get_suffix</code> to make sure to get
    a tree node.
  </p>
</blockquote>
<h5>
  <a
    id="user-content-parameters-3"
    class="anchor"
    aria-hidden="true"
    href="#parameters-3"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Parameters
</h5>
<ul>
  <li>
    <code>sequence</code>: <code>array-like</code>
    <ul>
      <li>List of integers representing a sequence of discrete varaibles.</li>
    </ul>
  </li>
</ul>
<h5>
  <a
    id="user-content-returns-3"
    class="anchor"
    aria-hidden="true"
    href="#returns-3"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Returns
</h5>
<ul>
  <li>
    <code>counts</code>: <code>int</code>
    <ul>
      <li>Total Number of occurences of <code>sequence</code></li>
    </ul>
  </li>
</ul>
<h5>
  <a
    id="user-content-example-3"
    class="anchor"
    aria-hidden="true"
    href="#example-3"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Example
</h5>
<div class="highlight highlight-source-python">
  <pre><span class="pl-s1">counts</span> <span class="pl-c1">=</span> <span class="pl-s1">tree</span>.<span class="pl-en">get_counts</span>(<span class="pl-s1">sequence</span>)</pre>
</div>
<h3>
  <a
    id="user-content-get_distributionsequence"
    class="anchor"
    aria-hidden="true"
    href="#get_distributionsequence"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  ><code>.get_distribution(sequence)</code>
</h3>
<p>
  Gets the vector of transition probabilities over the entire alphabet for the
  given sequence.
</p>
<blockquote>
  <p>
    <strong>Warning</strong>: Will throw a <code>KeyError</code> if the sequence
    is not a tree node. Consider using <code>get_suffix</code> to make sure to
    get a tree node.
  </p>
</blockquote>
<h5>
  <a
    id="user-content-parameters-4"
    class="anchor"
    aria-hidden="true"
    href="#parameters-4"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Parameters
</h5>
<ul>
  <li>
    <code>sequence</code>: <code>array-like</code>
    <ul>
      <li>list of integers representing a sequence of discrete variables.</li>
    </ul>
  </li>
</ul>
<h5>
  <a
    id="user-content-returns-4"
    class="anchor"
    aria-hidden="true"
    href="#returns-4"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Returns
</h5>
<ul>
  <li>
    <code>distribution</code>:<code>array-like</code>
    <ul>
      <li>
        list of floats representing the probability of observing a specific
        state (index) as the next symbol.
      </li>
    </ul>
  </li>
</ul>
<h5>
  <a
    id="user-content-example-4"
    class="anchor"
    aria-hidden="true"
    href="#example-4"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Example
</h5>
<div class="highlight highlight-source-python">
  <pre><span class="pl-s1">distribution</span> <span class="pl-c1">=</span> <span class="pl-s1">tree</span>.<span class="pl-en">get_distribution</span>(<span class="pl-s1">sequence</span>)</pre>
</div>
<h3>
  <a
    id="user-content-distance_tosecond_tree-p2"
    class="anchor"
    aria-hidden="true"
    href="#distance_tosecond_tree-p2"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  ><code>.distance_to(second_tree, p=2)</code>
</h3>
<blockquote>
  <p><strong>Warning</strong> under construction</p>
</blockquote>
<p>
  Computes the distance between two different Variable Length Markov Models over
  the same alphabet. The distance is not symmetric.
</p>
<h5>
  <a
    id="user-content-parameters-5"
    class="anchor"
    aria-hidden="true"
    href="#parameters-5"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Parameters
</h5>
<ul>
  <li>
    <code>second_tree</code>: <code>VLMC</code>
    <ul>
      <li>
        list of relevant contexts according to the Peres-Shield tree prunning
        method. Contexts are ordered by length.
      </li>
    </ul>
  </li>
  <li>
    <code>p</code>: <code>int</code>
    <ul>
      <li>Order of the Mikowsky distance to be considered.</li>
    </ul>
  </li>
</ul>
<h5>
  <a
    id="user-content-returns-6"
    class="anchor"
    aria-hidden="true"
    href="#returns-6"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Returns
</h5>
<ul>
  <li>
    <code>distance</code>:<code>float</code>
    <ul>
      <li>Distance between the <code>VLMC</code> object and a second one.</li>
    </ul>
  </li>
</ul>
<h5>
  <a
    id="user-content-example-6"
    class="anchor"
    aria-hidden="true"
    href="#example-6"
    ><span aria-hidden="true" class="octicon octicon-link"></span></a
  >Example
</h5>
<div class="highlight highlight-source-python">
  <pre><span class="pl-s1">distance</span> <span class="pl-c1">=</span> <span class="pl-s1">tree</span>.<span class="pl-en">distance_to</span>(<span class="pl-s1">tree2</span>, <span class="pl-s1">p</span><span class="pl-c1">=</span><span class="pl-c1">2</span>)</pre>
</div>
</div>
