<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let chantier: string;
  export let prixTotal: number;
  export let etat: string;
  export let ETA: number;
  const dispatch = createEventDispatcher<{
    click: MouseEvent & {
      currentTarget: EventTarget & HTMLButtonElement;
    };
  }>();
</script>

<button
  class="simple-chantier-componnent"
  on:click={(e) => {
    dispatch("click", e);
  }}
>
  <div class="type-chantier">
    {chantier}
  </div>
  <div class="prix"><span class="grad">Prix:</span> {prixTotal} Ariary</div>
  <div class="etat"><span class="grad">Etat:</span> {etat}</div>
  <div class="ETA"><span class="grad">ETA:</span> {ETA} %</div>
</button>

<style lang="scss">
  .simple-chantier-componnent {
    display: flex;
    flex-direction: column;
    align-items: first baseline;
    justify-content: center;
    gap: 5px;
    padding: 10px;
    border-radius: 0.25em;
    border: none;
    color: var(--primary);
    background-color: var(--primary-l5);
    transition: background-color 300ms ease-in-out;
    font-size: 20px;
  }
  .simple-chantier-componnent:hover {
    background-color: var(--primary-l4);
  }
  .grad {
    font-weight: 900;
  }
</style>
