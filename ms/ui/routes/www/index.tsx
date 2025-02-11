import Header from "../../islands/common/Header.tsx";
import LoginForm from "../../islands/register/LoginForm.tsx";
import MenuBar from "../../islands/common/MenuBar.tsx";

export default function index() {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        padding: "10px",
        textAlign: "center",
        fontFamily: "hiragino kaku gothic pro, sans-serif",
      }}
    >
      <Header />

      <LoginForm />

      <div style={{ marginTop: "24px" }}>
        <a href="/www/register" style={{ color: "#007bff", textDecoration: "none" }}>
          無料新規登録はこちら
        </a>
      </div>

      <MenuBar />
    </div>
  );
}