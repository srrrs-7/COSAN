import { FreshContext } from "$fresh/server.ts";

export const handler = async (_req: Request, _ctx: FreshContext): Response => {
  const supportUrl = Deno.env.get("SUPPORT_URL");

  console.log(_req);
  console.log(_ctx);
  
  const response = await fetch(`${supportUrl}/health`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
  const data = await response.json();
  return new Response(data);
};
