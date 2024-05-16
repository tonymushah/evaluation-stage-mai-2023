import { getContext, setContext } from "svelte";
import { writable, type Writable } from "svelte/store";

const KEY = "new-type-finition-context";

export function initFintionContext(): Writable<string | undefined> {
  return setContext(KEY, writable(undefined));
}

export function getFintionContext(): Writable<string | undefined> {
  const context = getContext<Writable<string | undefined>>(KEY);
  if (context) {
    return context;
  } else {
    throw new Error(`${KEY} context is undefined`);
  }
}
