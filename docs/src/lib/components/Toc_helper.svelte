<script>
  import { fly } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  export let expanded = false;
  export let name;
  export let children;
  import { ChevronUp, ChevronDown } from "lucide-svelte";

  function toggle() {
    expanded = !expanded;
  }
</script>

<span class:expanded on:click={toggle}
  >{name}
  {#if expanded}
    <ChevronUp />
  {:else}
    <ChevronDown />
  {/if}
</span>

{#if expanded}
  <ul>
    {#each children as child, i}
      <li>
        {#if child.children}
          <div
            in:fly={{ y: -20, duration: 300, delay: i * 50, easing: quintOut }}
          >
            <svelte:self {...child} />
          </div>
        {:else}
          <p
            in:fly={{ y: -20, duration: 300, delay: i * 50, easing: quintOut }}
          >
            {child.name}
          </p>
        {/if}
      </li>
    {/each}
  </ul>
{/if}

<style>
  * {
    font-family: Inter, system-ui, sans-serif;
    font-weight: 400;
  }
  span {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    padding: 0.5em;
    font-weight: bold;
    cursor: pointer;
  }

  span:hover,
  p:hover {
    color: var(--highlight);
  }

  p {
    cursor: pointer;
    padding: 0.5em;
    margin: 0;
  }

  ul {
    padding: 0em 0 0 0.5em;
    margin: 0 0 0 0.5em;
    list-style: none;
    border-left: 1px solid #eee;
  }
</style>
