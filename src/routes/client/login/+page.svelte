<script lang="ts">
  import type { ActionData } from "../$types";

  export let form: ActionData;
  $: console.log(form);
</script>

<div class="layout">
  <div class="login-form">
    <h3>Connection</h3>
    <form method="post" action="?/login">
      {#if form}
        {#if form.missing}
          <div class="error">Telephone d'entree est vide</div>
        {/if}
        {#if form.incorrect}
          <div class="error">Telephone d'entree est invalid</div>
        {/if}
        {#if form.error}
          <div class="error">{form.error}</div>
        {/if}
      {/if}
      <label>
        Telephone
        <input name="telephone" value={form?.email ?? ""} />
      </label>
      <button>Se connecter</button>
    </form>
  </div>
</div>

<style lang="scss">
  .layout {
    width: 98cqw;
    height: 98cqh;
    display: flex;
    justify-content: center;
    align-items: center;
    .login-form {
      background-color: var(--primary);
      padding: 25px 100px;
      border-radius: 0.25em;
      .error {
        color: var(--secondary-l2);
      }
      h3 {
        margin: 10px 0px;
        font-weight: 900;
        font-size: 30px;
      }
      form {
        padding-bottom: 10px;
        display: flex;
        align-items: start;
        flex-direction: column;
        gap: 5px;
        label {
          display: flex;
          align-items: start;
          flex-direction: column;
          gap: 5px;
          input {
            color: inherit;
            font-family: inherit;
            font-weight: 600;
            font-size: 20px;
          }
        }
      }
    }
  }
  button {
    font-family: inherit;
    font-weight: 900;
    color: var(--primary);
    border: none;
    background-color: var(--primary-l5);
    border-radius: 5px;
    font-size: 18px;
    transition: background-color 300ms ease-in-out;
  }
  button:hover {
    background-color: var(--primary-l4);
  }
</style>
