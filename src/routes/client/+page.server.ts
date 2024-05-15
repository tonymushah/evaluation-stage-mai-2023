import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { route } from "$lib/ROUTES";

export const load: PageServerLoad = async function ({ cookies }) {
  const clientToken = cookies.get("clientToken");
  if (clientToken) {
    return {};
  } else {
    redirect(303, route("/client/login"));
  }
};
