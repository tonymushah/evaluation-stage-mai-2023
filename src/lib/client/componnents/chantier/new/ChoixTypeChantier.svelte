<script lang="ts">
  import { graphql } from "$lib/client/gql";
  import { CombinedError, getContextClient } from "@urql/svelte";
  import { onMount, type ComponentProps } from "svelte";
  import { derived, writable } from "svelte/store";
  import SelectableTypeChantier from "./typeChantier/SelectableTypeChantier.svelte";

  const client = getContextClient();
  const typeChantierStore = writable(
    new Map<string, ComponentProps<SelectableTypeChantier>>()
  );
  const typeChantiers = derived(typeChantierStore, ($d) =>
    Array.from($d, ([id, props]) => ({ id, props }))
  );
  const query = graphql(`
    query clientNewTypeChantier($page: OffsetLimit) {
      typeChantier {
        list(page: $page) {
          data {
            duree
            description
            idTypeChantier
            nom
            prixTotal
          }
          total
          offset
          limit
        }
      }
    }
  `);
  let error: CombinedError | undefined = undefined;
  let offset = 0;
  let limit = 10;
  let total: number | undefined = undefined;
  $: isAtEnd = total != undefined && offset >= limit;
  async function execute(limit: number, offset: number) {
    if (!isAtEnd) {
      const res = await client.query(query, {
        page: {
          limit,
          offset,
        },
      });
      if (res.error) {
        error = res.error;
      }
      if (res.data) {
        typeChantierStore.update((m) => {
          res.data?.typeChantier.list.data.forEach((f) => {
            m.set(f.idTypeChantier, {
              description: f.description,
              durree: f.duree,
              nom: f.nom,
              id: f.idTypeChantier,
              prix: f.prixTotal,
            });
          });
          return m;
        });
        return [
          res.data.typeChantier.list.offset + limit,
          res.data.typeChantier.list.total,
        ];
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
  onMount(async function () {
    const res = await execute(limit, offset);
    if (res) {
      offset = res[0];
      total = res[1];
    }
  });
</script>

<div class="type-chantier">
  <h3>Type de chantier</h3>
  <div class="choix">
    {#each $typeChantiers as { id, props } (id)}
      <SelectableTypeChantier {...props}></SelectableTypeChantier>
    {/each}
    {#if !isAtEnd}
      <div class="observer" bind:this={observer} />
    {/if}
  </div>
</div>

<style lang="scss">
  .choix {
    display: flex;
    flex-wrap: nowrap;
    flex-direction: row;
    overflow-x: scroll;
    gap: 10px;
  }
</style>
