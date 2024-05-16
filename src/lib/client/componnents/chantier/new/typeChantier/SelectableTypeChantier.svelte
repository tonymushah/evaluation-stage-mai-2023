<script lang="ts">
  import { getTypeChantierContext } from "../context";

  export let id: string;
  export let nom: string;
  export let description: string;
  export let prix: number;
  export let durree: number;
  const selected = getTypeChantierContext();
  $: isSelected = id == $selected;
</script>

<div class="type-chantier">
  <h3>{nom}</h3>
  <h4>{description}</h4>
  <div class="data">
    <h4>Prix: {prix} Ariary</h4>
    <h4>Durree: {durree}</h4>
  </div>
  <button
    on:click={() => {
      if (!isSelected) {
        selected.set(id);
      }
    }}
  >
    {#if isSelected}
      Selectionne
    {:else}
      Selectionner
    {/if}
  </button>
</div>

<style lang="scss">
  .type-chantier {
    border: 1px;
    border-radius: 0.25em;
    background-color: var(--primary-l1);
    h3 {
      margin: 2px;
    }
    h4 {
      margin: 1px;
    }
  }
  button {
    font-family: inherit;
    font-weight: 900;
    color: var(--primary-l5);
    border: none;
    background-color: var(--primary);
    border-radius: 5px;
    font-size: 18px;
    transition: background-color 300ms ease-in-out;
    padding: 1px 10px;
  }
  button:hover {
    background-color: var(--primary-l1);
  }
</style>
