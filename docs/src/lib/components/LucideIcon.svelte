<script>
  import {
    Info,
    Play,
    Clipboard,
    List,
    FlaskConical,
    ArrowLeft,
    Copy,
    FileUp,
    FlipHorizontal,
    Library,
    XOctagon,
    Layers,
    Layout,
    Glasses,
    Palette,
    LayoutGrid,
    Paintbrush,
    Mouse,
    Hourglass,
    Check,
    FileText,
    Files,
    Home,
  } from "lucide-svelte";
  export let name;
</script>

{#if name === "info"}
  <svelte:component this={Info} {...$$props} />
{:else if name === "play"}
  <svelte:component this={Play} {...$$props} />
{:else if name === "clipboard"}
  <svelte:component this={Clipboard} {...$$props} />
{:else if name === "list"}
  <svelte:component this={List} {...$$props} />
{:else if name === "arrow-left"}
  <svelte:component this={ArrowLeft} {...$$props} />
{:else if name === "file-up"}
  <svelte:component this={FileUp} {...$$props} />
{:else if name === "copy"}
  <svelte:component this={Copy} {...$$props} />
{:else if name === "flip-horizontal"}
  <svelte:component this={FlipHorizontal} {...$$props} />
{:else if name === "library"}
  <svelte:component this={Library} {...$$props} />
{:else if name === "layers"}
  <svelte:component this={Layers} {...$$props} />
{:else if name === "layout"}
  <svelte:component this={Layout} {...$$props} />
{:else if name === "glasses"}
  <svelte:component this={Glasses} {...$$props} />
{:else if name === "palette"}
  <svelte:component this={Palette} {...$$props} />
{:else if name === "layout-grid"}
  <svelte:component this={LayoutGrid} {...$$props} />
{:else if name === "paintbrush"}
  <svelte:component this={Paintbrush} {...$$props} />
{:else if name === "mouse"}
  <svelte:component this={Mouse} {...$$props} />
{:else if name === "hourglass"}
  <svelte:component this={Hourglass} {...$$props} />
{:else if name === "check"}
  <svelte:component this={Check} {...$$props} />
{:else if name === "file text"}
  <svelte:component this={FileText} {...$$props} />
{:else if name === "files"}
  <svelte:component this={Files} {...$$props} />
{:else if name === "test"}
  <svelte:component this={FlaskConical} {...$$props} />
{:else if name === "home"}
  <svelte:component this={Home} {...$$props} />
{:else}
  <svelte:component this={XOctagon} {...$$props} />
{/if}
