// DO NOT EDIT. This file is generated by Fresh.
// This file SHOULD be checked into source version control.
// This file is automatically updated during development when running `dev.ts`.

import * as $_404 from "./routes/_404.tsx";
import * as $_app from "./routes/_app.tsx";
import * as $api_login from "./routes/api/login.ts";
import * as $api_register from "./routes/api/register.ts";
import * as $index from "./routes/index.tsx";
import * as $www_index from "./routes/www/index.tsx";
import * as $www_mypage_index from "./routes/www/mypage/index.tsx";
import * as $www_mypage_search from "./routes/www/mypage/search.tsx";
import * as $www_register from "./routes/www/register.tsx";
import * as $LoginForm from "./islands/LoginForm.tsx";
import * as $MyPage from "./islands/MyPage.tsx";
import * as $Register from "./islands/Register.tsx";
import * as $common_Header from "./islands/common/Header.tsx";
import * as $common_MenuBar from "./islands/common/MenuBar.tsx";
import * as $common_RankRecord_ from "./islands/common/RankRecord .tsx";
import * as $mypage_Ranking from "./islands/mypage/Ranking.tsx";
import * as $mypage_RegisterWord from "./islands/mypage/RegisterWord.tsx";
import * as $register_RegisterForm from "./islands/register/RegisterForm.tsx";
import type { Manifest } from "$fresh/server.ts";

const manifest = {
  routes: {
    "./routes/_404.tsx": $_404,
    "./routes/_app.tsx": $_app,
    "./routes/api/login.ts": $api_login,
    "./routes/api/register.ts": $api_register,
    "./routes/index.tsx": $index,
    "./routes/www/index.tsx": $www_index,
    "./routes/www/mypage/index.tsx": $www_mypage_index,
    "./routes/www/mypage/search.tsx": $www_mypage_search,
    "./routes/www/register.tsx": $www_register,
  },
  islands: {
    "./islands/LoginForm.tsx": $LoginForm,
    "./islands/MyPage.tsx": $MyPage,
    "./islands/Register.tsx": $Register,
    "./islands/common/Header.tsx": $common_Header,
    "./islands/common/MenuBar.tsx": $common_MenuBar,
    "./islands/common/RankRecord .tsx": $common_RankRecord_,
    "./islands/mypage/Ranking.tsx": $mypage_Ranking,
    "./islands/mypage/RegisterWord.tsx": $mypage_RegisterWord,
    "./islands/register/RegisterForm.tsx": $register_RegisterForm,
  },
  baseUrl: import.meta.url,
} satisfies Manifest;

export default manifest;
