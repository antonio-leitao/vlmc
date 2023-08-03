<script>
  import { location, link} from "svelte-spa-router";
</script>   

<div class="layout">
    <div class="sidebar">
        <p>
            Current location = {$location}
        </p>
        <nav>
            <a href="/" use:link>Home</a>
            <a href="/hello/antonio/leitao" use:link>Hello</a>
            <a href="/wild/colors" use:link>Colors</a>
        </nav>
    </div>
</div>

<style>
    
.layout{
    display:grid;
    grid-template-columns: minmax(150px, 15%) 1fr;
}
.sidebar{
    background-color: var(--light-green);
}
</style>>
