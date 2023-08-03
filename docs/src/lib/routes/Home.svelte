<script>
  import { link } from "svelte-spa-router";
  import LucideIcon from "../components/LucideIcon.svelte";
  import CopyToClipboard from "../components/CopyToClipboard.svelte";
  const options = [
      {
          name: "Gettings Started",
          description: "Installation and basic usage",
          href: "/start",
          icon: "play",
      },
    {
      name: "Documentation",
      description: "API documentation",
      href: "/documentation",
      icon: "info",
    },
    {
      name: "Examples",
      description: "List of curated examples.",
      href: "/examples",
      icon: "test",
    },
  ];
</script>

<div class="outer">
<div class="logo">
        <svg id="Layer_2" xmlns="http://www.w3.org/2000/svg" width="100%" viewBox="0 0 301.196 296.968"><g id="Layer_1-2"><circle cx="52.812" cy="52.6" r="13.195" transform="translate(-16.109 24.214) rotate(-22.5)"/><path d="m42.467,82.654c-12.354-4.287-21.227-16.021-21.227-29.834,0-17.441,14.139-31.579,31.579-31.579,14.879,0,27.347,10.293,30.691,24.146l22.114,6.86C105.315,23.34,81.798,0,52.819,0,23.648,0,0,23.648,0,52.819c0,26.898,20.111,49.084,46.114,52.381l-3.647-22.546Z"/><path d="m75.846,74.419c-1.161,1.237-2.424,2.375-3.769,3.413l3.653,22.587c8.902-4.297,16.414-11.02,21.683-19.309l-21.568-6.691Z"/><path d="m90.832,191.733l4.024,24.039c11.544,4.256,19.777,15.354,19.777,28.376,0,16.698-13.537,30.235-30.235,30.235s-30.235-13.537-30.235-30.235c0-9.44,4.328-17.868,11.107-23.413l-4.029-24.066c-17.562,8.582-29.662,26.615-29.662,47.479,0,29.171,23.648,52.819,52.819,52.819s52.819-23.648,52.819-52.819c0-26.992-20.251-49.239-46.385-52.415Z"/><circle cx="84.399" cy="244.351" r="13.195" transform="translate(-38.051 16.674) rotate(-9.217)"/><path d="m248.377,59.417c-18.317,0-34.452,9.327-43.927,23.487l23.461,7.278c5.37-4.986,12.56-8.038,20.465-8.038,16.62,0,30.093,13.473,30.093,30.093s-13.473,30.093-30.093,30.093c-14.352,0-26.35-10.05-29.358-23.495l-23.444-7.273c-.003.226-.017.449-.017.675,0,29.171,23.648,52.819,52.819,52.819s52.819-23.648,52.819-52.819-23.648-52.819-52.819-52.819Z"/><circle cx="248.377" cy="112.236" r="13.195" transform="translate(-7.331 17.89) rotate(-4.065)"/></g></svg>

    </div>
  <div class="banner">VLMC</div>
  <div class="subtitle">variable length markov chain</div>
    <CopyToClipboard text={"pip install vlmc"}/>   
    <div class="subtext"> 
        Variable Length Markov Model is a rust based Python Package Applied for some stuff and some others too. Check the documentation below
    </div>
  <div class="component_grid">
    {#each options as option}
      <a href={option.href} use:link>
        <div class="card">
          <div class="overview">
            <div class="right">
                <div class="circle">
                    <LucideIcon name={option.icon} size="36" />
                </div>
            </div>
            <div class="left">
              <h2>{option.name}</h2>
              <p>{option.description}</p>
            </div>
          </div>
        </div>
      </a>
    {/each}
  </div>
</div>

<style>
  .outer {
    display: flex;
    position: relative;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 820px;
  }
  .logo {
      width:50%;
      position:absolute;
      top:0;
      z-index: -1;
      stroke: rgba(0,0,0,0.05);
      fill: rgba(0,0,0,0.15);
  }
  
  .banner {
      font-size: 6rem;
      font-family: Helvetica, sans-serif;
      font-weight: 900;
      margin-top:8rem;
  }
  .subtitle {
      font-size: 0.6rem;
      color: var(--offdark);
      font-family: Helvetica, sans-serif;
      text-transform:uppercase;
      letter-spacing:0.2rem;
      font-weight:100;
      margin-bottom:5rem;
  }
  .component_grid {
    display: flex;
    align-items: center;
    justify-content: left;
    width: 100%;
    flex-wrap: wrap;
  }
  .card {
    cursor: pointer;
    margin: 0.5rem;
    background: var(--bg-secondary);
    border-radius: 5px;
    overflow: hidden;
    width: 222px;
    height: 75px;
    position: relative;
    display: flex;
    align-items: flex-end;
    padding: 1rem;
    padding-left: 1.2rem;
    transition: all 0.1s ease;
    overflow-y: hidden;
  }

  .card:hover {
    box-shadow: var(--hover-shadow);
  }

  .overview {
    display: flex;
    flex-direction: row;
    justify-content: left;
    align-items: center;
    position: absolute;
    top: 1.8rem;
    width: 88%;
  }
  .overview .left{
    margin-left: 0.8rem;
  }

  .overview .left h2 {
    margin: 0;
    font-size: 1.2rem;
  }
  .overview .left p {
    font-size: 0.8rem;
    margin: 0;
  }

  .circle {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border-radius: 100%;
  }

  .subtext {
      color: var(--gray);
      font-size: 1rem;
      line-height:1.1rem;
      font-family: Poppins;
      text-align: center;
      max-width:70ch;
      margin:2rem 0 2rem 0;
  }
  a {
    color: inherit;
    font: inherit;
  }
</style>
