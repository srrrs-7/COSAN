import RegisterForm from "../islands/register/RegisterForm.tsx";
import Agreement from "../components/Agreement.tsx";
import { useState } from "preact/hooks";

export default function Register() {
  return (
    <div style={{ display: "flex", flexDirection: "column", alignItems: "center", justifyContent: "center" }}>
      <h1 style={{ fontSize: "24px", backgroundColor: "#4a5568", width: "100%", justifyContent:"center", textAlign: "center", color: "white" }}>COSAN</h1>
      <p style={{ fontSize: "20px", fontWeight: "bold" }}>登録して古参になりませんか？</p>
      <Agreement />
      <RegisterForm />
      <div style={{ marginTop: "12px" }} >
        <a href="/www">ログイン画面へ戻る</a>
      </div>
    </div>
  );
}