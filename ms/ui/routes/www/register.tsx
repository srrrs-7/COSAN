import Agreement from "../../components/Agreement.tsx";
import RegisterForm from "../../islands/register/RegisterForm.tsx";
import Header from "../../islands/common/Header.tsx";

export default function register() {
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
      
      <Header />

      <p style={{ fontSize: "12px" }}>登録して古参になりませんか？</p>

      <Agreement />

      <RegisterForm />

      <div style={{ marginTop: "12px" }} >
        <a href="/www">ログイン画面へ戻る</a>
      </div>
    </div>
  );
}