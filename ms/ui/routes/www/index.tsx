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
        padding: "10px", // Add padding for better spacing
        textAlign: "center", // Center align text
        fontFamily: "hiragino kaku gothic pro, sans-serif", // Set a default font family
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