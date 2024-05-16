<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let chantier: string;
  export let prixTotal: number;
  export let etat: string;
  export let ETA: number;
  const dispatch = createEventDispatcher<{
    click: MouseEvent & {
      currenTarget: EventTarget & HTMLButtonElement;
    };
  }>();
</script>

<button class="simple-chantier-componnent">
  <div class="type-chantier">
    {chantier}
  </div>
  <div class="prix">{prixTotal} Ariary</div>
  <div class="etat">{etat}</div>
  <div class="ETA">{ETA} %</div>
</button>

<style lang="scss">
  .simple-chantier-componnent {
    display: flex;
    flex-direction: column;
    gap: 5px;
    padding: 5px;
    border-radius: 0.25em;
    border: none;
    color: var(--primary);
    background-color: var(--primary-l5);
    transition: background-color 300ms ease-in-out;
  }
  .simple-chantier-componnent:hover {
    background-color: var(--primary-l4);
  }
</style>
