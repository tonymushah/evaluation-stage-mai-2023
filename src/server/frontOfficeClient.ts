import { BACKEND_URL } from "$env/static/private";
import { Client, cacheExchange, fetchExchange } from "@urql/svelte";

export const frontOfficeClient = new Client({
  url: `${BACKEND_URL}/front-office`,
  exchanges: [cacheExchange, fetchExchange],
});
