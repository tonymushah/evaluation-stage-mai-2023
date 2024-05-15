<script lang="ts">
  import "@fontsource/source-sans-pro";
  import ClientHeader from "$lib/client/componnents/ClientHeader.svelte";
  import {
    Client,
    cacheExchange,
    fetchExchange,
    setContextClient,
  } from "@urql/svelte";
  import type { LayoutServerData } from "./$types";
  export let data: LayoutServerData;
  setContextClient(
    new Client({
      url: `${data.backend}/front-office`,
      exchanges: [fetchExchange, cacheExchange],
    })
  );
</script>

{#if !location.pathname.includes("login")}
  <ClientHeader />
{/if}

<slot />

<style lang="scss">
  :root {
    --primary: #78bbdb;
    --primary-l1: #74b5d3;
    --primary-l2: #6198b1;
    --primary-l3: #4f7a8f;
    --primary-l4: #3c5d6d;
    --primary-l5: #29404b;
    --secondary: #dbc379;
    --secondary-l2: #dbae79;
    --text-color: #1e2933;
    font-family: "Source Sans Pro", sans-serif;
    color: var(--text-color);
    font-size: 18px;
  }
</style>
