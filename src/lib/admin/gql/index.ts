export * from "./fragment-masking";
export * from "./gql";

import { Client, cacheExchange, fetchExchange } from "@urql/svelte";

const backendUrl = process.env.BACKEND_URL;

if (backendUrl == undefined) {
  throw new Error("The backend URL is undefined");
}

export const adminClient = new Client({
  url: `${backendUrl}/admin`,
  exchanges: [cacheExchange, fetchExchange],
});
