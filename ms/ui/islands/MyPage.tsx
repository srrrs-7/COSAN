import { JSX } from "preact";
import { useState } from "preact/hooks";
import RegisterWord from "../islands/mypage/RegisterWord.tsx";
import MenuBar from "../islands/common/MenuBar.tsx";

export default function MyPage() {
  return (
    <div>
      <div style={{ display: "flex", flexDirection: "column", alignItems: "center", justifyContent: "center", marginBottom: "16px" }}>
        <p style={{ fontSize: "20px" }}>
          古参を証明する唯一の証
        </p>
        <p style={{ marginBottom: "8px" }}>
          あなたの推しを登録しましょう。
        </p>
        <p style={{ marginBottom: "8px" }}>
          あなたの推しを登録することで、古参であることを証明できます。
        </p>
      </div>
      <div style={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
        <RegisterWord />
      </div>
      <MenuBar />
    </div>
  );
}