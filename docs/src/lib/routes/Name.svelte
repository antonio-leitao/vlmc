<script>
  export let num;
  export let params = {};
</script>

<h2>Hi there!</h2>

<p>
  route props:
  <b>{params.first}</b>
  <b
    >{#if params.last}{params.last}{/if}</b
  >
</p>

<p>
  static props: {num}
</p>
