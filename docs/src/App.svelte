<script>
  import Router, { replace, location } from "svelte-spa-router";
  import routes from "./routes.js";
  import Sidebar from "./lib/components/Sidebar.svelte";

  function conditionsFailed(event) {
    console.error("conditionsFailed event", event.detail);

    // Perform any action, for example replacing the current route
    if (event.detail.userData.foo == "bar") {
      replace("/hello/world");
    }
  }
</script>
<div class="layout">
    {#if $location==="/documentation"}
        <Sidebar/>
    {/if}
    <div class="container" style="width: {$location==='/documentation' ? '75vw':'100vw'};">
    
        <Router {routes} on:conditionsFailed={conditionsFailed} />
    </div>
</div>

<style>

.layout{
    display: grid;
    grid-template-columns: minmax(150px, 20%) 1fr;
  }

.container{
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-top:2rem;
  }
</style>
