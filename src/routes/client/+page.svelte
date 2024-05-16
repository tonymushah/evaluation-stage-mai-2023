<script lang="ts">
  import SimpleChatierComponnent from "$lib/client/componnents/chantier/SimpleChatierComponnent.svelte";
  import { graphql } from "$lib/client/gql";
  import { CombinedError, getContextClient } from "@urql/svelte";
  import { onDestroy, onMount, type ComponentProps } from "svelte";
  import { derived, writable } from "svelte/store";

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
  async function execute(offset: number, limit: number) {
    if (total == undefined || (total != undefined && offset >= total)) {
      const result = await client
        .query(query, {
          page: {
            offset,
            limit,
          },
        })
        .toPromise();
      if (result.data) {
        devisStore.update((m) => {
          result.data?.chantiers.list.data.forEach((d) => {
            m.set(d.id, {
              chantier: d.typeChantier.nom,
              prixTotal: d.devis.prixTotal + d.finition.prix,
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
  $: isAtEnd = total != undefined && offset >= total;
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

{#each $devis as { id, props } (id)}
  <div class="devis">
    <SimpleChatierComponnent {...props} />
  </div>
{:else}
  <div class="nothing">Vous avez aucun travaux en cours...</div>
{/each}

{#if !isAtEnd}
  <div bind:this={observer} />
{/if}

<style lang="scss">
  .devis {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
  }
</style>
