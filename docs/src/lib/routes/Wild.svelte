<script>
  // The params prop contains values matched from the URL
  export let params = {};
</script>

<h2>Wildcard</h2>

<p>The difference is wild works as a dict: params[wild]={params.wild}</p>

<p>
  Note that this route was artificially delayed for 5 seconds to let us see the <code
    >Loading</code
  > component even when the network is fast (such as testing on localhost).
</p>
