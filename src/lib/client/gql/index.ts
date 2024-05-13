export * from "./fragment-masking";
export * from "./gql";

import { Client, cacheExchange, fetchExchange } from "@urql/svelte";

const backendUrl = process.env.BACKEND_URL;

if (backendUrl == undefined) {
  throw new Error("The backend URL is undefined");
}

export const frontOfficeClient = new Client({
  url: `${backendUrl}/front-office`,
  exchanges: [cacheExchange, fetchExchange],
});
