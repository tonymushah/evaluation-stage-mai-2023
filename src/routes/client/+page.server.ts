import { redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";
import { route } from "$lib/ROUTES";
import { CLIENT_COOKIE_PATH } from "$env/static/private";

export const load: PageServerLoad = async function ({ cookies }) {
  const clientToken = cookies.get("clientToken");
  if (clientToken) {
    return {
      clientToken,
    };
  } else {
    redirect(303, route("/client/login"));
  }
};

export const actions = {
  logout: async function ({ cookies }) {
    cookies.delete("clientToken", {
      path: CLIENT_COOKIE_PATH,
    });
    return { sucess: true };
  },
} satisfies Actions;
