<script lang="ts">
  import ChoixTypeChantier from "./ChoixTypeChantier.svelte";
  import { initFintionContext, initTypeChantierContext } from "./context";

  const finition = initFintionContext();
  const typeChantier = initTypeChantierContext();
</script>

<h2>Creation Chantier</h2>

<div class="type">
  <ChoixTypeChantier />
</div>

<div class="finition"></div>
