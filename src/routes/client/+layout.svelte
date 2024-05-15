<script>
  import ClientHeader from "$lib/client/componnents/ClientHeader.svelte";
</script>

{#if !location.pathname.includes("login")}
  <ClientHeader />
{/if}

<slot />
