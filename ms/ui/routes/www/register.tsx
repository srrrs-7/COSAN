import Agreement from "../../components/Agreement.tsx";
import RegisterForm from "../../islands/register/RegisterForm.tsx";
import Header from "../../islands/common/Header.tsx";
import MenuBar from "../../islands/common/MenuBar.tsx";

export default function register() {
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

      <p style={{ fontSize: "12px", margin: "10px" }}>登録して古参になりませんか？</p>

      <RegisterForm />

      <Agreement />

      <div  style={{ marginTop: "4px" }}>
        <a href="/www" style={{ color: "#007bff", textDecoration: "none" }}>ログイン画面へ戻る</a>
      </div>

      <MenuBar />
    </div>
  );
}