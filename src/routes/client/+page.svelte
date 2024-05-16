<script lang="ts">
  import SimpleChatierComponnent from "$lib/client/componnents/chantier/SimpleChatierComponnent.svelte";
  import { graphql } from "$lib/client/gql";
  import { CombinedError, getContextClient } from "@urql/svelte";
  import { onDestroy, onMount, type ComponentProps } from "svelte";
  import { derived, writable } from "svelte/store";
  import cookie from "js-cookie";
  import type { PageServerData } from "./$types";
  import { clientTokenStore } from "$lib/client/stores/clientTokenStore";

  export let data: PageServerData;

  clientTokenStore.set(data.clientToken);

  const client = getContextClient();
  const query = graphql(`
    query clientListChantier($page: OffsetLimit!) {
      chantiers {
        list(pagination: $page) {
          data {
            id
            dateDebut
            finition {
              duree
              designation
              prix
            }
            typeChantier {
              nom
            }
            devis {
              prixTotal
            }
          }
          offset
          total
          limit
        }
      }
    }
  `);
  const devisStore = writable(
    new Map<string, ComponentProps<SimpleChatierComponnent>>()
  );
  const devis = derived(devisStore, ($d) =>
    Array.from($d, ([id, props]) => ({ id, props }))
  );
  let error: CombinedError | undefined = undefined;
  let offset = 0;
  const limit = 10;
  let total: number | undefined = undefined;
  $: isAtEnd = total != undefined && offset >= total;
  async function execute(offset: number, limit: number) {
    if (!isAtEnd) {
      const result = await client
        .query(
          query,
          {
            page: {
              offset,
              limit,
            },
          },
          {
            fetchOptions: () => {
              const headers = new Headers();
              const token = $clientTokenStore;
              if (token) headers.set("authorization", token);
              return {
                headers,
              };
            },
          }
        )
        .toPromise();
      if (result.data) {
        devisStore.update((m) => {
          result.data?.chantiers.list.data.forEach((d) => {
            m.set(d.id, {
              chantier: d.typeChantier.nom,
              prixTotal: Number(d.devis.prixTotal) + Number(d.finition.prix),
              etat: "Invalid",
              ETA: 0,
            });
          });
          return m;
        });
        return [offset + limit, result.data.chantiers.list.total];
      } else if (result.error) {
        error = result.error;
      }
    }
  }

  let observer: HTMLDivElement | undefined = undefined;
  const interObserver = new IntersectionObserver(async () => {
    if (!isAtEnd) {
      const o = await execute(offset, limit);
      if (o) {
        offset = o[0];
        total = o[1];
      }
    }
  });
  $: if (observer) {
    interObserver.observe(observer);
  }
  onMount(async () => {
    const o = await execute(offset, limit);
    if (o) {
      offset = o[0];
      total = o[1];
    }
  });
  onDestroy(() => {
    interObserver.disconnect();
  });
</script>

<div class="devis">
  {#each $devis as { id, props } (id)}
    <SimpleChatierComponnent {...props} />
  {:else}
    <div class="nothing">Vous avez aucun travaux en cours...</div>
  {/each}
</div>
{#if !isAtEnd}
  <div bind:this={observer} />
{/if}

<style lang="scss">
  .devis {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    padding: 5px;
    gap: 5px;
  }
</style>
