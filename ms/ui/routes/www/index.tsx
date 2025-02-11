import LoginForm from "../../islands/LoginForm.tsx";
import MenuBar from "../../islands/common/MenuBar.tsx";

export default function Login() {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        height: "100vh",
        padding: "10px", // Add padding for better spacing
        textAlign: "center", // Center align text
        fontFamily: "hiragino kaku gothic pro, sans-serif", // Set a default font family
      }}
    >
      <p style={{ fontSize: "48px", marginBottom: "16px" }}>COSAN</p>
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