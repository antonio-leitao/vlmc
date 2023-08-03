<script>
  import TocHelper from "./Toc_helper.svelte";
  import LucideIcon from "./LucideIcon.svelte";
  import { link } from "svelte-spa-router";
  const content = [
    {
      name: "VLMC",
      children: [
        { name: "get_suffix" },
        { name: "get_distribution" },
        { name: "get_contexts" },
        { name: "get_adjacency_matrix" },
        { name: "distance_to" },
      ],
    },
    {
      name: "Examples",
      children: [
        { name: "Human Language" },
        { name: "Whalespeak" },
        { name: "DNA Sequencing" },
      ],
    },
  ];

  const docs = {
    featured: [
      { name: "Home", icon: "home", href: "/" },
      { name: "Getting Started", icon: "play", href: "/start" },
      { name: "Examples", icon: "test", href:"/examples"},
    ],
    content: content,
  };
</script>

<div class="tocspace">
    
    <div class="logo">
        <div class="icon">
            <svg id="Layer_2" xmlns="http://www.w3.org/2000/svg" height="100%" viewBox="0 0 301.196 296.968"><g id="Layer_1-2"><circle cx="52.812" cy="52.6" r="13.195" transform="translate(-16.109 24.214) rotate(-22.5)"/><path d="m42.467,82.654c-12.354-4.287-21.227-16.021-21.227-29.834,0-17.441,14.139-31.579,31.579-31.579,14.879,0,27.347,10.293,30.691,24.146l22.114,6.86C105.315,23.34,81.798,0,52.819,0,23.648,0,0,23.648,0,52.819c0,26.898,20.111,49.084,46.114,52.381l-3.647-22.546Z"/><path d="m75.846,74.419c-1.161,1.237-2.424,2.375-3.769,3.413l3.653,22.587c8.902-4.297,16.414-11.02,21.683-19.309l-21.568-6.691Z"/><path d="m90.832,191.733l4.024,24.039c11.544,4.256,19.777,15.354,19.777,28.376,0,16.698-13.537,30.235-30.235,30.235s-30.235-13.537-30.235-30.235c0-9.44,4.328-17.868,11.107-23.413l-4.029-24.066c-17.562,8.582-29.662,26.615-29.662,47.479,0,29.171,23.648,52.819,52.819,52.819s52.819-23.648,52.819-52.819c0-26.992-20.251-49.239-46.385-52.415Z"/><circle cx="84.399" cy="244.351" r="13.195" transform="translate(-38.051 16.674) rotate(-9.217)"/><path d="m248.377,59.417c-18.317,0-34.452,9.327-43.927,23.487l23.461,7.278c5.37-4.986,12.56-8.038,20.465-8.038,16.62,0,30.093,13.473,30.093,30.093s-13.473,30.093-30.093,30.093c-14.352,0-26.35-10.05-29.358-23.495l-23.444-7.273c-.003.226-.017.449-.017.675,0,29.171,23.648,52.819,52.819,52.819s52.819-23.648,52.819-52.819-23.648-52.819-52.819-52.819Z"/><circle cx="248.377" cy="112.236" r="13.195" transform="translate(-7.331 17.89) rotate(-4.065)"/></g></svg>
        </div>
        <a href="/" use:link>
        <div class="banner">
            <div>VLMC</div>
            <div class="subtitle">variable length markov chain</div>
        </div>
        </a>
    </div>
  <nav class="featured">
    <ul>
      {#each docs.featured as feature}
        <a href={feature.href} use:link>
            <li class="featured-item">
              <LucideIcon name="{feature.icon}" size=20/>
              <p>{feature.name}</p>
            </li>
        </a>
      {/each}
    </ul>
  </nav>
  <nav class="main">
    {#each content as dir}
      <div class="chapter">{dir.name}</div>
      {#each dir.children as child}
        {#if Object.hasOwn(child, "children")}
          <TocHelper
            name={child.name}
            children={child.children}
            expanded={false}
          />
        {:else}
          <p>{child.name}</p>
        {/if}
      {/each}
    {/each}
  </nav>
</div>

<style>

  .tocspace {
    width: 14rem;
    display:flex;
    flex-direction: column;
    margin:2rem 1rem 0 2rem;
  }

  .logo {
    width: 14rem;
    display:flex;
    flex-direction: row;
    margin-bottom:0.7rem;
  }


  .icon {
      padding: 0.3rem;
  }
  .banner {
      font-size: 3rem;
      font-family: Helvetica, sans-serif;
      font-weight: 900;
      display:flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
  }
  .subtitle {
      font-size: 0.3rem;
      color: var(--offdark);
      font-family: Helvetica, sans-serif;
      text-transform:uppercase;
      letter-spacing:0.1rem;
      font-weight: 100;
  }

  ul {
    padding: 0;
    margin: 0;
    list-style: none;
  }

  .featured-item {
    cursor: pointer;
    border-radius: 0.45rem;
    padding-left: 0.4rem;
  }
  .featured-item:hover {
    color: var(--highlight);
    background: var(--bg-secondary);
    box-shadow: var(--med-shadow);
  }

  .chapter {
    font-family: Helvetica, sans-serif;
    font-family: Inter, system-ui, sans-serif;
    color: var(--gray);
    margin-top: 2rem;
    margin-bottom: 0.5rem;
    margin-left: 0.5rem;
    text-transform: uppercase;
    letter-spacing: 0.1rem;
    font-size: 13px;
    font-weight: 400;
  }

  p {
    cursor: pointer;
    padding: 0.5em;
    margin: 0;
  }

  p:hover {
    color: var(--highlight);
  }
  li {
    display: flex;
    align-items: center;
  }
</style>
