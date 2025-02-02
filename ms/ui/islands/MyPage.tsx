import { JSX } from "preact";
import { useState } from "preact/hooks";
import RegisterWord from "../islands/mypage/RegisterWord.tsx";
import MenuBar from "../islands/mypage/MenuBar.tsx";

export default function MyPage() {
  return (
    <div>
      <div style={{ display: "flex", justifyContent: "center", alignItems: "center", height: "100vh" }}>
        <RegisterWord />
      </div>
      <MenuBar />
    </div>
  );
}