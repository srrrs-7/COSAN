// DO NOT EDIT. This file is generated by Fresh.
// This file SHOULD be checked into source version control.
// This file is automatically updated during development when running `dev.ts`.

import * as $_404 from "./routes/_404.tsx";
import * as $_app from "./routes/_app.tsx";
import * as $api_login from "./routes/api/login.ts";
import * as $api_register from "./routes/api/register.ts";
import * as $index from "./routes/index.tsx";
import * as $mb_index from "./routes/mb/index.tsx";
import * as $mb_register from "./routes/mb/register.tsx";
import * as $mb_top from "./routes/mb/top.tsx";
import * as $www_index from "./routes/www/index.tsx";
import * as $www_register from "./routes/www/register.tsx";
import * as $www_top from "./routes/www/top.tsx";
import * as $Agreement from "./islands/Agreement.tsx";
import * as $Header from "./islands/Header.tsx";
import * as $LoginForm from "./islands/LoginForm.tsx";
import * as $MyPage from "./islands/MyPage.tsx";
import * as $RegisterForm from "./islands/RegisterForm.tsx";
import type { Manifest } from "$fresh/server.ts";

const manifest = {
  routes: {
    "./routes/_404.tsx": $_404,
    "./routes/_app.tsx": $_app,
    "./routes/api/login.ts": $api_login,
    "./routes/api/register.ts": $api_register,
    "./routes/index.tsx": $index,
    "./routes/mb/index.tsx": $mb_index,
    "./routes/mb/register.tsx": $mb_register,
    "./routes/mb/top.tsx": $mb_top,
    "./routes/www/index.tsx": $www_index,
    "./routes/www/register.tsx": $www_register,
    "./routes/www/top.tsx": $www_top,
  },
  islands: {
    "./islands/Agreement.tsx": $Agreement,
    "./islands/Header.tsx": $Header,
    "./islands/LoginForm.tsx": $LoginForm,
    "./islands/MyPage.tsx": $MyPage,
    "./islands/RegisterForm.tsx": $RegisterForm,
  },
  baseUrl: import.meta.url,
} satisfies Manifest;

export default manifest;
