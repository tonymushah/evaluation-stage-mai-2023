import { BACKEND_URL } from "$env/static/private";
import { Client, cacheExchange, fetchExchange } from "@urql/svelte";

export const adminClient = new Client({
  url: `${BACKEND_URL}/admin`,
  exchanges: [cacheExchange, fetchExchange],
});
