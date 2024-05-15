import { graphql } from "$lib/client/gql";
import { fail, redirect } from "@sveltejs/kit";
import { frontOfficeClient } from "../../../server/frontOfficeClient";
import type { Actions } from "./$types";
import { CLIENT_COOKIE_PATH } from "$env/static/private";
import { route } from "$lib/ROUTES";

const loginMutation = graphql(`
  mutation clientLogin($telephone: String!) {
    login(telephone: $telephone)
  }
`);

export const actions = {
  login: async function ({ cookies, request }) {
    const data = await request.formData();
    const telephone = data.get("telephone");
    if (telephone == null) {
      return fail(400, {
        telephone,
        missing: true,
      });
    }
    if (typeof telephone != "string" || telephone.trim().length == 0) {
      return fail(400, {
        telephone,
        invalid: true,
      });
    }
    const result = await frontOfficeClient.mutation(loginMutation, {
      telephone,
    });
    if (result.error) {
      return fail(500, {
        error: result.error.message,
        code: result.error.graphQLErrors[0]?.extensions.code,
      });
    }
    if (result.data) {
      cookies.set("clientToken", result.data.login, {
        path: CLIENT_COOKIE_PATH,
      });
      redirect(300, route("/client"));
      return { success: true };
    }
    return { success: true };
  },
} satisfies Actions;
