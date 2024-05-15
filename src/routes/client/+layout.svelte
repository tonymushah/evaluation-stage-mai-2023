<script lang="ts">
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
