<script>
  import { route } from "$lib/ROUTES";
</script>

<div class="top-layout">
  <div>Logo</div>
  <button formaction={route("logout /client")}>Logout</button>
</div>

<style lang="scss">
  .top-layout {
    color: var(--primary);
    background-color: var(--primary-l5);
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    padding: 10px 0px;
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
  }
</style>
