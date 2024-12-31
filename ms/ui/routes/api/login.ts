import { FreshContext } from "$fresh/server.ts";

export const handler = async (_req: Request, _ctx: FreshContext): Response => {
  const authUrl = Deno.env.get("AUTH_URL");

  console.log(_req);
  console.log(_ctx);

  const response = await fetch(`${authUrl}/login`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
  const data = await response.json();
  return new Response(data);
};
