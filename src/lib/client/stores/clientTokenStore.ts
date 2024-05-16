import { writable } from "svelte/store";

export const clientTokenStore = writable<string | undefined>(undefined);
