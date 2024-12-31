import { FreshContext } from "$fresh/server.ts";

export const handler = async (_req: Request, _ctx: FreshContext): Response => {
  const authUrl = Deno.env.get("AUTH_URL");
  const response = await fetch(`${authUrl}/register`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
  console.log(response);
  const data = await response.json();
  return new Response(data);
};
