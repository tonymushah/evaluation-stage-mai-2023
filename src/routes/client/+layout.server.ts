import { BACKEND_URL } from "$env/static/private";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async function () {
  return {
    backend: BACKEND_URL,
  };
};
