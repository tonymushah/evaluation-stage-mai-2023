import cookie from "js-cookie";

export default function getTokenClient(): string {
  const token = cookie.get("clientToken");
  if (token) {
    return token;
  } else {
    throw new Error("clientToken cookie is undefined");
  }
}
